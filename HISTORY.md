# History

This repository was extracted from the `cheap-clone/` subtree of
https://github.com/al8n/layer0 at commit
`978af8d3908ceb11d2ab32568aeabe0d078a0fd1`.

The extraction used `git subtree split --prefix=cheap-clone`. Relevant
author, date, commit-message, and ancestry history was preserved as rewritten
subtree commits; original commit IDs remain in `al8n/layer0`.

## 0.3.2 MSRV notes

The base crate, default configuration, `no-default-features`, and `alloc`
configuration retain Rust `1.77`. Optional integrations follow their upstream
effective MSRV: `smol_bytes_0_1` and `smol_bytes` require Rust `1.95`, while
`smol_str_0_3` is bounded to `smol_str >=0.3.2, <0.3.4` for Cargo `1.77`
manifest compatibility and downstream `SmolStr` type unification.
`faststr_0_2` is bounded to `faststr >=0.2.23, <0.2.24` to avoid the later
optional `rkyv` chain while preserving Cargo `1.77` compatibility and
downstream `FastStr` type unification.
