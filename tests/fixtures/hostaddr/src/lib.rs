#![no_std]

use cheap_clone::CheapClone;
use hostaddr::{Domain, Host, HostAddr};

// Keep these checks in library code so cargo check on a bare-metal target
// validates the integration without a test harness or an upstream trait feature.
pub fn clone_domain<'a>(value: &Domain<&'a str>) -> Domain<&'a str> {
  value.cheap_clone()
}

pub fn clone_host<'a>(value: &Host<&'a str>) -> Host<&'a str> {
  value.cheap_clone()
}

pub fn clone_addr<'a>(value: &HostAddr<&'a str>) -> HostAddr<&'a str> {
  value.cheap_clone()
}

#[cfg(test)]
mod tests {
  extern crate std;

  use super::*;
  use std::{string::String, vec::Vec};

  static_assertions::assert_not_impl_any!(Domain<String>: CheapClone);
  static_assertions::assert_not_impl_any!(Host<String>: CheapClone);
  static_assertions::assert_not_impl_any!(HostAddr<Vec<u8>>: CheapClone);
  static_assertions::assert_impl_all!(HostAddr<hostaddr::Buffer>: CheapClone);
  static_assertions::assert_not_impl_any!(Domain<std::boxed::Box<str>>: CheapClone);

  #[test]
  fn stack_storage_and_derive_work_without_allocation() {
    #[derive(Clone, cheap_clone::CheapClone, Debug, PartialEq, Eq)]
    #[cheap_clone(inherent)]
    struct Endpoint {
      address: HostAddr<hostaddr::Buffer>,
    }

    let domain = Domain::<hostaddr::Buffer>::try_from("example.com.").unwrap();
    assert_eq!(domain.cheap_clone(), domain);
    assert_eq!(domain.as_inner().cheap_clone(), *domain.as_inner());
    let endpoint = Endpoint {
      address: HostAddr::from((domain, 443)),
    };
    assert_eq!(endpoint.cheap_clone(), endpoint);
  }

  #[test]
  fn borrowed_domains_and_ip_addresses() {
    let domain = Domain::try_from_ascii_str("example.com").unwrap().as_ref();
    let cloned = clone_domain(&domain);
    assert_eq!(domain, cloned);
    assert!(core::ptr::eq(*domain.as_inner(), *cloned.as_inner()));

    let host = Host::try_from_ascii_str("example.com").unwrap();
    assert_eq!(clone_host(&host), host);
    let addr = HostAddr::try_from_ascii_str("example.com:443").unwrap();
    assert_eq!(clone_addr(&addr), addr);
    assert_eq!(clone_addr(&addr).port(), Some(443));
    let ipv6 = HostAddr::try_from_ascii_str("[::1]:8080").unwrap();
    assert_eq!(clone_addr(&ipv6), ipv6);
  }

  #[cfg(any(feature = "alloc", feature = "std"))]
  #[test]
  fn shared_storage_keeps_the_allocation() {
    use std::sync::Arc;

    let domain = Domain::<Arc<str>>::try_from("example.com").unwrap();
    let cloned = domain.cheap_clone();
    assert!(Arc::ptr_eq(domain.as_inner(), cloned.as_inner()));

    let host = Host::Domain(domain);
    let cloned = host.cheap_clone();
    let (Host::Domain(original), Host::Domain(copy)) = (&host, &cloned) else {
      panic!("domain variant changed");
    };
    assert!(Arc::ptr_eq(original.as_inner(), copy.as_inner()));

    let addr = HostAddr::new(cloned).with_port(443);
    let cloned = addr.cheap_clone();
    assert_eq!(cloned, addr);
    assert_eq!(cloned.port(), Some(443));
    assert!(Arc::ptr_eq(
      &addr.unwrap_domain().0,
      &cloned.unwrap_domain().0
    ));
  }
}
