# `emit_rand`

[![rand](https://github.com/emit-rs/emit_rand/actions/workflows/rand.yml/badge.svg)](https://github.com/emit-rs/emit_rand/actions/workflows/rand.yml)

[Current docs](https://docs.rs/emit_rand/0.9.1/emit_rand/index.html)

An implementation of `emit::Rng` using a specific version of the `rand` library.

`emit` itself has a `rand` feature and provides exactly the same `RandRng`, but only on select platforms. You can use `emit_rand` to always configure `emit` using `rand` if you're also using it elsewhere in your application, and want to guarantee only a specific version will end up in your dependency tree.

## Getting started

Add `emit` and `emit_rand` to your Cargo.toml:

```toml
[dependencies.emit]
version = "1"
# Disable default features of `emit` to avoid pulling in a possibly different version of `rand`
default-features = false
features = ["std", "implicit_rt"]

[dependencies.emit_rand]
version = "0.9"
```

Configure `emit` to use the `RandRng` type from this library during setup:

```rust
fn main() {
    let rt = emit::setup()
        .with_rng(emit_rand::rng())
        // Other configuration goes here
        .init();

    // Your app code goes here

    rt.blocking_flush(std::time::Duration::from_secs(5));
}
```

## Versioning and compatibility

`emit_rand` version `x.y.z` is compatible with `rand` version `x.y.*`.
