//! A trait which indicates that such type can be cloned cheaply.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

/**
 * `CheapClone` trait is inspired by https://github.com/graphprotocol/graph-node/blob/master/graph/src/cheap_clone.rs
 */

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

macro_rules! impl_cheap_clone_for_copy {
  ($($ty: ty), +$(,)?) => {
    $(
      impl crate::CheapClone for $ty {
        fn cheap_clone(&self) -> Self {
          *self
        }
      }
    )*
  };
}

/// Things that are fast to clone in the context of an application such as Graph Node
///
/// The purpose of this API is to reduce the number of calls to .clone() which need to
/// be audited for performance.
///
/// As a rule of thumb, only constant-time Clone impls should also implement CheapClone.
/// Eg:
/// - ✔ [`Arc<T>`](alloc::sync::Arc)
/// - ✔ [`Rc<T>`](alloc::rc::Rc)
/// - ✔ [`Bytes`](bytes::Bytes)
/// - ✗ [`Vec<T>`](alloc::vec::Vec)
/// - ✔ [`SmolStr`](smol_str::SmolStr)
/// - ✗ [`String`]
pub trait CheapClone: Clone {
  /// Returns a copy of the value.
  fn cheap_clone(&self) -> Self {
    self.clone()
  }
}

#[cfg(feature = "bytes")]
impl CheapClone for bytes::Bytes {}

#[cfg(feature = "smol_str")]
impl CheapClone for smol_str::SmolStr {}

#[cfg(feature = "alloc")]
mod a {
  use super::CheapClone;

  impl<T: ?Sized> CheapClone for alloc::rc::Rc<T> {}
  impl<T: ?Sized> CheapClone for alloc::sync::Arc<T> {}
  impl<T: ?Sized + CheapClone> CheapClone for alloc::boxed::Box<T> {}
}

#[cfg(feature = "std")]
mod s {
  use super::CheapClone;

  impl<T: ?Sized + CheapClone> CheapClone for std::pin::Pin<T> {}

  impl_cheap_clone_for_copy!(
    std::net::IpAddr,
    std::net::Ipv4Addr,
    std::net::Ipv6Addr,
    std::net::SocketAddr,
    std::net::SocketAddrV4,
    std::net::SocketAddrV6,
  );
}

impl<T: CheapClone> CheapClone for Option<T> {}
impl<T: CheapClone, E: CheapClone> CheapClone for Result<T, E> {}
#[cfg(feature = "either")]
impl<L: CheapClone, R: CheapClone> CheapClone for either::Either<L, R> {}

impl_cheap_clone_for_copy! {
  bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
  core::num::NonZeroI8,
  core::num::NonZeroI16,
  core::num::NonZeroI32,
  core::num::NonZeroI64,
  core::num::NonZeroI128,
  core::num::NonZeroIsize,
  core::num::NonZeroU8,
  core::num::NonZeroU16,
  core::num::NonZeroU32,
  core::num::NonZeroU64,
  core::num::NonZeroU128,
  core::num::NonZeroUsize,
  &str
}

impl<T: Copy, const N: usize> CheapClone for [T; N] {
  fn cheap_clone(&self) -> Self {
    *self
  }
}
