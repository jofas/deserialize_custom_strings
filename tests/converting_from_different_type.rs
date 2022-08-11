use serde::Deserialize;

use deserialize_custom_strings::convert::{
  deserialize_from, deserialize_from_option_str,
  deserialize_from_str, deserialize_try_from,
};

#[derive(Deserialize, Debug, PartialEq)]
struct Foo {
  #[serde(deserialize_with = "deserialize_from::<_, bool, _>")]
  bar: u8,
  #[serde(deserialize_with = "deserialize_try_from::<_, i8, _>")]
  baz: u8,
  #[serde(deserialize_with = "deserialize_from_str")]
  bat: i32,
  #[serde(deserialize_with = "deserialize_from_option_str")]
  qux: Option<bool>,
}

#[test]
fn foo1() {
  let json = r#"{
    "bar": true,
    "baz": 127,
    "bat": "-4",
    "qux": "true"
  }"#;

  let expect = Foo {
    bar: 1,
    baz: 127,
    bat: -4,
    qux: Some(true),
  };

  let actual: Foo = serde_json::from_str(json).unwrap();

  assert_eq!(actual, expect);
}

#[test]
fn foo2() {
  let json = r#"{
    "bar": false,
    "baz": 0,
    "bat": "12",
    "qux": "false"
  }"#;

  let expect = Foo {
    bar: 0,
    baz: 0,
    bat: 12,
    qux: Some(false),
  };

  let actual: Foo = serde_json::from_str(json).unwrap();

  assert_eq!(actual, expect);
}

#[test]
fn foo3() {
  let json = r#"{
    "bar": false,
    "baz": 0,
    "bat": "12",
    "qux": null
  }"#;

  let expect = Foo {
    bar: 0,
    baz: 0,
    bat: 12,
    qux: None,
  };

  let actual: Foo = serde_json::from_str(json).unwrap();

  assert_eq!(actual, expect);
}

/// `bar` field does not support to be parsed from [String].
///
#[test]
fn failing_bar() {
  let json = r#"{
    "bar": "true",
    "baz": 127,
    "bat": "-4",
    "qux": "true"
  }"#;

  assert!(serde_json::from_str::<Foo>(json).is_err());
}

/// [i8] overflows at `128`.
///
#[test]
fn failing_baz() {
  let json = r#"{
    "bar": true,
    "baz": 128,
    "bat": "-4",
    "qux": "true"
  }"#;

  assert!(serde_json::from_str::<Foo>(json).is_err());
}

/// `-4` is not a [String].
///
#[test]
fn failing_bat() {
  let json = r#"{
    "bar": true,
    "baz": 127,
    "bat": -4,
    "qux": "true"
  }"#;

  assert!(serde_json::from_str::<Foo>(json).is_err());
}

/// `0` is not a [bool].
///
#[test]
fn failing_qux() {
  let json = r#"{
    "bar": true,
    "baz": 127,
    "bat": "-4",
    "qux": "0"
  }"#;

  assert!(serde_json::from_str::<Foo>(json).is_err());
}
