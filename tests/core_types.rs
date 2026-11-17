use cheap_clone::CheapClone;
use core::{cmp::Ordering, marker::PhantomData, marker::PhantomPinned, time::Duration};
use static_assertions::assert_impl_all;

struct NotClone;
assert_impl_all!(PhantomData<NotClone>: CheapClone);
assert_impl_all!(PhantomData<[NotClone]>: CheapClone);
assert_impl_all!(PhantomPinned: CheapClone);
assert_impl_all!(Duration: CheapClone);
assert_impl_all!(Ordering: CheapClone);
assert_impl_all!(&[NotClone]: CheapClone);
assert_impl_all!(&dyn core::fmt::Debug: CheapClone);

#[test]
fn unsized_references_keep_identity() {
  let values = [NotClone, NotClone];
  let slice = &values[..];
  assert!(core::ptr::eq(slice, slice.cheap_clone()));
  let object: &dyn core::fmt::Debug = &42;
  assert!(core::ptr::eq(object, object.cheap_clone()));
}

#[test]
fn numeric_wrappers_use_the_inner_cheap_clone() {
  struct Value(u8);
  impl Clone for Value {
    fn clone(&self) -> Self {
      panic!("must use CheapClone");
    }
  }
  impl CheapClone for Value {
    fn cheap_clone(&self) -> Self {
      Self(self.0)
    }
  }
  assert_eq!(core::num::Wrapping(Value(3)).cheap_clone().0.0, 3);
  assert_eq!(core::num::Saturating(Value(4)).cheap_clone().0.0, 4);
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[test]
fn weak_clones_share_allocations_without_keeping_them_alive() {
  use std::{rc::Rc, sync::Arc};

  let rc: Rc<[NotClone]> = Rc::from(vec![NotClone]);
  let weak = Rc::downgrade(&rc);
  let cloned = weak.cheap_clone();
  assert!(std::rc::Weak::ptr_eq(&weak, &cloned));
  assert_eq!(Rc::strong_count(&rc), 1);
  drop(rc);
  assert!(cloned.upgrade().is_none());

  let arc: Arc<[NotClone]> = Arc::from(vec![NotClone]);
  let weak = Arc::downgrade(&arc);
  let cloned = weak.cheap_clone();
  assert!(std::sync::Weak::ptr_eq(&weak, &cloned));
  assert_eq!(Arc::strong_count(&arc), 1);
  drop(arc);
  assert!(cloned.upgrade().is_none());
}

#[cfg(feature = "std")]
assert_impl_all!(std::time::Instant: CheapClone);
#[cfg(feature = "std")]
assert_impl_all!(std::time::SystemTime: CheapClone);
