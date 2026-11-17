# cheap-clone 1.0.0 and cheap-clone-derive 1.0.0

The first stable release adds an optional derive macro and expands the set of
types that can explicitly promise cheap cloning.

- Release `cheap-clone` and `cheap-clone-derive` together at 1.0.0. The optional
  `derive` feature now depends on the 1.0 series of the macro crate.
- Enable `derive` for field-by-field `CheapClone` implementations. Inherent
  methods are opt-in, fields can select custom clone functions, and the crate
  path can be overridden for renamed dependencies and facade re-exports.
- Optional, versioned integrations cover shared strings and bytes, date/time
  values, UUIDs, decimals, HTTP values, and fixed-capacity containers.
- Remove the unreleased Chrono integration following its unmaintained advisory,
  [RUSTSEC-2026-0543](https://rustsec.org/advisories/RUSTSEC-2026-0543.html).
  Date/time integrations remain available through `jiff` and `time`.
- Add `cower` 0.1.1+ support through `cower_0_1` or `cower`: `Cow` requires cheap
  backing storage, and `CowRepr` requires a cheap owned representation. This
  integration supports Rust 1.85 in core-only, alloc, and std configurations.
- Add `emailaddr` 1.x support for cheap storage, stack buffers, parsing policies
  and results. Add `hostaddr` 1.1+ support for `Domain`, `Host`, `HostAddr` and
  `Buffer` through `hostaddr_1` or its `hostaddr` alias.
- Preserve the legacy `smol_bytes` feature names for the 0.1 series; the new
  `smol-bytes` alias selects 0.2.
- The base crate and derive macro support Rust 1.85. Optional integrations have
  their upstream MSRVs; `smol-bytes` and all-features checks require Rust 1.95.

## Migrating hostaddr users

Upgrade both dependencies, remove `hostaddr`'s old `cheap-clone` feature, and
enable the integration on `cheap-clone`:

```toml
cheap-clone = { version = "1", features = ["hostaddr"] }
hostaddr = "1.1"
```

The 0.3 and 1.0 `CheapClone` traits are distinct. Update public trait bounds and
dependent crates together where they exchange types constrained by this trait.
`hostaddr` 1.1 intentionally removes the old 0.3 implementations and feature.

The OIDC release workflow publishes `cheap-clone-derive` 1.0.0 before
`cheap-clone` 1.0.0. `hostaddr` 1.1 and `cower` 0.1.1 are already available.
See the README for supported types and cloning-cost limits, and
[the release guide](.github/RELEASING.md) for publishing instructions.
