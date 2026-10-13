# History

This repository was extracted from the `cheap-clone/` subtree of
https://github.com/al8n/layer0 at commit
`978af8d3908ceb11d2ab32568aeabe0d078a0fd1`.

The extraction used `git subtree split --prefix=cheap-clone`. Relevant
author, date, commit-message, and ancestry history was preserved as rewritten
subtree commits; original commit IDs remain in `al8n/layer0`.

## 0.3.2 MSRV notes

The base crate, default configuration, `no-default-features`, and `alloc`
configuration require Rust `1.85`, because Cargo must parse edition-2024
manifests from optional dependencies even when those features are inactive.
The non-smol optional feature powerset is verified with Rust `1.89`.
`smol_bytes_0_1` and `smol_bytes` require Rust `1.95`; the full `smol_str_0_3`
`0.3` and `faststr_0_2` `0.2` dependency ranges are restored.
