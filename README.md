Thread Time Measurement Library
============================

[Documentation](https://docs.rs/thread-time) |
[Github](https://github.com/hamptonmoore/thread-time) |
[Crate](https://crates.io/crates/thread-time)


A simple and idiomatic interface for measurement thread CPU time:

```rust

let start = ThreadTime::new();
# .. do something ..
let cpu_time: Duration = start.elapsed();
println!(" {:?}");

```

It's purpose is to be a `Send` safe version of the [cpu-time](https://github.com/tailhook/cpu-time) package, but focusing solely on measuring thread time and linux support.
