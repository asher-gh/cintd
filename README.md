# Architecture

- Using `tokio` provides a multi-threaded asynchronous runtime.
- Diesel had some compatibility issues with Prisma generated queries. However,
  there is an interesting project
  [Prisma Client Rust](https://github.com/Brendonovich/prisma-client-rust) that
  provides Prisma support in Rust.
- `axum` is a web framework that uses `tokio` and `hyper` to provide an easy to
  setup multi-threaded web server.
- `tokio-chron-scheduler` is a crate that allows to schedule jobs
  asynchronously.

# Benchmarks

Benchmarks with `wrk` with 12 threads and 400 active connections

## Node server

```
Memory consumption: 91MB
Running 30s test @ http://127.0.0.1:3000
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.45ms  467.04us  27.91ms   97.79%
    Req/Sec     6.09k   277.03     9.22k    96.83%
  2180468 requests in 30.01s, 372.22MB read
  Socket errors: connect 0, read 570, write 0, timeout 0
Requests/sec:  72647.37
Transfer/sec:     12.40MB
```

## Hyper (Rust) server with 8 threads

```
Memory consumption: 24MB
Running 30s test @ http://127.0.0.1:3000
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.92ms    3.71ms 137.95ms   95.14%
    Req/Sec    12.35k     2.16k   42.10k    76.79%
  4425649 requests in 30.10s, 552.90MB read
  Socket errors: connect 0, read 378, write 0, timeout 0
Requests/sec: 147030.08
Transfer/sec:     18.37MB
```
