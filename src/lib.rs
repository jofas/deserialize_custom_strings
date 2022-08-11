use serde::de::Error;
use serde::Deserialize;

use regex::Regex;

lazy_static::lazy_static! {
  static ref RE_EMAIL: Regex = Regex::new(
    r"^(?P<user>[a-z0-9.!#$%&'*+/=?^_`{|}~-]+)@(?P<domain>[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?(\.[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?)*)$",
  ).unwrap();
}

/// In case you encounter a JSON API that does not return the `u64`
/// as a number like you handsome and smart person deserve, but
/// instead wraps it in a `String`, like:
///
/// ```rust
/// use serde::Deserialize;
/// use serde_json::{Error, from_str};
///
/// #[derive(Deserialize)]
/// struct Foo {
///  definetly_a_u64: u64,
/// }
///
/// let json = r#"{
///   "definetly_a_u64": "0123456789"
/// }"#;
///
/// let foo: Result<Foo, Error> = from_str(json);
///
/// assert!(foo.is_err());
/// ```
///
/// Make the above code working with:
///
/// ```rust
/// use serde::Deserialize;
/// use serde_json::{Error, from_str};
///
/// use deserialize_custom_strings::deserialize_from;
///
/// #[derive(Deserialize)]
/// struct Foo {
///  #[serde(deserialize_with = "deserialize_from::<_, bool, _>")]
///  definetly_a_u64: u64,
/// }
///
/// let json = r#"{
///   "definetly_a_u64": true
/// }"#;
///
/// let foo: Result<Foo, Error> = from_str(json);
///
/// assert!(foo.is_ok());
/// assert_eq!(foo.unwrap().definetly_a_u64, 1);
/// ```
///
pub fn deserialize_from<'de, D, S, T>(
  deserializer: D,
) -> Result<T, D::Error>
where
  D: serde::de::Deserializer<'de>,
  S: Deserialize<'de>,
  T: From<S>,
{
  let s = S::deserialize(deserializer)?;
  Ok(T::from(s))
}

pub fn deserialize_try_from<'de, D, S, T>(
  deserializer: D,
) -> Result<T, D::Error>
where
  D: serde::de::Deserializer<'de>,
  S: Deserialize<'de>,
  T: std::convert::TryFrom<S>,
{
  let s = S::deserialize(deserializer)?;

  <T as std::convert::TryFrom<S>>::try_from(s).map_err(|_| {
    Error::custom(
      "failed to parse deserialized value to desired type",
    )
  })
}

/// In case you encounter a JSON API that does not return the `u64`
/// as a number like you handsome and smart person deserve, but
/// instead wraps it in a `String`, like:
///
/// ```rust
/// use serde::Deserialize;
/// use serde_json::{Error, from_str};
///
/// #[derive(Deserialize)]
/// struct Foo {
///  definetly_a_u64: u64,
/// }
///
/// let json = r#"{
///   "definetly_a_u64": "0123456789"
/// }"#;
///
/// let foo: Result<Foo, Error> = from_str(json);
///
/// assert!(foo.is_err());
/// ```
///
/// Make the above code working with:
///
/// ```rust
/// use serde::Deserialize;
/// use serde_json::{Error, from_str};
///
/// use deserialize_custom_strings::deserialize_from_str;
///
/// #[derive(Deserialize)]
/// struct Foo {
///  #[serde(deserialize_with = "deserialize_from_str")]
///  definetly_a_u64: u64,
/// }
///
/// let json = r#"{
///   "definetly_a_u64": "0123456789"
/// }"#;
///
/// let foo: Result<Foo, Error> = from_str(json);
///
/// assert!(foo.is_ok());
/// assert_eq!(foo.unwrap().definetly_a_u64, 123456789);
/// ```
///
pub fn deserialize_from_str<'de, D, T>(
  deserializer: D,
) -> Result<T, D::Error>
where
  D: serde::de::Deserializer<'de>,
  T: std::str::FromStr,
{
  let s = String::deserialize(deserializer)?;

  s.parse().map_err(|_| {
    Error::custom(
      "failed to parse deserialized value to desired type",
    )
  })
}

pub fn deserialize_from_option_str<'de, D, T>(
  deserializer: D,
) -> Result<Option<T>, D::Error>
where
  D: serde::de::Deserializer<'de>,
  T: std::str::FromStr,
{
  let s: Option<String> = Option::deserialize(deserializer)?;

  Ok(match s {
    Some(s) => Some(s.parse().map_err(|_| {
      Error::custom(
        "failed to parse deserialized value to desired type",
      )
    })?),
    None => None,
  })
}

pub fn deserialize_phone_number<'de, D>(
  deserializer: D,
) -> Result<String, D::Error>
where
  D: serde::de::Deserializer<'de>,
{
  let r = Regex::new(r"(?P<x>(^\+)|([0-9]))[^0-9]+")
    .map_err(Error::custom)?;

  let s = String::deserialize(deserializer)?;

  Ok((*r.replace_all(&s, "$x")).to_owned())
}

pub fn deserialize_email<'de, D>(
  deserializer: D,
) -> Result<String, D::Error>
where
  D: serde::de::Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  let s = s.trim().to_lowercase();

  if RE_EMAIL.is_match(&s) {
    Ok(s)
  } else {
    Err(Error::custom("ill formatted e-mail address"))
  }
}

pub fn deserialize_url<'de, D>(
  deserializer: D,
) -> Result<String, D::Error>
where
  D: serde::de::Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;

  let mut res = String::new();
  let mut iter = s.chars();

  while let Some(c) = iter.next() {
    if c == '%' {
      let left = iter.next();
      let right = iter.next();
      match (left, right) {
        (Some(l), Some(r)) => {
          let byte = u8::from_str_radix(&format!("{}{}", l, r), 16)
            .map_err(Error::custom)?;
          res += &(byte as char).to_string();
        }
        _ => return Err("index error").map_err(Error::custom),
      }
    } else {
      res += &c.to_string();
    }
  }

  Ok(res)
}
