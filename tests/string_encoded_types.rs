use serde::Deserialize;

use deserialize_custom_strings::str_encoded::{
  deserialize_encoded_json, deserialize_encoded_yaml,
};

#[derive(Deserialize, Debug, PartialEq)]
struct OuterWithInnerJson {
  #[serde(deserialize_with = "deserialize_encoded_json")]
  inner: Inner,
}

#[derive(Deserialize, Debug, PartialEq)]
struct OuterWithInnerYaml {
  #[serde(deserialize_with = "deserialize_encoded_yaml")]
  inner: Inner,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Inner {
  bar: bool,
  baz: u8,
  bat: String,
}

#[test]
fn yaml_with_encoded_json() {
  let yaml = r#"
    inner: '{"bar": true, "baz": 255, "bat": "hello"}'
  "#;

  let expect = OuterWithInnerJson {
    inner: Inner {
      bar: true,
      baz: 255,
      bat: "hello".to_owned(),
    },
  };

  let actual: OuterWithInnerJson =
    serde_yaml::from_str(yaml).unwrap();

  assert_eq!(actual, expect);
}

#[test]
fn json_with_encoded_json() {
  let yaml = r#"{
    "inner": "{\"bar\": true, \"baz\": 255, \"bat\": \"hello\"}"
  }"#;

  let expect = OuterWithInnerJson {
    inner: Inner {
      bar: true,
      baz: 255,
      bat: "hello".to_owned(),
    },
  };

  let actual: OuterWithInnerJson =
    serde_json::from_str(yaml).unwrap();

  assert_eq!(actual, expect);
}

#[test]
fn yaml_with_encoded_yaml() {
  let yaml = r#"
    inner: >
      bar: true

      baz: 255

      bat: hello
  "#;

  let expect = OuterWithInnerYaml {
    inner: Inner {
      bar: true,
      baz: 255,
      bat: "hello".to_owned(),
    },
  };

  let actual: OuterWithInnerYaml =
    serde_yaml::from_str(yaml).unwrap();

  assert_eq!(actual, expect);
}

#[test]
fn json_with_encoded_yaml() {
  let yaml = r#"{
    "inner": "bar: true\nbaz: 255\nbat: hello"
  }"#;

  let expect = OuterWithInnerYaml {
    inner: Inner {
      bar: true,
      baz: 255,
      bat: "hello".to_owned(),
    },
  };

  let actual: OuterWithInnerYaml =
    serde_json::from_str(yaml).unwrap();

  assert_eq!(actual, expect);
}
