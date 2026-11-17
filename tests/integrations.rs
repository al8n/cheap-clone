#[cfg(feature = "arcstr_1")]
#[test]
fn arcstr_clones_share_the_string_and_substring() {
  use cheap_clone::CheapClone;
  let text = arcstr_1::ArcStr::from("shared string".repeat(32));
  let cloned = text.cheap_clone();
  assert_eq!(text, cloned);
  assert_eq!(text.as_ptr(), cloned.as_ptr());
  let slice = text.substr(7..);
  let cloned = slice.cheap_clone();
  assert_eq!(slice, cloned);
  assert_eq!(slice.as_str().as_ptr(), cloned.as_str().as_ptr());
}

#[cfg(feature = "ecow_0_3")]
#[test]
fn ecow_shares_storage_until_mutation() {
  use cheap_clone::CheapClone;
  use ecow_0_3::{EcoBytes, EcoString, EcoVec};
  struct ExpensiveClone;
  impl Clone for ExpensiveClone {
    fn clone(&self) -> Self {
      panic!("EcoVec must share its elements");
    }
  }
  static_assertions::assert_impl_all!(EcoVec<ExpensiveClone>: CheapClone);
  let expensive: EcoVec<_> = vec![ExpensiveClone].into();
  assert_eq!(expensive.as_ptr(), expensive.cheap_clone().as_ptr());

  let values: EcoVec<_> = vec![String::from("original")].into();
  let mut cloned = values.cheap_clone();
  assert_eq!(values.as_ptr(), cloned.as_ptr());
  cloned.make_mut()[0].push_str(" changed");
  assert_eq!(values[0], "original");
  assert_eq!(cloned[0], "original changed");

  let payload = "shared string".repeat(32);
  let text = EcoString::from(payload.as_str());
  let cloned = text.cheap_clone();
  assert_eq!(text, cloned);
  assert_eq!(text.as_ptr(), cloned.as_ptr());
  let bytes = EcoBytes::from(payload.as_bytes());
  let cloned = bytes.cheap_clone();
  assert_eq!(bytes, cloned);
  assert_eq!(bytes.as_ptr(), cloned.as_ptr());
}

#[cfg(feature = "hipstr_0_8")]
mod hipstr {
  use cheap_clone::CheapClone;
  use hipstr_0_8::{HipByt, HipStr, LocalHipByt, LocalHipStr, Unique};
  use static_assertions::assert_not_impl_any;

  assert_not_impl_any!(hipstr_0_8::string::HipStr<'static, Unique>: CheapClone);
  assert_not_impl_any!(hipstr_0_8::bytes::HipByt<'static, Unique>: CheapClone);

  #[test]
  fn borrowed_inline_and_shared_representations() {
    let borrowed = HipStr::borrowed("borrowed");
    assert_eq!(borrowed.cheap_clone().as_ptr(), borrowed.as_ptr());
    let inline = HipStr::from("inline");
    assert_eq!(inline.cheap_clone(), inline);

    let payload = "shared string".repeat(32);
    let text = HipStr::from(payload.clone());
    let cloned = text.cheap_clone();
    assert_eq!(text, cloned);
    assert_eq!(text.as_ptr(), cloned.as_ptr());
    let local = LocalHipStr::from(payload.clone());
    let cloned = local.cheap_clone();
    assert_eq!(local.as_ptr(), cloned.as_ptr());
    let bytes = HipByt::from(payload.clone().into_bytes());
    let cloned = bytes.cheap_clone();
    assert_eq!(bytes.as_ptr(), cloned.as_ptr());
    let bytes = LocalHipByt::from(payload.into_bytes());
    let cloned = bytes.cheap_clone();
    assert_eq!(bytes.as_ptr(), cloned.as_ptr());
  }

  #[cfg(feature = "std")]
  #[test]
  fn os_strings_and_paths() {
    use hipstr_0_8::{HipOsStr, HipPath, LocalHipOsStr, LocalHipPath};
    static_assertions::assert_impl_all!(HipOsStr<'static>: CheapClone);
    static_assertions::assert_impl_all!(LocalHipOsStr<'static>: CheapClone);
    static_assertions::assert_impl_all!(LocalHipPath<'static>: CheapClone);
    assert_not_impl_any!(hipstr_0_8::os_string::HipOsStr<'static, Unique>: CheapClone);
    assert_not_impl_any!(hipstr_0_8::path::HipPath<'static, Unique>: CheapClone);
    let path = HipPath::from(std::path::PathBuf::from("long-path/".repeat(32)));
    assert_eq!(path.cheap_clone(), path);
  }
}

#[cfg(feature = "triomphe_0_1")]
#[test]
fn triomphe_handles_share_non_clone_payloads() {
  use cheap_clone::CheapClone;
  use triomphe_0_1::{Arc, ArcBorrow, ArcUnion, OffsetArc, ThinArc};
  struct NotClone;
  static_assertions::assert_impl_all!(Arc<[NotClone]>: CheapClone);
  static_assertions::assert_impl_all!(OffsetArc<[NotClone]>: CheapClone);

  let text: Arc<str> = Arc::from("shared");
  assert!(Arc::ptr_eq(&text, &text.cheap_clone()));
  let borrowed = text.borrow_arc();
  assert!(ArcBorrow::ptr_eq(&borrowed, &borrowed.cheap_clone()));
  let offset = Arc::into_raw_offset(text);
  assert!(core::ptr::eq(&*offset, &*offset.cheap_clone()));

  let thin = ThinArc::from_header_and_iter(NotClone, [NotClone].into_iter());
  assert_eq!(thin.as_ptr(), thin.cheap_clone().as_ptr());
  let union: ArcUnion<NotClone, NotClone> = ArcUnion::from_first(Arc::new(NotClone));
  assert!(ArcUnion::ptr_eq(&union, &union.cheap_clone()));
}

#[cfg(feature = "http_1")]
#[test]
fn http_shares_large_values_and_preserves_sensitive_metadata() {
  use cheap_clone::CheapClone;
  use http_1::{HeaderMap, HeaderValue, Method, Uri, header::HeaderName};
  static_assertions::assert_not_impl_any!(Method: CheapClone);
  static_assertions::assert_not_impl_any!(HeaderMap: CheapClone);

  let mut value = HeaderValue::from_bytes("header".repeat(64).as_bytes()).unwrap();
  value.set_sensitive(true);
  let cloned = value.cheap_clone();
  assert_eq!(value, cloned);
  assert!(cloned.is_sensitive());
  assert_eq!(value.as_bytes().as_ptr(), cloned.as_bytes().as_ptr());

  let name = HeaderName::from_bytes(format!("x-{}", "custom".repeat(32)).as_bytes()).unwrap();
  let cloned = name.cheap_clone();
  assert_eq!(name.as_str().as_ptr(), cloned.as_str().as_ptr());
  let uri: Uri = format!(
    "custom-scheme://example.com/{}?key=value",
    "path".repeat(64)
  )
  .parse()
  .unwrap();
  let cloned = uri.cheap_clone();
  assert_eq!(uri, cloned);
  assert_eq!(uri.path().as_ptr(), cloned.path().as_ptr());
  assert_eq!(uri.scheme().unwrap().cheap_clone(), *uri.scheme().unwrap());
  assert_eq!(
    uri.authority().unwrap().cheap_clone(),
    *uri.authority().unwrap()
  );
  assert_eq!(
    uri.path_and_query().unwrap().cheap_clone(),
    *uri.path_and_query().unwrap()
  );
}

#[cfg(feature = "uuid_1")]
#[test]
fn uuid_preserves_all_bits() {
  use cheap_clone::CheapClone;
  let value = uuid_1::Uuid::from_u128(u128::MAX - 42);
  assert_eq!(value.cheap_clone().as_u128(), value.as_u128());
}

#[cfg(feature = "rust_decimal_1")]
#[test]
fn decimal_preserves_scale_and_sign() {
  use cheap_clone::CheapClone;
  let value = rust_decimal_1::Decimal::new(-123400, 4);
  assert_eq!(value.cheap_clone().serialize(), value.serialize());
}

#[cfg(feature = "time_0_3")]
#[test]
fn time_preserves_offset_and_nanoseconds() {
  use cheap_clone::CheapClone;
  use time_0_3::{Date, Month, UtcOffset};
  let datetime = Date::from_calendar_date(2024, Month::February, 29)
    .unwrap()
    .with_hms_nano(12, 34, 56, 123456789)
    .unwrap();
  assert_eq!(datetime.cheap_clone(), datetime);
  let datetime = datetime.assume_offset(UtcOffset::from_hms(5, 30, 0).unwrap());
  let cloned = datetime.cheap_clone();
  assert_eq!(datetime, cloned);
  assert_eq!(datetime.offset(), cloned.offset());
  assert_eq!(datetime.nanosecond(), cloned.nanosecond());
}

#[cfg(feature = "chrono_0_4")]
#[test]
fn chrono_preserves_offset_and_nanoseconds() {
  use cheap_clone::CheapClone;
  let value =
    chrono_0_4::DateTime::parse_from_rfc3339("2024-02-29T12:34:56.123456789+05:30").unwrap();
  let cloned = value.cheap_clone();
  assert_eq!(value.naive_utc(), cloned.naive_utc());
  assert_eq!(value.offset(), cloned.offset());
}

#[cfg(feature = "chrono_0_4")]
#[test]
fn chrono_uses_the_offsets_cheap_clone_without_requiring_a_cheap_timezone() {
  use cheap_clone::CheapClone;
  use chrono_0_4::{DateTime, FixedOffset, MappedLocalTime, NaiveDate, NaiveDateTime, TimeZone};

  #[derive(Debug)]
  struct Offset(FixedOffset);
  impl Clone for Offset {
    fn clone(&self) -> Self {
      panic!("must call the offset's CheapClone");
    }
  }
  impl CheapClone for Offset {
    fn cheap_clone(&self) -> Self {
      Self(self.0)
    }
  }
  impl chrono_0_4::Offset for Offset {
    fn fix(&self) -> FixedOffset {
      self.0
    }
  }

  #[derive(Clone)]
  struct Zone;
  impl TimeZone for Zone {
    type Offset = Offset;
    fn from_offset(_: &Offset) -> Self {
      Self
    }
    fn offset_from_local_date(&self, _: &NaiveDate) -> MappedLocalTime<Offset> {
      unreachable!()
    }
    fn offset_from_local_datetime(&self, _: &NaiveDateTime) -> MappedLocalTime<Offset> {
      unreachable!()
    }
    fn offset_from_utc_date(&self, _: &NaiveDate) -> Offset {
      unreachable!()
    }
    fn offset_from_utc_datetime(&self, _: &NaiveDateTime) -> Offset {
      unreachable!()
    }
  }
  static_assertions::assert_not_impl_any!(Zone: CheapClone);
  let naive = NaiveDate::from_ymd_opt(2024, 2, 29)
    .unwrap()
    .and_hms_opt(1, 2, 3)
    .unwrap();
  let value: DateTime<Zone> =
    DateTime::from_naive_utc_and_offset(naive, Offset(FixedOffset::east_opt(19800).unwrap()));
  let cloned = value.cheap_clone();
  assert_eq!(cloned.naive_utc(), naive);
  assert_eq!(cloned.offset().0, value.offset().0);
}

#[cfg(feature = "jiff_1")]
#[test]
fn jiff_preserves_civil_time_and_fixed_time_zone_without_tzdb() {
  use cheap_clone::CheapClone;
  use jiff_1::{Span, Timestamp, civil::date, tz::Offset, tz::TimeZone};
  let civil = date(2024, 2, 29).at(12, 34, 56, 123456789);
  assert_eq!(civil.cheap_clone(), civil);
  let timezone = TimeZone::fixed(Offset::from_seconds(19800).unwrap());
  assert_eq!(timezone.cheap_clone(), timezone);
  let timestamp = Timestamp::from_second(1700000000).unwrap();
  assert_eq!(timestamp.cheap_clone(), timestamp);
  let zoned = timezone.to_zoned_datetime(civil).unwrap();
  let cloned = zoned.cheap_clone();
  assert_eq!(zoned.timestamp(), cloned.timestamp());
  assert_eq!(zoned.time_zone(), cloned.time_zone());
  let span = Span::new().months(3).days(2).nanoseconds(5);
  assert_eq!(span.cheap_clone().fieldwise(), span.fieldwise());
}
