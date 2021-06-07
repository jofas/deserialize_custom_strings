use serde::de::Error;
use serde::Deserialize;

use regex::Regex;

/// In case you encounter a JSON API that does not return the `u64`
/// as a number like you handsome and smart person deserve, but
/// instead wraps it in a `String`, like:
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
/// ```rust
/// use serde::Deserialize;
/// use serde_json::{Error, from_str};
///
/// use deserialize_custom_strings::deserialize_u64;
///
/// #[derive(Deserialize)]
/// struct Foo {
///  #[serde(deserialize_with = "deserialize_u64")]
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
pub fn deserialize_u64<'de, D>(
  deserializer: D,
) -> Result<u64, D::Error>
where
  D: serde::de::Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  s.parse().map_err(Error::custom)
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
  Ok(s.to_lowercase())
}

pub fn deserialize_urlencoded<'de, D>(
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
