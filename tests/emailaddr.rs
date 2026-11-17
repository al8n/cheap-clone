#![cfg(feature = "emailaddr_1")]

use cheap_clone::CheapClone;
use emailaddr_1::{Buffer, DomainPart, EmailAddr, LocalPart, Relax, Strict};
use static_assertions::{assert_impl_all, assert_not_impl_any};

assert_impl_all!(EmailAddr<Buffer>: CheapClone);
assert_impl_all!(EmailAddr<Buffer, Relax>: CheapClone);
assert_impl_all!(LocalPart<&str>: CheapClone);
assert_impl_all!(DomainPart<&[u8]>: CheapClone);
assert_not_impl_any!(EmailAddr<String>: CheapClone);
assert_not_impl_any!(EmailAddr<String, Relax>: CheapClone);
assert_not_impl_any!(EmailAddr<Vec<u8>>: CheapClone);
assert_not_impl_any!(EmailAddr<Box<str>>: CheapClone);
assert_not_impl_any!(EmailAddr<std::borrow::Cow<'static, str>>: CheapClone);
assert_not_impl_any!(LocalPart<String>: CheapClone);
assert_not_impl_any!(DomainPart<Vec<u8>>: CheapClone);

#[derive(Clone)]
struct CloneOnlyPolicy;
assert_impl_all!(EmailAddr<Buffer, CloneOnlyPolicy>: CheapClone);

#[test]
fn borrowed_addresses_and_parts_keep_their_storage() {
  let text = "\"user name\"@example.com";
  let addr = EmailAddr::<&str>::try_from(text).unwrap();
  let cloned = addr.cheap_clone();
  assert_eq!(cloned, addr);
  assert_eq!(cloned.as_str().as_ptr(), text.as_ptr());
  let local = addr.local_part();
  let domain = addr.domain_part();
  assert!(core::ptr::eq(
    *local.as_inner(),
    *local.cheap_clone().as_inner()
  ));
  assert!(core::ptr::eq(
    *domain.as_inner(),
    *domain.cheap_clone().as_inner()
  ));

  let bytes = EmailAddr::<&[u8]>::try_from(text.as_bytes()).unwrap();
  assert_eq!(bytes.cheap_clone().as_bytes().as_ptr(), text.as_ptr());
  let dst = EmailAddr::try_from_ascii_str(text).unwrap();
  assert!(core::ptr::eq(dst, dst.cheap_clone()));
  let local_dst = LocalPart::try_from_ascii_str("user").unwrap();
  assert!(core::ptr::eq(local_dst, local_dst.cheap_clone()));
  let domain_dst = DomainPart::try_from_ascii_bytes(b"example.com").unwrap();
  assert!(core::ptr::eq(domain_dst, domain_dst.cheap_clone()));
}

#[test]
fn stack_addresses_preserve_strict_and_relaxed_policies() {
  use emailaddr_1::{DomainOptions, Options};

  let strict = EmailAddr::<Buffer>::try_from("user@example.com").unwrap();
  assert_eq!(strict.cheap_clone(), strict);
  assert_eq!(strict.as_inner().cheap_clone(), *strict.as_inner());
  let options = Options::new().with_domain(DomainOptions::new().with_minimum_dns_labels(1));
  let relaxed = EmailAddr::<Buffer, Relax>::parse_with_options("user@localhost", options).unwrap();
  assert_eq!(relaxed.cheap_clone(), relaxed);
  assert_eq!(relaxed.cheap_clone().as_str(), "user@localhost");
  assert_eq!(options.cheap_clone(), options);
  assert_eq!(options.local().cheap_clone(), options.local());
  assert_eq!(options.domain().cheap_clone(), options.domain());
  assert_eq!(options.limits().cheap_clone(), options.limits());
  assert_eq!(Strict.cheap_clone(), Strict);
  assert_eq!(Relax.cheap_clone(), Relax);
}

#[test]
fn parse_errors_preserve_the_error() {
  let error = EmailAddr::<Buffer>::try_from("invalid").unwrap_err();
  assert_eq!(error.cheap_clone(), error);
  let error = LocalPart::try_from_ascii_str("invalid space").unwrap_err();
  assert_eq!(error.cheap_clone(), error);
  let error = DomainPart::try_from_ascii_str("-invalid.com").unwrap_err();
  assert_eq!(error.cheap_clone(), error);
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn arc_and_rc_addresses_share_the_allocation() {
  use std::{rc::Rc, sync::Arc};

  let addr = EmailAddr::<Arc<str>>::try_from("user@example.com").unwrap();
  let cloned = addr.cheap_clone();
  assert!(Arc::ptr_eq(addr.as_inner(), cloned.as_inner()));
  let relaxed: EmailAddr<_, Relax> = addr.into();
  assert!(Arc::ptr_eq(
    relaxed.as_inner(),
    relaxed.cheap_clone().as_inner()
  ));

  let addr = EmailAddr::<Rc<str>>::try_from("user@example.com").unwrap();
  assert!(Rc::ptr_eq(addr.as_inner(), addr.cheap_clone().as_inner()));
  let addr = EmailAddr::<Arc<[u8]>>::try_from(b"user@example.com".as_slice()).unwrap();
  assert!(Arc::ptr_eq(addr.as_inner(), addr.cheap_clone().as_inner()));
}

#[cfg(any(feature = "alloc", feature = "std"))]
#[test]
fn parse_outputs_preserve_original_and_normalized_variants() {
  use emailaddr_1::ParseEmailAddrOutput;

  assert_not_impl_any!(ParseEmailAddrOutput<String>: CheapClone);
  let input = "user@example.com";
  let original = EmailAddr::<&str>::try_from_str(input).unwrap();
  let ParseEmailAddrOutput::Original(cloned) = original.cheap_clone() else {
    panic!("original variant changed");
  };
  assert_eq!(cloned.as_str().as_ptr(), input.as_ptr());

  let normalized = EmailAddr::<&str>::try_from_str("user@测试.中国").unwrap();
  let ParseEmailAddrOutput::Normalized(cloned) = normalized.cheap_clone() else {
    panic!("normalized variant changed");
  };
  assert_eq!(cloned.as_str(), "user@xn--0zwm56d.xn--fiqs8s");
  assert_eq!(normalized.cheap_clone(), normalized);
}
