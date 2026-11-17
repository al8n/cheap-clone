#![no_std]
#![forbid(unsafe_code)]
#![deny(warnings, missing_docs)]
//! A consumer with no direct dependency on cheap-clone.

#[derive(Clone, path::to::cheap_clone::CheapClone)]
#[cheap_clone(crate = "path::to::cheap_clone", inherent)]
struct Message<'a> {
  bytes: &'a [u8],
  #[cheap_clone(with = "crate::copy_tag")]
  tag: Tag,
}

#[derive(Clone, Copy)]
struct Tag(u8);

fn copy_tag(tag: &Tag) -> Tag {
  *tag
}

#[derive(Clone, path::to::cheap_clone::CheapClone)]
#[cheap_clone(crate = "::path::to::cheap_clone")]
enum Envelope<'a> {
  Message(Message<'a>),
}

/// Exercises relative and absolute facade paths without importing the trait.
pub fn clone_through_facade(bytes: &[u8], tag: u8) -> (&[u8], u8) {
  let message = Message {
    bytes,
    tag: Tag(tag),
  }
  .cheap_clone();
  let Envelope::Message(message) =
    path::to::cheap_clone::CheapClone::cheap_clone(&Envelope::Message(message));
  (message.bytes, message.tag.0)
}

#[test]
fn facade_paths_keep_the_field_values() {
  let bytes = [1, 2, 3];
  let (cloned, tag) = clone_through_facade(&bytes, 42);
  assert!(core::ptr::eq(cloned, &bytes[..]));
  assert_eq!(tag, 42);
}
