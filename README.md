trying to repro some Rust compile time slowdowns involving async/futures/zbus

Instructions:

1. `cargo build`
2. `touch src/lib.rs` (or make a non-functional edit and save)
3. `cargo build -F N` where `N` is 1, 4, 8, 12, or 16 (to compile that many functions)

Notes(zachs18):

The following appear to be somewhat load-bearing:

* Rust >= 1.72.0 (1.71.1 does not reproduce the issue, rustc 1.74.0-nightly (59a829484 2023-08-30) *does* reproduce the issue)

* Specifically `zbus::Connection::system()` or `zbus::Connection::session()` (using an arbitrary future-returning function does not reproduce the same issue; `tokio::time::sleep`, `std::future::ready(())`, `std::future::pending::<zbus::Result<zbus::Connection>>()` don't reproduce the issue.)

* Having an `async fn $name` (inlining the body of `$name` into `let future = async move { ... };` does not reproduce the issue).

* The `let future = $name();` binding in `fn $wrapper_name` (if the `$name()` call is explicitly inlined into `Box::pin()`, the issue does not appear (even in a `{ $name() }` block-expression or `std::convert::identity`; but the issue *does* appear if in a `{let f = $name(); f}` block expression))

* `Box::pin(future)` (it can be ignored, but if it is commented out, the issue does not appear).

The time it takes appears to be linear-ish in the number of relevant functions (`cargo b -F 4` takes 4.67s, `cargo b -F 8` takes 9.49s, `cargo b -F 12` takes 13.84s, `cargo b -F 16` takes 18.09s)

The choice of `tokio` vs `async-io` feature for `zbus` does not appear to change anything (the issue appears with both; I left the tokio version in the cargo.toml because it gives fewer overall deps).

`incremental = false` does not affect the issue (The issue is still reproducible with incremental = true, but you have to actually change `lib.rs` not just `touch src/lib.rs`.)

Binary vs library does not appear to affect the issue (rename to `main.rs` and add a `fn main(){}` and the issue persists).
