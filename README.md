# WASM
A Rust-native WebAssembly syntax model useful for generate, reading, and emitting WebAssembly code.

## Usage
To use `wasm`, first add this to your `Cargo.toml`:

```toml
[dependencies]
wasm = "0.0.1"
```

Then, add this to your crate:

```rust
use wasm::model::Module;

fn main() {
    // ...
}
```

## Examples

Create an empty WASM module:

```rust
use wasm::model::Module;

fn main() {
    let builder = Module.builder();
    let module = builder.build();
}
```

# License

Licensed under Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be 
licensed as above, without any additional terms or conditions.