<div align="center">
<h1>cheap-clone</h1>
</div>
<div align="center">

A trait that indicates that such a type can be cloned cheaply.

[<img alt="github" src="https://img.shields.io/badge/github-al8n/cheap--clone-8da0cb?style=for-the-badge&logo=Github" height="22">][Github-url]
<img alt="LoC" src="https://img.shields.io/endpoint?url=https%3A%2F%2Fgist.githubusercontent.com%2Fal8n%2F327b2a8aef9003246e45c6e47fe63937%2Fraw%2Fcheap-clone" height="22">
[<img alt="Build" src="https://img.shields.io/github/actions/workflow/status/al8n/cheap-clone/ci.yml?logo=Github-Actions&style=for-the-badge" height="22">][CI-url]
[<img alt="codecov" src="https://img.shields.io/codecov/c/gh/al8n/cheap-clone?style=for-the-badge&logo=codecov" height="22">][codecov-url]

[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-cheap--clone-66c2a5?style=for-the-badge&labelColor=555555&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">][doc-url]
[<img alt="crates.io" src="https://img.shields.io/crates/v/cheap-clone?style=for-the-badge&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iaXNvLTg4NTktMSI/Pg0KPCEtLSBHZW5lcmF0b3I6IEFkb2JlIElsbHVzdHJhdG9yIDE5LjAuMCwgU1ZHIEV4cG9ydCBQbHVnLUluIC4gU1ZHIFZlcnNpb246IDYuMDAgQnVpbGQgMCkgIC0tPg0KPHN2ZyB2ZXJzaW9uPSIxLjEiIGlkPSJMYXllcl8xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIiB4PSIwcHgiIHk9IjBweCINCgkgdmlld0JveD0iMCAwIDUxMiA1MTIiIHhtbDpzcGFjZT0icHJlc2VydmUiPg0KPGc+DQoJPGc+DQoJCTxwYXRoIGQ9Ik0yNTYsMEwzMS41MjgsMTEyLjIzNnYyODcuNTI4TDI1Niw1MTJsMjI0LjQ3Mi0xMTIuMjM2VjExMi4yMzZMMjU2LDB6IE0yMzQuMjc3LDQ1Mi41NjRMNzQuOTc0LDM3Mi45MTNWMTYwLjgxDQoJCQlsMTU5LjMwMyw3OS42NTFWNDUyLjU2NHogTTEwMS44MjYsMTI1LjY2MkwyNTYsNDguNTc2bDE1NC4xNzQsNzcuMDg3TDI1NiwyMDIuNzQ5TDEwMS44MjYsMTI1LjY2MnogTTQzNy4wMjYsMzcyLjkxMw0KCQkJbC0xNTkuMzAzLDc5LjY1MVYyNDAuNDYxbDE1OS4zMDMtNzkuNjUxVjM3Mi45MTN6IiBmaWxsPSIjRkZGIi8+DQoJPC9nPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPGc+DQo8L2c+DQo8Zz4NCjwvZz4NCjxnPg0KPC9nPg0KPC9zdmc+DQo=" height="22">][crates-url]
[<img alt="crates.io" src="https://img.shields.io/crates/d/cheap-clone?color=critical&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBzdGFuZGFsb25lPSJubyI/PjwhRE9DVFlQRSBzdmcgUFVCTElDICItLy9XM0MvL0RURCBTVkcgMS4xLy9FTiIgImh0dHA6Ly93d3cudzMub3JnL0dyYXBoaWNzL1NWRy8xLjEvRFREL3N2ZzExLmR0ZCI+PHN2ZyB0PSIxNjQ1MTE3MzMyOTU5IiBjbGFzcz0iaWNvbiIgdmlld0JveD0iMCAwIDEwMjQgMTAyNCIgdmVyc2lvbj0iMS4xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHAtaWQ9IjM0MjEiIGRhdGEtc3BtLWFuY2hvci1pZD0iYTMxM3guNzc4MTA2OS4wLmkzIiB3aWR0aD0iNDgiIGhlaWdodD0iNDgiIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIj48ZGVmcz48c3R5bGUgdHlwZT0idGV4dC9jc3MiPjwvc3R5bGU+PC9kZWZzPjxwYXRoIGQ9Ik00NjkuMzEyIDU3MC4yNHYtMjU2aDg1LjM3NnYyNTZoMTI4TDUxMiA3NTYuMjg4IDM0MS4zMTIgNTcwLjI0aDEyOHpNMTAyNCA2NDAuMTI4QzEwMjQgNzgyLjkxMiA5MTkuODcyIDg5NiA3ODcuNjQ4IDg5NmgtNTEyQzEyMy45MDQgODk2IDAgNzYxLjYgMCA1OTcuNTA0IDAgNDUxLjk2OCA5NC42NTYgMzMxLjUyIDIyNi40MzIgMzAyLjk3NiAyODQuMTYgMTk1LjQ1NiAzOTEuODA4IDEyOCA1MTIgMTI4YzE1Mi4zMiAwIDI4Mi4xMTIgMTA4LjQxNiAzMjMuMzkyIDI2MS4xMkM5NDEuODg4IDQxMy40NCAxMDI0IDUxOS4wNCAxMDI0IDY0MC4xOTJ6IG0tMjU5LjItMjA1LjMxMmMtMjQuNDQ4LTEyOS4wMjQtMTI4Ljg5Ni0yMjIuNzItMjUyLjgtMjIyLjcyLTk3LjI4IDAtMTgzLjA0IDU3LjM0NC0yMjQuNjQgMTQ3LjQ1NmwtOS4yOCAyMC4yMjQtMjAuOTI4IDIuOTQ0Yy0xMDMuMzYgMTQuNC0xNzguMzY4IDEwNC4zMi0xNzguMzY4IDIxNC43MiAwIDExNy45NTIgODguODMyIDIxNC40IDE5Ni45MjggMjE0LjRoNTEyYzg4LjMyIDAgMTU3LjUwNC03NS4xMzYgMTU3LjUwNC0xNzEuNzEyIDAtODguMDY0LTY1LjkyLTE2NC45MjgtMTQ0Ljk2LTE3MS43NzZsLTI5LjUwNC0yLjU2LTUuODg4LTMwLjk3NnoiIGZpbGw9IiNmZmZmZmYiIHAtaWQ9IjM0MjIiIGRhdGEtc3BtLWFuY2hvci1pZD0iYTMxM3guNzc4MTA2OS4wLmkwIiBjbGFzcz0iIj48L3BhdGg+PC9zdmc+&style=for-the-badge" height="22">][crates-url]
<img alt="license" src="https://img.shields.io/badge/License-Apache%202.0/MIT-blue.svg?style=for-the-badge&fontColor=white&logoColor=f5c076&logo=data:image/svg+xml;base64,PCFET0NUWVBFIHN2ZyBQVUJMSUMgIi0vL1czQy8vRFREIFNWRyAxLjEvL0VOIiAiaHR0cDovL3d3dy53My5vcmcvR3JhcGhpY3MvU1ZHLzEuMS9EVEQvc3ZnMTEuZHRkIj4KDTwhLS0gVXBsb2FkZWQgdG86IFNWRyBSZXBvLCB3d3cuc3ZncmVwby5jb20sIFRyYW5zZm9ybWVkIGJ5OiBTVkcgUmVwbyBNaXhlciBUb29scyAtLT4KPHN2ZyBmaWxsPSIjZmZmZmZmIiBoZWlnaHQ9IjgwMHB4IiB3aWR0aD0iODAwcHgiIHZlcnNpb249IjEuMSIgaWQ9IkNhcGFfMSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIgdmlld0JveD0iMCAwIDI3Ni43MTUgMjc2LjcxNSIgeG1sOnNwYWNlPSJwcmVzZXJ2ZSIgc3Ryb2tlPSIjZmZmZmZmIj4KDTxnIGlkPSJTVkdSZXBvX2JnQ2FycmllciIgc3Ryb2tlLXdpZHRoPSIwIi8+Cg08ZyBpZD0iU1ZHUmVwb190cmFjZXJDYXJyaWVyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiLz4KDTxnIGlkPSJTVkdSZXBvX2ljb25DYXJyaWVyIj4gPGc+IDxwYXRoIGQ9Ik0xMzguMzU3LDBDNjIuMDY2LDAsMCw2Mi4wNjYsMCwxMzguMzU3czYyLjA2NiwxMzguMzU3LDEzOC4zNTcsMTM4LjM1N3MxMzguMzU3LTYyLjA2NiwxMzguMzU3LTEzOC4zNTcgUzIxNC42NDgsMCwxMzguMzU3LDB6IE0xMzguMzU3LDI1OC43MTVDNzEuOTkyLDI1OC43MTUsMTgsMjA0LjcyMywxOCwxMzguMzU3UzcxLjk5MiwxOCwxMzguMzU3LDE4IHMxMjAuMzU3LDUzLjk5MiwxMjAuMzU3LDEyMC4zNTdTMjA0LjcyMywyNTguNzE1LDEzOC4zNTcsMjU4LjcxNXoiLz4gPHBhdGggZD0iTTE5NC43OTgsMTYwLjkwM2MtNC4xODgtMi42NzctOS43NTMtMS40NTQtMTIuNDMyLDIuNzMyYy04LjY5NCwxMy41OTMtMjMuNTAzLDIxLjcwOC0zOS42MTQsMjEuNzA4IGMtMjUuOTA4LDAtNDYuOTg1LTIxLjA3OC00Ni45ODUtNDYuOTg2czIxLjA3Ny00Ni45ODYsNDYuOTg1LTQ2Ljk4NmMxNS42MzMsMCwzMC4yLDcuNzQ3LDM4Ljk2OCwyMC43MjMgYzIuNzgyLDQuMTE3LDguMzc1LDUuMjAxLDEyLjQ5NiwyLjQxOGM0LjExOC0yLjc4Miw1LjIwMS04LjM3NywyLjQxOC0xMi40OTZjLTEyLjExOC0xNy45MzctMzIuMjYyLTI4LjY0NS01My44ODItMjguNjQ1IGMtMzUuODMzLDAtNjQuOTg1LDI5LjE1Mi02NC45ODUsNjQuOTg2czI5LjE1Miw2NC45ODYsNjQuOTg1LDY0Ljk4NmMyMi4yODEsMCw0Mi43NTktMTEuMjE4LDU0Ljc3OC0zMC4wMDkgQzIwMC4yMDgsMTY5LjE0NywxOTguOTg1LDE2My41ODIsMTk0Ljc5OCwxNjAuOTAzeiIvPiA8L2c+IDwvZz4KDTwvc3ZnPg==" height="22">

</div>

## Installation

```toml
[dependencies]
cheap-clone = "1"
```

## Deriving `CheapClone`

Enable the optional `derive` feature:

```toml
[dependencies]
cheap-clone = { version = "1", features = ["derive"] }
```

```rust
#[derive(Clone, cheap_clone::CheapClone)]
#[cheap_clone(inherent)]
struct Message {
  id: u64,
  body: std::sync::Arc<str>,
  #[cheap_clone(with = "crate::clone_tag")]
  tag: Tag,
}

// An external type need not implement CheapClone when a custom function is used.
#[derive(Clone, Copy)]
struct Tag(u8);

fn clone_tag(tag: &Tag) -> Tag {
  *tag
}
```

The derive generates `CheapClone` by calling each field's `CheapClone` trait
method using a fully qualified path. It does not generate `Clone`; derive or
implement that separately. It supports structs and enums, including lifetimes,
const generics, and existing `where` clauses. Unions and packed types are rejected.

| Attribute | Location | Behavior |
| --- | --- | --- |
| `#[cheap_clone(inherent)]` | Type | Also generates a public `cheap_clone()` method forwarding to the trait. Callers do not need to import the trait. |
| `#[cheap_clone(with = "path::to::function")]` | Field | Calls the function with `&FieldType` and expects `FieldType` in return. Omits this field's automatic `CheapClone` bound. |
| `#[cheap_clone(crate = "::renamed_crate")]` | Type | Overrides the automatically detected dependency path, for example when using a facade. The path must expose `__private::CheapClone`. |

The `with` attribute works on named and tuple fields in structs and enum
variants. Function paths may include generic arguments or qualified associated
functions, such as `crate::copy_value::<T>` or `<T as crate::Trait>::copy_value`.
Paths are resolved where the derive is used; use `crate::`, `super::`, or `::`
as appropriate. A custom function is responsible for preserving the field's
value and being cheap to call. It takes precedence even when the field already
implements `CheapClone`.

Bounds apply to each field's complete type. For example, an `Arc<T>` field
requires `Arc<T>: CheapClone`, not `T: CheapClone`. The type must also implement
`Clone`; Rust's built-in `#[derive(Clone)]` can introduce extra generic bounds,
so a manual `Clone` implementation may be preferable for generic wrappers.

Inherent methods are opt-in. Without `#[cheap_clone(inherent)]`, import
`cheap_clone::CheapClone` or use `cheap_clone::CheapClone::cheap_clone(&value)`.
Generated trait paths go through `::cheap_clone::__private::CheapClone`, using
the detected dependency name when renamed in Cargo.toml. The `derive` feature
works with `default-features = false`, without requiring `std` or `alloc` in
the consuming crate, and supports Rust `1.85`.

### Re-exporting through a facade

An explicit `crate` path takes precedence over Cargo dependency-name detection.
A consumer can depend only on a facade that re-exports `cheap_clone`:

```rust,ignore
// In the facade crate named `path`:
pub mod to {
    pub use cheap_clone;
}

// In a consumer that depends on `path`, not directly on cheap-clone:
#[derive(Clone, path::to::cheap_clone::CheapClone)]
#[cheap_clone(crate = "path::to::cheap_clone", inherent)]
struct Message {
    id: u64,
}
```

The macro uses `path::to::cheap_clone::__private::CheapClone` for the trait
implementation, field bounds, field calls, and inherent forwarding method.
Absolute paths such as `"::path::to::cheap_clone"` and local paths such as
`"crate::facade"` are supported too. The facade must expose `__private`, which
re-exporting the whole crate preserves.

## Optional integrations

Integrations are opt-in. Each versioned feature has a short alias:

| Feature | Alias | Supported types |
| --- | --- | --- |
| `among_1` | `among` | `Among<L, M, R>` with `CheapClone` fields |
| `arcstr_1` | `arcstr` | `ArcStr`, `Substr` |
| `bytes_1` | `bytes` | Immutable `Bytes` |
| `chrono_0_4` | `chrono` | `NaiveDate`, `NaiveTime`, `NaiveDateTime`, `DateTime<Tz>`, `TimeDelta`/`Duration`, `FixedOffset`, `Utc`, `Weekday`, `Month`, `Days`, `Months` |
| `cower_0_1` | `cower` | `Cow<'a, T, P>` with `T::Storage<'a>: CheapClone`; `CowRepr<'a, T, P>` with `T::Owned: CheapClone` |
| `ecow_0_3` | `ecow` | `EcoString`, `EcoBytes`, `EcoVec<T: Clone>` |
| `either_1` | `either` | `Either<L, R>` with `CheapClone` fields |
| `emailaddr_1` | `emailaddr` | `EmailAddr<S, P>`, `LocalPart<S>`, `DomainPart<S>` with `S: CheapClone`; `Buffer`, policies, options and parse errors; with `alloc`/`std`, also `ParseEmailAddrOutput<S, P>` |
| `faststr_0_2` | `faststr` | `FastStr` |
| `generic-array_1` | `generic-array` | `GenericArray<T: Copy, N>` with a `Copy` backing array |
| `generic-arraydeque_0_2` | `generic-arraydeque` | `GenericArrayDeque<T: Copy, N>` |
| `heapless_0_9` | `heapless` | `Vec<T: Copy, N, LenT>`, `Deque<T: Copy, N>`, `String<N, LenT>` |
| `hybrid-array_0_4` | `hybrid-array` | `Array<T: Copy, N>` (including `ArrayN`) with a `Copy` backing array |
| `hybrid-arraydeque_0_1` | `hybrid-arraydeque` | `ArrayDeque<T: Copy, N>` (including `ArrayDequeN`) |
| `hipstr_0_8` | `hipstr` | `HipStr`, `HipByt` and their `Local` variants; with `std`, also `HipOsStr`, `HipPath` and their `Local` variants |
| `hostaddr_1` | `hostaddr` | hostaddr **1.1+**: `Domain<S>`, `Host<S>`, `HostAddr<S>` with `S: CheapClone`, and `Buffer` |
| `http_1` | `http` | `HeaderName`, `HeaderValue`, `Uri`, `Scheme`, `Authority`, `PathAndQuery`, `StatusCode`, `Version` |
| `jiff_1` | `jiff` | Jiff **1.x**: `civil::{Date, Time, DateTime, Weekday}`, `Timestamp`, `SignedDuration`, `Span`, `ZonedDateTime`, `tz::{Offset, TimeZone}`, `Unit`, `RoundMode` |
| `rust_decimal_1` | `rust_decimal` | `Decimal` |
| `smol_str_0_2`, `smol_str_0_3` | `smol_str` → 0.3 | `SmolStr` |
| `smol-bytes_0_1`, `smol-bytes_0_2` | `smol-bytes` → 0.2 | `Buffer`, `Utf8Buffer`, immutable `shared::{Bytes, Utf8Bytes}` and `compact::{Bytes, Utf8Bytes}` |
| `time_0_3` | `time` | `Date`, `Time`, `PlainDateTime`/`PrimitiveDateTime`, `OffsetDateTime`, `UtcDateTime`, `Timestamp`, `SignedDuration`/`Duration`, `UtcOffset`, `Month`, `Weekday` |
| `triomphe_0_1` | `triomphe` | `Arc<T: ?Sized>`, `ArcBorrow`, `OffsetArc`, `ThinArc`, `ArcUnion` |
| `uuid_1` | `uuid` | `Uuid` |

```toml
cheap-clone = { version = "1", features = ["derive", "arcstr", "jiff", "uuid"] }
```

## Rust version support

The base crate, `derive`, default configuration, `--no-default-features`, and
`alloc` support Rust `1.85`. The `cower` integration is also tested on Rust
`1.85` in core-only, `alloc`, and `std` configurations. Non-smol canonical
feature pairs are checked with Rust `1.89`; optional integrations follow their
upstream effective MSRV.
`emailaddr` itself requires Rust `1.86`; its allocating configurations and the
`hostaddr` downstream integration fixture are checked with Rust `1.89`.
`smol-bytes` integrations require Rust `1.95`, which is also used for the
all-features checks. Every public feature and alias is checked separately,
and no-std integrations are additionally checked on a bare-metal target.

#### License

`cheap-clone` is under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.

Copyright (c) 2026 Al Liu.

[Github-url]: https://github.com/al8n/cheap-clone
[CI-url]: https://github.com/al8n/cheap-clone/actions/workflows/ci.yml
[doc-url]: https://docs.rs/cheap-clone
[crates-url]: https://crates.io/crates/cheap-clone
[codecov-url]: https://app.codecov.io/gh/al8n/cheap-clone/
