# deserialize_custom_strings

[![Build Status](https://github.com/jofas/deserialize_custom_strings/actions/workflows/build.yml/badge.svg)](https://github.com/jofas/deserialize_custom_strings/actions/workflows/build.yml)
[![Codecov](https://codecov.io/gh/jofas/deserialize_custom_strings/branch/master/graph/badge.svg?token=69YKZ1JIBK)](https://codecov.io/gh/jofas/deserialize_custom_strings)
[![Latest Version](https://img.shields.io/crates/v/deserialize_custom_strings.svg)](https://crates.io/crates/deserialize_custom_strings)
[![Downloads](https://img.shields.io/crates/d/deserialize_custom_strings?label=downloads)](https://crates.io/crates/deserialize_custom_strings)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/deserialize_custom_strings/latest/deserialize_custom_strings)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)


Utility functions for deserializing fields from different types or as 
custom strings (e.g. emails, urls or phone numbers) using 
[serde](https://serde.rs/).


## Table of Contents

<!--ts-->
   * [Deserializing fields by converting from different type](#deserializing-fields-by-converting-from-different-type)
      * [From](#from)
      * [TryFrom](#tryfrom)
      * [FromStr](#fromstr)
   * [Custom string deserializers](#custom-string-deserializers)
      * [Emails](#emails)
      * [Phone numbers](#phone-numbers)
      * [Urls](#urls)
<!--te-->


## Deserializing fields by converting from different type

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


### From

TODO: mention it as a an equivalent to container attribute `from` for
fields


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

```rust
use serde::Deserialize;

use deserialize_custom_strings::convert::deserialize_from_option_str;

#[derive(Deserialize, Debug)]
struct Foo {
  #[serde(deserialize_with = "deserialize_from_option_str")]
  bar: Option<u8>,
}

let json = r#"{
  "bar": null
}"#;

let foo: Foo = serde_json::from_str(json).unwrap();

assert_eq!(foo.bar, None);


let json = r#"{
  "bar": "255"
}"#;

let foo: Foo = serde_json::from_str(json).unwrap();

assert_eq!(foo.bar, Some(255));
```

## Custom string deserializers


### Emails


### Phone numbers


### Urls
