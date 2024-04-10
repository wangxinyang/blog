# [wangxy.com](https://wangxy.fly.dev/)

This is a my personal blog at <>, written in [Rust](https://www.rust-lang.org)
using the [Leptos](https://leptos.dev) framework, and styled with [Tailwind CSS](https://tailwindcss.com).

## Image

![top](https://github.com/wangxinyang/blog/blob/main/captures/top.jpg)

## List of the features

- Rust
- Leptos
- Tailwind Css
- Tailwind/Typography

## How to run it in your local

If you don't have `cargo-leptos` installed you can install it with

```bash
cargo install cargo-leptos
```

Then cd into your project directory

```bash
cd blog
```

## Running your project

```bash
cargo leptos watch
```

## Css JIT

To use the Tailwind CSS JIT, you need to add the following to your `package.json` or execute the command below

```bash
bun run watch
```

If you have no bun environment, you can install it with

```bash
# Linux&MacOS
curl -fsSL https://bun.sh/install | bash

# Windows
powershell -c "irm bun.sh/install.ps1 | iex"

```

## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
3. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
4. `npm install -g sass` - install `dart-sass` (should be optional in future

## Compiling for Release

```bash
cargo leptos build --release
```

Will generate your server binary in target/server/release and your site package in target/site
