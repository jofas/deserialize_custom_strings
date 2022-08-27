use serde::de::Error;
use serde::Deserialize;

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

pub fn deserialize_from_option<'de, D, S, T>(
  deserializer: D,
) -> Result<Option<T>, D::Error>
where
  D: serde::de::Deserializer<'de>,
  S: Deserialize<'de>,
  T: From<S>,
{
  let s: Option<S> = Option::deserialize(deserializer)?;
  Ok(s.map(|s| T::from(s)))
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

pub fn deserialize_try_from_option<'de, D, S, T>(
  deserializer: D,
) -> Result<Option<T>, D::Error>
where
  D: serde::de::Deserializer<'de>,
  S: Deserialize<'de>,
  T: std::convert::TryFrom<S>,
{
  let s: Option<S> = Option::deserialize(deserializer)?;

  Ok(match s {
    Some(s) => Some(
      <T as std::convert::TryFrom<S>>::try_from(s).map_err(|_| {
        Error::custom(
          "failed to parse deserialized value to desired type",
        )
      })?,
    ),
    None => None,
  })
}

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

pub fn deserialize_from_str_option<'de, D, T>(
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
