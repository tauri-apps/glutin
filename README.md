# glutin_tao

Glutin is a low-level library for OpenGL context creation, glutin_tao uses tao instead of winit.


[![](https://img.shields.io/crates/v/glutin.svg)](https://crates.io/crates/glutin)
[![Docs.rs](https://docs.rs/glutin/badge.svg)](https://docs.rs/glutin)

```toml
[dependencies]
glutin = "0.30.8"
```

## [Documentation](https://docs.rs/glutin_tao)

### Try it!

```bash
git clone https://github.com/tauri-apps/glutin
cd glutin
cargo run --example window
```

### Usage

Glutin is an OpenGL context creation library, and doesn't directly provide
OpenGL bindings for you.

For examples, please look [here](https://github.com/rust-windowing/glutin/tree/master/glutin_examples).

Note that glutin aims at being a low-level brick in your rendering
infrastructure. You are encouraged to write another layer of abstraction
between glutin and your application.

The minimum Rust version target by glutin is `1.65.0`.

## Platform-specific notes

### Wayland

Wayland is currently unsupported.