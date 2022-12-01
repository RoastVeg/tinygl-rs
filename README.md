# tgl - Rust bindings to TinyGL

[![Crates.io](https://img.shields.io/crates/v/tgl.svg)](https://crates.io/crates/tgl)
[![Documentation](https://docs.rs/tgl/badge.svg)](https://docs.rs/tgl)

[TinyGL](https://github.com/c-chads/tinygl) is a very lightweight partial OpenGL implementation. Its small size makes it ideal for static linking.

These bindings aim to broadly replicate the API of the [gl](https://github.com/brendanzab/gl-rs) crate, though there are notable differences.

## Features

- Impressive performance with low resource requirements
- Highly portable OpenGL implementation, with no runtime dependencies
- `#[no_std]` support out of the box

## Call for Examples

Want to contribute a neat example of something you've made with tgl? Got a project that uses tgl as a dependency you'd like to see featured here? Open a PR!

## Recommended Usage

Use `tgl::Init` to set up a "zbuffer", which TinyGL provides and we expose via `tgl::zbuffer::open`. A zbuffer can copy to framebuffers such as [rust_minifb](https://github.com/emoon/rust_minifb) with ease. Check out the examples directory to see this in action.

With a framebuffer up and running, you can start making GL calls. Remember that TinyGL is only a partial implementation, and not all the methods you might expect are present.
