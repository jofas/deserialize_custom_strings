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
