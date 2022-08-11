use serde::de::Error;
use serde::Deserialize;

use regex::Regex;

lazy_static::lazy_static! {
  static ref RE_SANITIZE_PHONE_NUMBER: Regex = Regex::new(
    r"(?P<x>(^\+)|([0-9]))[^0-9]+",
  ).unwrap();
}

pub fn deserialize_phone_number<'de, D>(
  deserializer: D,
) -> Result<String, D::Error>
where
  D: serde::de::Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  let s = s.trim().to_lowercase();
  let s = RE_SANITIZE_PHONE_NUMBER.replace_all(&s, "$x");

  if validator::validate_phone(&*s) {
    Ok((*s).to_owned())
  } else {
    Err(Error::custom("ill formatted phone number"))
  }
}

pub fn deserialize_email<'de, D>(
  deserializer: D,
) -> Result<String, D::Error>
where
  D: serde::de::Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  let s = s.trim().to_lowercase();

  if validator::validate_email(&s) {
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
  let s = s.trim().to_lowercase();

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

  if validator::validate_url(&res) {
    Ok(res)
  } else {
    Err(Error::custom("ill formatted url"))
  }
}
