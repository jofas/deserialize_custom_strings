use serde::Deserialize;

use deserialize_custom_strings::convert::{
  deserialize_from, deserialize_from_option, deserialize_from_str,
  deserialize_from_str_option, deserialize_try_from,
  deserialize_try_from_option,
};

#[derive(Deserialize, Debug, PartialEq)]
struct Foo {
  #[serde(deserialize_with = "deserialize_from::<_, bool, _>")]
  bar: u8,
  #[serde(deserialize_with = "deserialize_try_from::<_, i8, _>")]
  baz: u8,
  #[serde(deserialize_with = "deserialize_from_str")]
  bat: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
struct FooOption {
  #[serde(
    deserialize_with = "deserialize_from_option::<_, bool, _>"
  )]
  bar: Option<u8>,
  #[serde(
    deserialize_with = "deserialize_try_from_option::<_, i8, _>"
  )]
  baz: Option<u8>,
  #[serde(deserialize_with = "deserialize_from_str_option")]
  bat: Option<i32>,
}

#[test]
fn foo1() {
  let json = r#"{
    "bar": true,
    "baz": 127,
    "bat": "-4"
  }"#;

  let expect = Foo {
    bar: 1,
    baz: 127,
    bat: -4,
  };

  let actual: Foo = serde_json::from_str(json).unwrap();

  assert_eq!(actual, expect);
}

#[test]
fn foo2() {
  let json = r#"{
    "bar": false,
    "baz": 0,
    "bat": "12"
  }"#;

  let expect = Foo {
    bar: 0,
    baz: 0,
    bat: 12,
  };

  let actual: Foo = serde_json::from_str(json).unwrap();

  assert_eq!(actual, expect);
}

#[test]
fn foo3() {
  let json = r#"{
    "bar": false,
    "baz": 0,
    "bat": "12"
  }"#;

  let expect = Foo {
    bar: 0,
    baz: 0,
    bat: 12,
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
    "bat": "-4"
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
    "bat": "-4"
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
    "bat": -4
  }"#;

  assert!(serde_json::from_str::<Foo>(json).is_err());
}

#[test]
fn foo_option1() {
  let json = r#"{
    "bar": true,
    "baz": 127,
    "bat": "-4"
  }"#;

  let expect = FooOption {
    bar: Some(1),
    baz: Some(127),
    bat: Some(-4),
  };

  let actual: FooOption = serde_json::from_str(json).unwrap();

  assert_eq!(actual, expect);
}

#[test]
fn foo_option2() {
  let json = r#"{
    "bar": false,
    "baz": null,
    "bat": "12"
  }"#;

  let expect = FooOption {
    bar: Some(0),
    baz: None,
    bat: Some(12),
  };

  let actual: FooOption = serde_json::from_str(json).unwrap();

  assert_eq!(actual, expect);
}

#[test]
fn foo_option3() {
  let json = r#"{
    "bar": null,
    "baz": null,
    "bat": null
  }"#;

  let expect = FooOption {
    bar: None,
    baz: None,
    bat: None,
  };

  let actual: FooOption = serde_json::from_str(json).unwrap();

  assert_eq!(actual, expect);
}
