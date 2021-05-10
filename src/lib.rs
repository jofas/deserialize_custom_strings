use serde::de::Error;
use serde::Deserialize;

use regex::Regex;

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
