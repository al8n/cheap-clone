#![no_std]
#![forbid(unsafe_code)]
#![deny(warnings, missing_docs)]
//! Checks expansion in an edition-2024 no-std consumer with a renamed dependency.

/// A downstream type that manually implements the public trait.
#[derive(Clone)]
pub struct Manual(u8);

// A hidden re-export must not seal the public trait or require a derive.
impl ::renamed::CheapClone for Manual {}

/// Exercises a manual implementation through the public trait path.
pub fn manual_clone(value: u8) -> u8 {
  <Manual as ::renamed::CheapClone>::cheap_clone(&Manual(value)).0
}

#[derive(Clone, Copy)]
struct Foreign(u8);

fn copy_foreign(value: &Foreign) -> Foreign {
  *value
}

/// A message using both standard field bounds and a custom function.
#[derive(Clone, ::renamed::CheapClone)]
#[cheap_clone(inherent)]
pub enum Message<'a, const N: usize> {
  /// No data.
  Empty,
  /// Named data.
  Named {
    /// Borrowed text.
    text: &'a str,
    /// A fixed array.
    bytes: [u8; N],
  },
  /// Tuple data.
  Tuple(u32),
}

#[derive(Clone, ::renamed::CheapClone)]
#[cheap_clone(inherent)]
struct Custom(#[cheap_clone(with = "crate::copy_foreign")] Foreign);

/// Exercises the custom field function without importing the trait.
pub fn copied_value() -> u8 {
  Custom(Foreign(42)).cheap_clone().0.0
}

/// Exercises the generated enum code.
pub fn copy_message<'a, const N: usize>(message: &Message<'a, N>) -> Message<'a, N> {
  message.cheap_clone()
}
