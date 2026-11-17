#![no_std]
#![forbid(unsafe_code)]
#![deny(warnings, missing_docs)]
//! A facade exporting the runtime and derive through a nested path.

/// Facade namespace.
pub mod to {
  pub use renamed as cheap_clone;
}
