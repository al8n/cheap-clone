#![cfg(feature = "derive")]

use core::cell::Cell;

#[derive(Clone, Debug, PartialEq, cheap_clone::CheapClone)]
#[cheap_clone(inherent)]
struct Named {
  id: u64,
  r#type: bool,
}

#[derive(Clone, Debug, PartialEq, cheap_clone::CheapClone)]
struct Tuple(u32, &'static str);

#[derive(Clone, Debug, PartialEq, cheap_clone::CheapClone)]
struct Unit;

#[derive(Clone, Debug, PartialEq, cheap_clone::CheapClone)]
struct EmptyNamed {}

#[derive(Clone, Debug, PartialEq, cheap_clone::CheapClone)]
struct EmptyTuple();

#[derive(Clone, Debug, PartialEq, cheap_clone::CheapClone)]
#[cheap_clone(inherent)]
enum Message {
  Unit,
  Tuple(u64, bool),
  Named { r#type: u8 },
  EmptyNamed {},
  EmptyTuple(),
}

#[derive(Clone, cheap_clone::CheapClone)]
enum Never {}

static_assertions::assert_impl_all!(Never: cheap_clone::CheapClone);

#[test]
fn all_struct_and_enum_shapes() {
  let never: Option<Never> = None;
  assert!(cheap_clone::CheapClone::cheap_clone(&never).is_none());
  let value = Named {
    id: 42,
    r#type: true,
  };
  // No trait import: this call must use the opt-in inherent method.
  assert_eq!(value.cheap_clone(), value);
  let value = Tuple(7, "hello");
  assert_eq!(cheap_clone::CheapClone::cheap_clone(&value), value);
  assert_eq!(cheap_clone::CheapClone::cheap_clone(&Unit), Unit);
  assert_eq!(
    cheap_clone::CheapClone::cheap_clone(&EmptyNamed {}),
    EmptyNamed {}
  );
  assert_eq!(
    cheap_clone::CheapClone::cheap_clone(&EmptyTuple()),
    EmptyTuple()
  );
  for value in [
    Message::Unit,
    Message::Tuple(1, true),
    Message::Named { r#type: 2 },
    Message::EmptyNamed {},
    Message::EmptyTuple(),
  ] {
    assert_eq!(value.cheap_clone(), value);
  }
}

struct Tracked<'a> {
  clone_calls: &'a Cell<usize>,
  cheap_calls: &'a Cell<usize>,
}

impl Clone for Tracked<'_> {
  fn clone(&self) -> Self {
    self.clone_calls.set(self.clone_calls.get() + 1);
    Self {
      clone_calls: self.clone_calls,
      cheap_calls: self.cheap_calls,
    }
  }
}

impl cheap_clone::CheapClone for Tracked<'_> {
  fn cheap_clone(&self) -> Self {
    self.cheap_calls.set(self.cheap_calls.get() + 1);
    Self {
      clone_calls: self.clone_calls,
      cheap_calls: self.cheap_calls,
    }
  }
}

#[derive(Clone, cheap_clone::CheapClone)]
#[cheap_clone(inherent)]
struct TrackedStruct<'a> {
  field: Tracked<'a>,
}

#[test]
fn calls_each_fields_trait_method_and_inherent_forwards_to_trait() {
  let clone_calls = Cell::new(0);
  let cheap_calls = Cell::new(0);
  let value = TrackedStruct {
    field: Tracked {
      clone_calls: &clone_calls,
      cheap_calls: &cheap_calls,
    },
  };
  let _ = value.cheap_clone();
  let _ = cheap_clone::CheapClone::cheap_clone(&value);
  assert_eq!(clone_calls.get(), 0);
  assert_eq!(cheap_calls.get(), 2);
  let _ = value.clone();
  assert_eq!(clone_calls.get(), 1);
}

fn custom_tracked<'a>(value: &Tracked<'a>) -> Tracked<'a> {
  value.cheap_calls.set(value.cheap_calls.get() + 10);
  Tracked {
    clone_calls: value.clone_calls,
    cheap_calls: value.cheap_calls,
  }
}

#[test]
fn custom_function_overrides_existing_cheap_clone_and_runs_once() {
  #[derive(Clone, cheap_clone::CheapClone)]
  #[cheap_clone(inherent)]
  struct Custom<'a>(#[cheap_clone(with = "crate::custom_tracked")] Tracked<'a>);

  let clone_calls = Cell::new(0);
  let cheap_calls = Cell::new(0);
  let value = Custom(Tracked {
    clone_calls: &clone_calls,
    cheap_calls: &cheap_calls,
  });
  let _ = value.cheap_clone();
  assert_eq!(clone_calls.get(), 0);
  assert_eq!(cheap_calls.get(), 10);
}

// Exercise custom clone callbacks on a Clone-only type without CheapClone.
#[derive(Clone, Debug, PartialEq)]
struct Foreign(u8);

fn clone_foreign(value: &Foreign) -> Foreign {
  Foreign(value.0)
}

#[derive(Clone, Debug, PartialEq, cheap_clone::CheapClone)]
#[cheap_clone(inherent)]
struct Custom {
  #[cheap_clone(with = "crate::clone_foreign")]
  field: Foreign,
  other: u32,
}

#[derive(Clone, Debug, PartialEq, cheap_clone::CheapClone)]
#[cheap_clone(inherent)]
struct CustomTuple(#[cheap_clone(with = "crate::clone_foreign")] Foreign);

#[derive(Clone, Debug, PartialEq, cheap_clone::CheapClone)]
#[cheap_clone(inherent)]
enum CustomEnum {
  Named {
    #[cheap_clone(with = "crate::clone_foreign")]
    field: Foreign,
  },
  Tuple(#[cheap_clone(with = "<crate::Foreign as ::core::clone::Clone>::clone")] Foreign),
}

#[test]
fn custom_functions_support_fields_without_cheap_clone() {
  let value = Custom {
    field: Foreign(3),
    other: 4,
  };
  assert_eq!(value.cheap_clone(), value);
  let value = CustomTuple(Foreign(5));
  assert_eq!(value.cheap_clone(), value);
  for value in [
    CustomEnum::Named { field: Foreign(6) },
    CustomEnum::Tuple(Foreign(7)),
  ] {
    assert_eq!(value.cheap_clone(), value);
  }
}

#[derive(Clone, cheap_clone::CheapClone)]
#[cheap_clone(inherent)]
struct GenericCustom<T: Copy>(#[cheap_clone(with = "crate::copy_value::<T>")] T);

fn copy_value<T: Copy>(value: &T) -> T {
  *value
}

#[derive(Clone, cheap_clone::CheapClone)]
struct Generic<T>(T);

static_assertions::assert_impl_all!(Generic<u32>: cheap_clone::CheapClone);
static_assertions::assert_not_impl_any!(Generic<String>: cheap_clone::CheapClone);

#[derive(Clone, cheap_clone::CheapClone)]
#[cheap_clone(inherent)]
struct Packet<'a, T = u8, const N: usize = 4>
where
  T: Copy,
{
  name: &'a str,
  value: T,
  bytes: [u8; N],
}

trait Source {
  type Item: Clone;
}

struct NotClone;

impl Source for NotClone {
  type Item = u8;
}

#[derive(cheap_clone::CheapClone)]
#[cheap_clone(inherent)]
struct Associated<T: Source> {
  value: T::Item,
}

impl<T: Source> Clone for Associated<T> {
  fn clone(&self) -> Self {
    Self {
      value: self.value.clone(),
    }
  }
}

#[test]
fn generics_lifetimes_const_generics_and_associated_types() {
  assert_eq!(cheap_clone::CheapClone::cheap_clone(&Generic(3u32)).0, 3);
  assert_eq!(GenericCustom(1u8).cheap_clone().0, 1);
  let name = String::from("packet");
  let packet = Packet {
    name: &name,
    value: 2u8,
    bytes: [1, 2, 3, 4],
  };
  let cloned = packet.cheap_clone();
  assert_eq!(cloned.name, name);
  assert_eq!(cloned.value, 2);
  assert_eq!(cloned.bytes, packet.bytes);
  let associated = Associated::<NotClone> { value: 7 };
  assert_eq!(associated.cheap_clone().value, 7);
}

#[cfg(feature = "std")]
#[test]
fn arc_inner_types_need_neither_clone_nor_cheap_clone() {
  use std::sync::Arc;

  #[derive(cheap_clone::CheapClone)]
  #[cheap_clone(inherent)]
  struct Handle<T: ?Sized>(Arc<T>);

  impl<T: ?Sized> Clone for Handle<T> {
    fn clone(&self) -> Self {
      Self(Arc::clone(&self.0))
    }
  }

  let value = Handle(Arc::new(NotClone));
  assert!(Arc::ptr_eq(&value.0, &value.cheap_clone().0));
  let value = Handle::<str>(Arc::from("unsized"));
  assert!(Arc::ptr_eq(&value.0, &value.cheap_clone().0));
}

#[test]
fn existing_inherent_method_is_untouched_by_default() {
  #[derive(Clone, cheap_clone::CheapClone)]
  struct Value(u8);

  impl Value {
    fn cheap_clone(&self) -> u8 {
      self.0
    }
  }

  let value = Value(3);
  assert_eq!(value.cheap_clone(), 3);
  assert_eq!(cheap_clone::CheapClone::cheap_clone(&value).0, 3);
}

#[test]
fn absolute_paths_ignore_shadowed_names_and_field_methods() {
  mod cheap_clone {}
  mod core {}

  #[derive(Clone)]
  struct Field(u8);

  impl Field {
    // An inherent method with the same name must never be selected by derive.
    fn cheap_clone(&self) -> u8 {
      self.0
    }
  }

  impl ::cheap_clone::CheapClone for Field {}

  #[derive(Clone, ::cheap_clone::CheapClone)]
  #[cheap_clone(inherent)]
  struct Value(Field);

  assert_eq!(Value(Field(8)).cheap_clone().0.cheap_clone(), 8);
}

mod facade {
  pub use cheap_clone::__private;
}

#[test]
fn explicit_crate_path_supports_facades() {
  #[derive(Clone, cheap_clone::CheapClone)]
  #[cheap_clone(crate = "crate::facade", inherent)]
  struct Value(u8);

  assert_eq!(Value(9).cheap_clone().0, 9);
}

#[derive(Clone, cheap_clone::CheapClone)]
#[cheap_clone(inherent)]
struct Conditional {
  value: u8,
  #[cfg(any())]
  missing: DoesNotExist,
}

#[test]
fn configured_out_fields_are_ignored() {
  assert_eq!(Conditional { value: 10 }.cheap_clone().value, 10);
}
