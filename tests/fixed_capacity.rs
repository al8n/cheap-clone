#![cfg(any(
  feature = "hybrid-array_0_4",
  feature = "generic-array_1",
  feature = "heapless_0_9",
  feature = "hybrid-arraydeque_0_1",
  feature = "generic-arraydeque_0_2",
))]

use cheap_clone::CheapClone;
use static_assertions::assert_not_impl_any;

#[derive(Copy, Debug, PartialEq, Eq)]
struct CopyOnly(u8);

// Ensure the integration copies values rather than calling arbitrary Clone code.
#[allow(clippy::non_canonical_clone_impl)]
impl Clone for CopyOnly {
  fn clone(&self) -> Self {
    panic!("must copy directly");
  }
}

#[cfg(feature = "hybrid-array_0_4")]
#[test]
fn hybrid_arrays_copy_elements_including_empty_arrays() {
  use hybrid_array_0_4::{
    Array,
    typenum::{U0, U4},
  };
  assert_not_impl_any!(Array<String, U4>: CheapClone);
  let value = Array::<_, U4>::from([CopyOnly(1), CopyOnly(2), CopyOnly(3), CopyOnly(4)]);
  assert_eq!(value.cheap_clone(), value);
  let empty = Array::<CopyOnly, U0>::from([]);
  assert_eq!(empty.cheap_clone(), empty);
}

#[cfg(feature = "generic-array_1")]
#[test]
fn generic_arrays_copy_elements_including_empty_arrays() {
  use generic_array_1::{
    GenericArray,
    typenum::{U0, U4},
  };
  assert_not_impl_any!(GenericArray<String, U4>: CheapClone);
  let value = GenericArray::<_, U4>::from([CopyOnly(1), CopyOnly(2), CopyOnly(3), CopyOnly(4)]);
  assert_eq!(value.cheap_clone(), value);
  let empty = GenericArray::<CopyOnly, U0>::from([]);
  assert_eq!(empty.cheap_clone(), empty);
}

#[cfg(any(feature = "hybrid-arraydeque_0_1", feature = "generic-arraydeque_0_2"))]
macro_rules! arraydeque_test {
  ($test:ident, $deque:ty) => {
    #[test]
    fn $test() {
      let mut deque = <$deque>::new();
      assert!(deque.cheap_clone().is_empty());
      for value in 0..4 {
        assert!(deque.push_back(CopyOnly(value)).is_none());
      }
      assert_eq!(deque.pop_front(), Some(CopyOnly(0)));
      assert_eq!(deque.pop_front(), Some(CopyOnly(1)));
      assert!(deque.push_back(CopyOnly(4)).is_none());
      assert!(deque.push_back(CopyOnly(5)).is_none());
      // Full, with the ring split across the end of its backing array.
      let mut cloned = deque.cheap_clone();
      assert_eq!(cloned.iter().map(|x| x.0).collect::<Vec<_>>(), [2, 3, 4, 5]);
      assert_eq!(cloned.pop_back(), Some(CopyOnly(5)));
      assert_eq!(deque.len(), 4);
      let cloned = cloned.cheap_clone();
      assert_eq!(cloned.iter().map(|x| x.0).collect::<Vec<_>>(), [2, 3, 4]);
    }
  };
}

#[cfg(feature = "hybrid-arraydeque_0_1")]
arraydeque_test!(
  hybrid_deque_preserves_wrapped_order,
  hybrid_arraydeque_0_1::ArrayDeque<CopyOnly, hybrid_arraydeque_0_1::typenum::U4>
);
#[cfg(feature = "hybrid-arraydeque_0_1")]
assert_not_impl_any!(hybrid_arraydeque_0_1::ArrayDeque<String, hybrid_arraydeque_0_1::typenum::U4>: CheapClone);

#[cfg(feature = "generic-arraydeque_0_2")]
arraydeque_test!(
  generic_deque_preserves_wrapped_order,
  generic_arraydeque_0_2::GenericArrayDeque<CopyOnly, generic_arraydeque_0_2::typenum::U4>
);
#[cfg(feature = "generic-arraydeque_0_2")]
assert_not_impl_any!(generic_arraydeque_0_2::GenericArrayDeque<String, generic_arraydeque_0_2::typenum::U4>: CheapClone);

#[cfg(feature = "heapless_0_9")]
#[test]
fn heapless_copies_values_and_utf8_without_allocation() {
  use heapless_0_9::{Deque, String as InlineString, Vec as InlineVec};
  assert_not_impl_any!(InlineVec<String, 4>: CheapClone);
  assert_not_impl_any!(Deque<String, 4>: CheapClone);

  let mut values = InlineVec::<CopyOnly, 4, u8>::new();
  assert!(values.cheap_clone().is_empty());
  for value in 0..4 {
    values.push(CopyOnly(value)).unwrap();
  }
  let mut cloned = values.cheap_clone();
  assert_eq!(values, cloned);
  cloned.pop();
  assert_eq!(values.len(), 4);

  let mut deque = Deque::<CopyOnly, 4>::new();
  assert!(deque.cheap_clone().is_empty());
  for value in 0..4 {
    deque.push_back(CopyOnly(value)).unwrap();
  }
  deque.pop_front();
  deque.pop_front();
  deque.push_back(CopyOnly(4)).unwrap();
  deque.push_back(CopyOnly(5)).unwrap();
  let cloned = deque.cheap_clone();
  assert_eq!(cloned.iter().map(|x| x.0).collect::<Vec<_>>(), [2, 3, 4, 5]);

  let text = InlineString::<16, u8>::try_from("共享🦀").unwrap();
  assert_eq!(text.cheap_clone().as_str(), text.as_str());
}
