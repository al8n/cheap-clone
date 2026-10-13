#![cfg(feature = "smol_bytes_0_1")]

use cheap_clone::CheapClone;
use smol_bytes_0_1::{compact, shared, Buffer, Utf8Buffer};
use static_assertions::{assert_impl_all, assert_not_impl_any};

assert_impl_all!(Buffer: CheapClone);
assert_impl_all!(Utf8Buffer: CheapClone);
assert_impl_all!(shared::Bytes: CheapClone);
assert_impl_all!(compact::Bytes: CheapClone);
assert_impl_all!(shared::Utf8Bytes: CheapClone);
assert_impl_all!(compact::Utf8Bytes: CheapClone);
assert_not_impl_any!(smol_bytes_0_1::BytesMut: CheapClone);
assert_not_impl_any!(smol_bytes_0_1::Utf8BytesMut: CheapClone);

#[test]
fn fixed_and_immutable_smol_bytes_clone_cheaply() {
  let buffer = Buffer::new();
  assert_eq!(buffer.cheap_clone(), buffer);

  let utf8_buffer = Utf8Buffer::from_static("inline");
  assert_eq!(utf8_buffer.cheap_clone().as_str(), "inline");

  let shared_inline = shared::Bytes::from_static(b"inline");
  assert_eq!(shared_inline.cheap_clone().as_slice(), b"inline");

  let compact_inline = compact::Bytes::from_static(b"inline");
  assert_eq!(compact_inline.cheap_clone().as_slice(), b"inline");

  let shared_utf8 = shared::Utf8Bytes::from_static("shared");
  assert_eq!(shared_utf8.cheap_clone().as_str(), "shared");

  let compact_utf8 = compact::Utf8Bytes::from_static("compact");
  assert_eq!(compact_utf8.cheap_clone().as_str(), "compact");
}

#[test]
fn immutable_smol_bytes_heap_clones_preserve_contents() {
  static PAYLOAD: [u8; 128] = [b'x'; 128];

  let shared = shared::Bytes::from_static(&PAYLOAD);
  assert!(shared.is_heap());
  assert_eq!(shared.cheap_clone().as_slice(), &PAYLOAD);

  let compact = compact::Bytes::from_static(&PAYLOAD);
  assert!(compact.is_heap());
  assert_eq!(compact.cheap_clone().as_slice(), &PAYLOAD);

  let shared_utf8 = shared::Utf8Bytes::from_static("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
  assert!(shared_utf8.is_heap());
  assert_eq!(shared_utf8.cheap_clone().as_str(), shared_utf8.as_str());

  let compact_utf8 = compact::Utf8Bytes::from_static("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
  assert!(compact_utf8.is_heap());
  assert_eq!(compact_utf8.cheap_clone().as_str(), compact_utf8.as_str());
}
