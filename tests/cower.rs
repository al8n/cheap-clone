#![cfg(feature = "cower_0_1")]

use cheap_clone::CheapClone;
use cower_0_1::{Cow, CowRepr, StaticPolicy, ToOwned};

#[test]
fn static_policy_preserves_borrows_and_owned_references() {
  struct NotClone(u8);
  static VALUE: NotClone = NotClone(42);
  static_assertions::assert_impl_all!(Cow<'static, NotClone, StaticPolicy>: CheapClone);

  let local = NotClone(7);
  let borrowed: Cow<'_, NotClone, StaticPolicy> = Cow::borrowed(&local);
  let cloned = borrowed.cheap_clone();
  assert!(core::ptr::eq(cloned.as_borrowed().unwrap(), &local));
  assert_eq!(cloned.as_ref().0, 7);

  let owned: Cow<'_, NotClone, StaticPolicy> = Cow::owned(&VALUE);
  let cloned = owned.cheap_clone();
  assert!(core::ptr::eq(*cloned.as_owned().unwrap(), &VALUE));

  let text: Cow<'_, str, StaticPolicy> = Cow::owned("shared");
  assert!(text.cheap_clone().is_owned());
  assert_eq!(text.as_ref().as_ptr(), text.cheap_clone().as_ref().as_ptr());
}

// Neither the borrowed value nor the policy needs a Clone implementation.
// An unsized policy also guards against accidentally adding a Sized bound.
struct CustomPolicy {
  _marker: [()],
}
struct CheapNumber(u32);

impl core::borrow::Borrow<u32> for CheapNumber {
  fn borrow(&self) -> &u32 {
    &self.0
  }
}

impl Clone for CheapNumber {
  fn clone(&self) -> Self {
    panic!("the integration must call cheap_clone, not Clone::clone");
  }
}

impl CheapClone for CheapNumber {
  fn cheap_clone(&self) -> Self {
    Self(self.0)
  }
}

impl ToOwned<CustomPolicy> for u32 {
  type Owned = CheapNumber;
  type Storage<'a> = CowRepr<'a, Self, CustomPolicy>;
}

#[test]
fn custom_policy_delegates_to_the_owned_cheap_clone() {
  let repr: CowRepr<'_, u32, CustomPolicy> = CowRepr::Owned(CheapNumber(42));
  let cloned = repr.cheap_clone();
  assert!(cloned.is_owned());
  assert_eq!(*cloned, 42);

  let value: Cow<'_, u32, CustomPolicy> = Cow::from_storage(repr);
  let cloned = value.cheap_clone();
  assert_eq!(cloned.as_owned().unwrap().0, 42);

  let local = 7;
  let repr: CowRepr<'_, u32, CustomPolicy> = CowRepr::Borrowed(&local);
  assert!(matches!(repr.cheap_clone(), CowRepr::Borrowed(value) if core::ptr::eq(value, &local)));
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
#[test]
fn core_default_policy_uses_static_reference_storage() {
  let local = [1, 2, 3];
  let borrowed: Cow<'_, [u8]> = Cow::borrowed(&local);
  assert!(core::ptr::eq(
    borrowed.cheap_clone().as_borrowed().unwrap(),
    &local[..]
  ));
  let owned: Cow<'_, str> = Cow::owned("static");
  assert!(owned.cheap_clone().is_owned());
  assert_eq!(*owned.cheap_clone().as_owned().unwrap(), "static");
}

#[cfg(any(feature = "std", feature = "alloc"))]
mod allocating {
  use super::*;
  use std::sync::Arc;

  struct Shared;
  impl ToOwned<Shared> for str {
    type Owned = Arc<str>;
    type Storage<'a> = CowRepr<'a, Self, Shared>;
  }

  // Even cheap Owned values do not make arbitrary backing storage cheap.
  static_assertions::assert_not_impl_any!(Cow<'static, u32>: CheapClone);
  static_assertions::assert_not_impl_any!(Cow<'static, str>: CheapClone);
  static_assertions::assert_not_impl_any!(CowRepr<'static, str>: CheapClone);
  static_assertions::assert_not_impl_any!(CowRepr<'static, [u8]>: CheapClone);
  static_assertions::assert_impl_all!(CowRepr<'static, u32>: CheapClone);

  #[test]
  fn shared_policy_preserves_ownership_and_shares_allocation() {
    let owned: Cow<'_, str, Shared> = Cow::owned(Arc::from("payload".repeat(64)));
    let cloned = owned.cheap_clone();
    assert!(Arc::ptr_eq(
      owned.as_owned().unwrap(),
      cloned.as_owned().unwrap()
    ));
    assert_eq!(Arc::strong_count(owned.as_owned().unwrap()), 2);

    let local = String::from("borrowed");
    let borrowed: Cow<'_, str, Shared> = Cow::borrowed(&local);
    let cloned = borrowed.cheap_clone();
    assert!(core::ptr::eq(cloned.as_borrowed().unwrap(), local.as_str()));
  }
}

#[cfg(feature = "derive")]
#[test]
fn derived_struct_supports_cower_fields() {
  #[derive(Clone, cheap_clone::CheapClone)]
  struct Message<'a> {
    text: Cow<'a, str, StaticPolicy>,
  }

  let message = Message {
    text: Cow::owned("message"),
  };
  let cloned = message.cheap_clone();
  assert!(cloned.text.is_owned());
  assert_eq!(cloned.text.as_ref(), "message");
}
