use bytes::{BufMut, BytesMut};
use memchr::memmem;
use std::{
    collections::HashMap,
    io::{self, Read, Result},
    net::TcpStream,
};

use http_core::{Method, Request, StartLine};

const MAX_BUFFER_SIZE: usize = 8124;
const CONTENT_LENGTH: &str = "content-length";

/// Courtesy of chatgpt
fn read_into_bytesmut<R: Read>(r: &mut R, buf: &mut BytesMut) -> Result<usize> {
    buf.reserve(512);

    let mut_uninit = buf.chunk_mut();

    // SAFETY: We're only writing into uninitialized capacity; we will set length manually.
    let dst = unsafe { mut_uninit.as_mut_ptr().as_mut().unwrap() };
    let dst_len = mut_uninit.len();

    let n = r.read(unsafe { std::slice::from_raw_parts_mut(dst, dst_len) })?;

    unsafe {
        buf.advance_mut(n);
    }
    Ok(n)
}

/// parses a raw http header string `key: value` into a (String, String)
fn parse_line_to_kv(line: String) -> Option<(String, String)> {
    line.split_once(':')
        .map(|(k, v)| (k.trim().to_ascii_lowercase(), v.trim().to_string()))
}

struct LineReader<'a> {
    stream: &'a mut TcpStream,
    buf: BytesMut,
}

impl<'a> LineReader<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Self {
        Self {
            stream,
            buf: BytesMut::with_capacity(MAX_BUFFER_SIZE),
        }
    }

    pub fn read_n_bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        // Read until we have at least n bytes in the buffer
        while self.buf.len() < n {
            let bytes_read = read_into_bytesmut(&mut self.stream, &mut self.buf)?;
            if bytes_read == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Stream ended before reading requested bytes",
                ));
            }
        }

        // Extract exactly n bytes from the buffer
        Ok(self.buf.split_to(n).to_vec())
    }
}

impl<'a> Iterator for LineReader<'a> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(pos) = memmem::find(&self.buf, b"\r\n") {
                let line_bytes = self.buf.split_to(pos + 2);
                let line = &line_bytes[..pos];
                let line_str = unsafe { String::from_utf8_unchecked(line.to_vec()) };
                return Some(Ok(line_str));
            }

            match read_into_bytesmut(&mut self.stream, &mut self.buf) {
                Ok(0) => {
                    if self.buf.is_empty() {
                        return None;
                    } else {
                        let leftover = self.buf.split();
                        let line_str = unsafe { String::from_utf8_unchecked(leftover.to_vec()) };
                        return Some(Ok(line_str));
                    }
                }
                Ok(_) => {}
                Err(e) => return Some(Err(e)),
            };

            if self.buf.len() > MAX_BUFFER_SIZE {
                return Some(Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Line to long",
                )));
            }
        }
    }
}

fn parse_start_line(start_line_raw: Option<Result<String>>) -> io::Result<StartLine> {
    let start_line_str = start_line_raw.and_then(|r| r.ok()).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "HTTP Request start line is ill-formed",
        )
    })?;

    let mut parts = start_line_str.split_ascii_whitespace();
    let mut next_token = |name| {
        parts.next().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("start line missing {}", name),
            )
        })
    };

    Ok(StartLine {
        method: Method::from_token(next_token("method")?),
        path: next_token("path")?.to_string(),
        version: next_token("version")?.to_string(),
    })
}

pub fn parse_request(stream: &mut TcpStream) -> Result<http_core::Request> {
    let mut reader = LineReader::new(stream);

    let start_line = parse_start_line(reader.next())?;

    let headers: HashMap<String, String> = (&mut reader)
        .take_while(|line| line.as_ref().map(|s| !s.is_empty()).unwrap_or(false))
        .filter_map(|line| line.ok())
        .filter_map(parse_line_to_kv)
        .collect();

    let content_length = headers.get(CONTENT_LENGTH);

    let body = match content_length {
        Some(cl_str) => Some(reader.read_n_bytes(cl_str.parse().unwrap())?),
        None => None,
    };

    Ok(Request {
        start_line,
        headers,
        body,
    })
}
