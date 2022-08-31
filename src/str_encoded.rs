use serde::de::Error;
use serde::Deserialize;

pub fn deserialize_encoded_json<'de, D, T>(
  deserializer: D,
) -> Result<T, D::Error>
where
  D: serde::de::Deserializer<'de>,
  T: serde::de::DeserializeOwned,
{
  let s = String::deserialize(deserializer)?;
  serde_json::from_str(&s)
    .map_err(|_| Error::custom("failed parsing string encoded json"))
}

pub fn deserialize_encoded_yaml<'de, D, T>(
  deserializer: D,
) -> Result<T, D::Error>
where
  D: serde::de::Deserializer<'de>,
  T: serde::de::DeserializeOwned,
{
  let s = String::deserialize(deserializer)?;
  serde_yaml::from_str(&s)
    .map_err(|_| Error::custom("failed parsing string encoded yaml"))
}
