/*!
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
*/

#![cfg_attr(not(test), no_std)]

use emit_core::{rng::Rng, runtime::InternalRng};
use rand::{Rng as _, RngCore};

/**
Create a new source of randomness based on [`RandRng`].
*/
pub const fn rng() -> RandRng {
    RandRng::new()
}

/**
An [`Rng`] based on the [`rand`] library.
*/
#[derive(Default, Debug, Clone, Copy)]
pub struct RandRng {}

impl RandRng {
    /**
    Create a new source of randomness.
    */
    pub const fn new() -> Self {
        RandRng {}
    }
}

impl Rng for RandRng {
    fn fill<A: AsMut<[u8]>>(&self, mut arr: A) -> Option<A> {
        rand::rng().fill_bytes(arr.as_mut());

        Some(arr)
    }

    fn gen_u64(&self) -> Option<u64> {
        Some(rand::rng().random())
    }

    fn gen_u128(&self) -> Option<u128> {
        Some(rand::rng().random())
    }
}

impl InternalRng for RandRng {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen() {
        assert_ne!(RandRng::new().gen_u128(), RandRng::new().gen_u128());
        assert_ne!(RandRng::new().gen_u64(), RandRng::new().gen_u64());
        assert_ne!(RandRng::new().fill([0; 32]), RandRng::new().fill([0; 32]));
    }
}
