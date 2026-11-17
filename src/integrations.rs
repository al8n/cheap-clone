#[allow(unused_imports)]
use crate::CheapClone;

#[cfg(feature = "cower_0_1")]
const _: () = {
  use cower_0_1::{Cow, CowRepr, ToOwned};

  // Cow delegates to its actual storage, which need not be CowRepr.
  impl<'a, T: ?Sized + ToOwned<P>, P: ?Sized> CheapClone for Cow<'a, T, P>
  where
    T::Storage<'a>: CheapClone,
  {
    fn cheap_clone(&self) -> Self {
      Self::from_storage(self.as_storage().cheap_clone())
    }
  }

  impl<T: ?Sized + ToOwned<P>, P: ?Sized> CheapClone for CowRepr<'_, T, P>
  where
    T::Owned: CheapClone,
  {
    fn cheap_clone(&self) -> Self {
      match self {
        Self::Borrowed(value) => Self::Borrowed(value),
        Self::Owned(value) => Self::Owned(value.cheap_clone()),
      }
    }
  }
};

#[cfg(feature = "hostaddr_1")]
const _: () = {
  // Clone preserves the validated domain representation without reparsing it.
  impl<S: CheapClone> CheapClone for hostaddr_1::Domain<S> {}
  impl<S: CheapClone> CheapClone for hostaddr_1::Host<S> {}
  impl<S: CheapClone> CheapClone for hostaddr_1::HostAddr<S> {}
  impl_cheap_clone_for_copy!(hostaddr_1::Buffer);
};

#[cfg(feature = "emailaddr_1")]
const _: () = {
  use emailaddr_1::{DomainPart, EmailAddr, LocalPart};

  // Upstream's Clone preserves the validated representation without parsing
  // it again. P: Clone comes from its derive; the policy is only PhantomData.
  impl<S: CheapClone, P: Clone> CheapClone for EmailAddr<S, P> {}
  impl<S: CheapClone> CheapClone for LocalPart<S> {}
  impl<S: CheapClone> CheapClone for DomainPart<S> {}

  #[cfg(any(feature = "alloc", feature = "std"))]
  impl<S: CheapClone, P: Clone> CheapClone for emailaddr_1::ParseEmailAddrOutput<S, P> {}

  impl CheapClone for emailaddr_1::ParseEmailAddrError {}

  impl_cheap_clone_for_copy!(
    emailaddr_1::Buffer,
    emailaddr_1::Strict,
    emailaddr_1::Relax,
    emailaddr_1::Options,
    emailaddr_1::LocalOptions,
    emailaddr_1::DomainOptions,
    emailaddr_1::Limits,
    emailaddr_1::SmtpUtf8Policy,
    emailaddr_1::DomainLiteralPolicy,
    emailaddr_1::DomainUnicodePolicy,
    emailaddr_1::ParseLocalPartError,
    emailaddr_1::ParseDomainPartError,
  );
};

#[cfg(feature = "arcstr_1")]
const _: () = {
  impl CheapClone for arcstr_1::ArcStr {}
  impl CheapClone for arcstr_1::Substr {}
};

#[cfg(feature = "ecow_0_3")]
const _: () = {
  impl<T: Clone> CheapClone for ecow_0_3::EcoVec<T> {}
  impl CheapClone for ecow_0_3::EcoString {}
  impl CheapClone for ecow_0_3::EcoBytes {}
};

#[cfg(feature = "uuid_1")]
impl_cheap_clone_for_copy!(uuid_1::Uuid);

#[cfg(feature = "rust_decimal_1")]
impl_cheap_clone_for_copy!(rust_decimal_1::Decimal);

// Like [T; N], these copy a statically bounded amount of inline storage.
// Copy the values directly instead of invoking a potentially custom T::clone.
#[cfg(feature = "hybrid-array_0_4")]
impl<T: Copy, N: hybrid_array_0_4::ArraySize> CheapClone for hybrid_array_0_4::Array<T, N>
where
  N::ArrayType<T>: Copy,
{
  fn cheap_clone(&self) -> Self {
    *self
  }
}

#[cfg(feature = "generic-array_1")]
impl<T: Copy, N: generic_array_1::ArrayLength> CheapClone for generic_array_1::GenericArray<T, N>
where
  N::ArrayType<T>: Copy,
{
  fn cheap_clone(&self) -> Self {
    *self
  }
}

#[cfg(feature = "hybrid-arraydeque_0_1")]
impl<T: Copy, N: hybrid_arraydeque_0_1::ArraySize> CheapClone
  for hybrid_arraydeque_0_1::ArrayDeque<T, N>
{
  fn cheap_clone(&self) -> Self {
    let mut cloned = Self::new();
    for &value in self.iter() {
      // Both deques have the same capacity, so every element fits.
      let overflow = cloned.push_back(value);
      debug_assert!(overflow.is_none());
    }
    cloned
  }
}

#[cfg(feature = "generic-arraydeque_0_2")]
impl<T: Copy, N: generic_arraydeque_0_2::ArrayLength> CheapClone
  for generic_arraydeque_0_2::GenericArrayDeque<T, N>
{
  fn cheap_clone(&self) -> Self {
    let mut cloned = Self::new();
    for &value in self.iter() {
      let overflow = cloned.push_back(value);
      debug_assert!(overflow.is_none());
    }
    cloned
  }
}

#[cfg(feature = "heapless_0_9")]
const _: () = {
  use heapless_0_9::{Deque, LenType, String, Vec};

  impl<T: Copy, const N: usize, LenT: LenType> CheapClone for Vec<T, N, LenT> {
    fn cheap_clone(&self) -> Self {
      self.iter().copied().collect()
    }
  }

  impl<T: Copy, const N: usize> CheapClone for Deque<T, N> {
    fn cheap_clone(&self) -> Self {
      let mut cloned = Self::new();
      cloned.extend(self.iter().copied());
      cloned
    }
  }

  impl<const N: usize, LenT: LenType> CheapClone for String<N, LenT> {}
};

#[cfg(feature = "hipstr_0_8")]
const _: () = {
  // Only reference-counted backends: Unique deep-copies allocated contents.
  // Arc/Rc can also deep-copy on reference-count overflow; see the README.
  impl CheapClone for hipstr_0_8::HipStr<'_> {}
  impl CheapClone for hipstr_0_8::HipByt<'_> {}
  impl CheapClone for hipstr_0_8::LocalHipStr<'_> {}
  impl CheapClone for hipstr_0_8::LocalHipByt<'_> {}

  #[cfg(feature = "std")]
  const _: () = {
    impl CheapClone for hipstr_0_8::HipOsStr<'_> {}
    impl CheapClone for hipstr_0_8::HipPath<'_> {}
    impl CheapClone for hipstr_0_8::LocalHipOsStr<'_> {}
    impl CheapClone for hipstr_0_8::LocalHipPath<'_> {}
  };
};

#[cfg(feature = "http_1")]
const _: () = {
  use http_1::{StatusCode, Uri, Version, header, uri};

  impl CheapClone for header::HeaderName {}
  impl CheapClone for header::HeaderValue {}
  impl CheapClone for uri::Authority {}
  impl CheapClone for uri::PathAndQuery {}
  // Custom schemes clone a fixed-size Box<ByteStr>, sharing the string bytes.
  // These remain O(1), but are not necessarily allocation-free.
  impl CheapClone for uri::Scheme {}
  impl CheapClone for Uri {}
  impl_cheap_clone_for_copy!(StatusCode, Version);
};

#[cfg(feature = "time_0_3")]
impl_cheap_clone_for_copy!(
  time_0_3::Date,
  time_0_3::Time,
  time_0_3::PlainDateTime,
  time_0_3::OffsetDateTime,
  time_0_3::UtcDateTime,
  time_0_3::Timestamp,
  time_0_3::SignedDuration,
  time_0_3::UtcOffset,
  time_0_3::Month,
  time_0_3::Weekday,
);

#[cfg(feature = "jiff_1")]
const _: () = {
  impl_cheap_clone_for_copy!(
    jiff_1::civil::Date,
    jiff_1::civil::Time,
    jiff_1::civil::DateTime,
    jiff_1::civil::Weekday,
    jiff_1::Timestamp,
    jiff_1::SignedDuration,
    jiff_1::Span,
    jiff_1::tz::Offset,
    jiff_1::Unit,
    jiff_1::RoundMode,
  );

  // TimeZone shares transition rules when alloc is enabled and stores only
  // small fixed data otherwise. ZonedDateTime adds fixed-size date/time data.
  impl CheapClone for jiff_1::tz::TimeZone {}
  impl CheapClone for jiff_1::ZonedDateTime {}
};
