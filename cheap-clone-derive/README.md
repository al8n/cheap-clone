# cheap-clone-derive

Derive macro for [`cheap-clone`](https://crates.io/crates/cheap-clone).
Enable it through the runtime crate:

```toml
[dependencies]
cheap-clone = { version = "1", features = ["derive"] }
```

```rust
#[derive(Clone, cheap_clone::CheapClone)]
struct Message {
    id: u64,
    body: std::sync::Arc<str>,
}
```

The macro calls `CheapClone` for each field and supports structs, enums,
generics, renamed dependencies, and `no_std` consumers. It requires Rust 1.85.

- `#[cheap_clone(inherent)]` also generates an inherent `cheap_clone()` method.
- `#[cheap_clone(with = "path::to::function")]` selects a field's clone function.
- `#[cheap_clone(crate = "path::to::cheap_clone")]` overrides the runtime path.

See the [runtime documentation](https://docs.rs/cheap-clone) for details.
