# Rust HTTP Server

A simple HTTP Server written in Rust with no pre-build libraries. Project mainly for learning

## Sync HTTP Server
A blocking TcpStream I/O with threadpool implementation.

### Benchmark Results

| Metric | Result |
|:--|:--|
| **Command** | `wrk -t4 -c100 -d10s http://<LAN IP>/` |
| **Requests** | 164,045 total (16,374 req/s) |
| **Transfer** | 9.39 MB read (0.94 MB/s) |
| **Latency** | avg 240 µs  •  stdev 58 µs  •  max 2.8 ms |
| **Req/Sec** | avg 16.49 k  •  stdev 634  •  max 17.16 k |

> 4 threads · 100 concurrent connections · 10 s test over LAN
