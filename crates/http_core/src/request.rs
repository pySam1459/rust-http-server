use core::fmt;
use std::collections::HashMap;

/// HTTP method
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Update,
    Connect,
    Options,
    Trace,
    Patch,
    /// Unknown or extension methods are preserved as their raw string.
    Unknown(String),
}

impl Method {
    /// Create a Method from a token (case-sensitive per RFC; we match on common uppercase forms)
    pub fn from_token(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "HEAD" => Method::Head,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "UPDATE" => Method::Update,
            "CONNECT" => Method::Connect,
            "OPTIONS" => Method::Options,
            "TRACE" => Method::Trace,
            "PATCH" => Method::Patch,
            other => Method::Unknown(other.to_string()),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Method::Get => write!(f, "GET"),
            Method::Head => write!(f, "HEAD"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
            Method::Update => write!(f, "UPDATE"),
            Method::Connect => write!(f, "CONNECT"),
            Method::Options => write!(f, "OPTIONS"),
            Method::Trace => write!(f, "TRACE"),
            Method::Patch => write!(f, "PATCH"),
            Method::Unknown(s) => write!(f, "{}", s),
        }
    }
}

pub struct StartLine {
    pub method: Method,
    pub path: String,
    pub version: String,
}

/// Basic HTTP request data structure
pub struct Request {
    pub start_line: StartLine,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let body_display = self
            .body
            .as_ref()
            .and_then(|b| String::from_utf8(b.clone()).ok())
            .unwrap_or_else(|| "-".to_string());

        write!(
            f,
            "Request[{} {} {}, headers={}, body={}]",
            self.start_line.method,
            self.start_line.path,
            self.start_line.version,
            self.headers.len(),
            body_display
        )
    }
}
