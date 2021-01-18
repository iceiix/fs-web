# fs-web

Experiments in alternative implementations of Rust's standard library [std::fs](https://doc.rust-lang.org/std/fs/) module, for use on the web

Was intended to be a drop-in replacement for std::fs on wasm32-unknown-unknown targets

* `src/fs.rs`: std::fs API, based on [library/std/src/fs.rs](https://github.com/rust-lang/rust/blob/master/library/std/src/fs.rs)
* `src/unix_fs.rs`: Implementation for Unix systems, based on [library/std/src/sys/unix/fs.rs](https://github.com/rust-lang/rust/blob/master/library/std/src/sys/unix/fs.rs). Not as portable as the real Unix backend, but since it supports fewer platforms it can be useful for reference purposes. Targets the subset of Unix defined in the standard library by `target_os = "emscripten"`. In theory, this module's use of [libc](https://lib.rs/crates/libc) could be replaced by a web-based `libc` implementation, such as from Emscripten's [library_fs.js](https://github.com/emscripten-core/emscripten/blob/master/src/library_fs.js).
* `src/nop_fs.rs`: Minimal implementation written from scratch that does nothing, and always tries to succeed. This is unlike [library/std/src/sys/wasm](https://github.com/rust-lang/rust/tree/master/library/std/src/sys/wasm), which will panic with an unsupported message when calling std::fs, so it can be useful when porting native apps to the web, bypassing blocking filesystem requirements.
* `src/mem_fs.rs`: Incomplete/broken memory-backed filesystem implementation. Inspired by Emscripten's [MEMFS](https://emscripten.org/docs/api_reference/Filesystem-API.html#filesystem-api-memfs), but written in Rust instead of JavaScript. Implements some data structures but stuck on lifetime requirements. There are more complete memory filesystem crates out there.
* `src/static_fs.rs`: Another simple filesystem implementation, where file data is statically stored in the program at compile-time, resembling Emscripten's [preloaded files](https://emscripten.org/docs/api_reference/Filesystem-API.html#FS.createPreloadedFile). Presents a static read-only filesystem, intended to load resources.

To use:

```sh
cargo test
cargo run --bin hello_file
```

## License

Dual-licensed MIT and ApacheV2

Includes code from [Rust's standard library](https://github.com/rust-lang/rust/tree/master/library), thanks to all the contributors
