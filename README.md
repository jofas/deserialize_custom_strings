# deserialize_custom_strings

[![Build Status](https://github.com/jofas/deserialize_custom_strings/actions/workflows/build.yml/badge.svg)](https://github.com/jofas/deserialize_custom_strings/actions/workflows/build.yml)
[![Codecov](https://codecov.io/gh/jofas/deserialize_custom_strings/branch/master/graph/badge.svg?token=69YKZ1JIBK)](https://codecov.io/gh/jofas/deserialize_custom_strings)
[![Latest Version](https://img.shields.io/crates/v/deserialize_custom_strings.svg)](https://crates.io/crates/deserialize_custom_strings)
[![Downloads](https://img.shields.io/crates/d/deserialize_custom_strings?label=downloads)](https://crates.io/crates/deserialize_custom_strings)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/deserialize_custom_strings/latest/deserialize_custom_strings)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)


Utility functions for deserializing fields from different types or as 
custom strings (e.g. emails, urls or phone numbers) using 
[serde's](https://serde.rs/) `deserialize_with` field attribute.
This crate's aim is to fortify your application against badly 
formatted user input and helps you integrate unintuitive and complex 
APIs without you having to write any logic.


## Table of Contents

<!--ts-->
   * [Deserializing fields by converting from different type](#deserializing-fields-by-converting-from-different-type)
      * [From](#from)
      * [TryFrom](#tryfrom)
      * [FromStr](#fromstr)
      * [Embedded json strings](#embedded-json-strings)
      * [Optional fields](#optional-fields)
         * [Omitted fields](#omitted-fields)
   * [Custom string deserializers](#custom-string-deserializers)
      * [Emails](#emails)
      * [Phone numbers](#phone-numbers)
      * [Urls](#urls)
<!--te-->


## Deserializing fields by converting from different type

TODO: general description of what capabilities serde already offers
and what this crates does to enhance these capabilities. Give
real world example (u64 from string)


### From

TODO: mention it as a an equivalent to container attribute `from` for
fields

```rust
use serde::Deserialize;

use deserialize_custom_strings::convert::deserialize_from;

#[derive(Deserialize, Debug)]
struct Foo {
  #[serde(deserialize_with = "deserialize_from::<_, bool, _>")]
  bar: u8,
}

let json = r#"{
  "bar": true
}"#;

let foo: Foo = serde_json::from_str(json).unwrap();

assert_eq!(foo.bar, 1);
```


### TryFrom

```rust
use serde::Deserialize;

use deserialize_custom_strings::convert::deserialize_try_from;

#[derive(Deserialize, Debug)]
struct Foo {
  #[serde(deserialize_with = "deserialize_try_from::<_, i8, _>")]
  bar: u8,
}

// This will parse `bar` as i8 (ranging from -128 to 127) before
// trying to convert the i8 value to u8 with the `TryFrom` trait.
// If the i8 is out of u8 range (0 to 255), parsing will fail.

let json = r#"{
  "bar": 127
}"#;

let foo: Foo = serde_json::from_str(json).unwrap();

assert_eq!(foo.bar, 127);

let json = r#"{
  "bar": 127
}"#;

let foo: Foo = serde_json::from_str(json).unwrap();

assert_eq!(foo.bar, 127);


// Fails, because 128 is not a value i8 covers

let json = r#"{
  "bar": 128
}"#;

assert!(serde_json::from_str::<Foo>(json).is_err());


// Fails, because -1 is not convertible from i8 to u8

let json = r#"{
  "bar": -1
}"#;

assert!(serde_json::from_str::<Foo>(json).is_err());
```


### FromStr

```rust
use serde::Deserialize;

use deserialize_custom_strings::convert::deserialize_from_str;

#[derive(Deserialize, Debug)]
struct Foo {
  #[serde(deserialize_with = "deserialize_from_str")]
  bar: u8,
}

let json = r#"{
  "bar": "255"
}"#;

let foo: Foo = serde_json::from_str(json).unwrap();

assert_eq!(foo.bar, 255);
```

### Embedded json strings


### Optional fields

TODO: link to 
[tracking issue](https://github.com/rust-lang/rust/issues/31844) for
specialization and describe why this make Option weak concerning the
implementation of conversion traits `From`, `TryFrom` and `FromStr`.

```rust
use serde::Deserialize;

use deserialize_custom_strings::convert::{
  deserialize_from_option, 
  deserialize_try_from_option,
  deserialize_from_str_option,
};

#[derive(Deserialize, Debug)]
struct Foo {
  #[serde(deserialize_with = "deserialize_from_option::<_, bool, _>")]
  bar: Option<u8>,
  #[serde(deserialize_with = "deserialize_try_from_option::<_, i8, _>")]
  baz: Option<u8>,
  #[serde(deserialize_with = "deserialize_from_str_option")]
  bat: Option<u8>,
}

let json = r#"{
  "bar": null,
  "baz": null,
  "bat": null
}"#;

let foo: Foo = serde_json::from_str(json).unwrap();

assert_eq!(foo.bar, None);
assert_eq!(foo.baz, None);
assert_eq!(foo.bat, None);


let json = r#"{
  "bar": true,
  "baz": 127,
  "bat": "255"
}"#;

let foo: Foo = serde_json::from_str(json).unwrap();

assert_eq!(foo.bar, Some(1));
assert_eq!(foo.baz, Some(127));
assert_eq!(foo.bat, Some(255));
```

#### Omitted fields

When deserializing json and you're using serde's `deserialize_with`
field attribute, optional fields can't be omitted from the json 
object. 
If an optional field is omitted from the json object, deserialization
will fail:

```rust
use serde::Deserialize;

use deserialize_custom_strings::convert::deserialize_from_option;

#[derive(Deserialize, Debug)]
struct Foo {
  #[serde(deserialize_with = "deserialize_from_option::<_, bool, _>")]
  bar: Option<u8>,
}

// Works, because the "bar" field is explicitly given

let json = r#"{
  "bar": null
}"#;

let foo: Foo = serde_json::from_str(json).unwrap();

assert_eq!(foo.bar, None);


// Does not work, because "bar" field is omitted from the json
// object. If the `deserialize_with` field attribute wouldn't have
// been used, deserialization would work, but serde/serde_json is not
// able to handle omitted fields when `deserialize_with` is used

let json = "{}";

let err = serde_json::from_str::<Foo>(json);

assert!(err.is_err());
```

This [issue](https://github.com/serde-rs/serde/issues/2249) tracks 
serde's support for omitted fields when the `deserialize_with` field
attribute is used.


## Custom string deserializers


### Emails


### Phone numbers


### Urls
