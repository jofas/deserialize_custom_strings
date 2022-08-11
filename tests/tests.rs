use serde::{Deserialize, Serialize};

use deserialize_custom_strings::{
  deserialize_email, deserialize_phone_number, deserialize_url,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PhoneNumber(
  #[serde(deserialize_with = "deserialize_phone_number")] String,
);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Email(#[serde(deserialize_with = "deserialize_email")] String);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Urlencoded(
  #[serde(deserialize_with = "deserialize_url")] String,
);

#[test]
fn phone_number1() {
  let phone_number = PhoneNumber("+49 11 231 2312 3123".to_owned());

  let phone_number = serde_json::to_string(&phone_number).unwrap();

  let phone_number: PhoneNumber =
    serde_json::from_str(&phone_number).unwrap();

  assert_eq!(
    phone_number,
    PhoneNumber("+491123123123123".to_owned())
  );
}

#[test]
fn phone_number2() {
  let phone_number = PhoneNumber("+49 175/3323-6724".to_owned());

  let phone_number = serde_json::to_string(&phone_number).unwrap();

  let phone_number: PhoneNumber =
    serde_json::from_str(&phone_number).unwrap();

  assert_eq!(phone_number, PhoneNumber("+4917533236724".to_owned()));
}

#[test]
fn phone_number3() {
  let phone_number = PhoneNumber("0175/3323+67 24".to_owned());

  let phone_number = serde_json::to_string(&phone_number).unwrap();

  let phone_number: PhoneNumber =
    serde_json::from_str(&phone_number).unwrap();

  assert_eq!(phone_number, PhoneNumber("017533236724".to_owned()));
}

#[test]
fn phone_number4() {
  let phone_number = PhoneNumber("++49175//3323+67  24".to_owned());

  let phone_number = serde_json::to_string(&phone_number).unwrap();

  let phone_number: PhoneNumber =
    serde_json::from_str(&phone_number).unwrap();

  assert_eq!(phone_number, PhoneNumber("+4917533236724".to_owned()));
}

#[test]
fn email1() {
  let email = Email("Test@Test.De".to_owned());

  let email = serde_json::to_string(&email).unwrap();

  let email: Email = serde_json::from_str(&email).unwrap();

  assert_eq!(email, Email("test@test.de".to_owned()));
}

#[test]
fn urlencoded1() {
  let s = Urlencoded("something%2F".to_owned());

  let s = serde_json::to_string(&s).unwrap();

  let s: Urlencoded = serde_json::from_str(&s).unwrap();

  assert_eq!(s, Urlencoded("something/".to_owned()));
}

#[test]
fn urlencoded2() {
  let s = Urlencoded("some ü halfway ü decent %2F string".to_owned());

  let s = serde_json::to_string(&s).unwrap();

  let s: Urlencoded = serde_json::from_str(&s).unwrap();

  assert_eq!(
    s,
    Urlencoded("some ü halfway ü decent / string".to_owned())
  );
}

#[test]
#[should_panic]
fn wrong_urlencoded1() {
  let s = Urlencoded("no bytes %2G".to_owned());

  let s = serde_json::to_string(&s).unwrap();

  let _: Urlencoded = serde_json::from_str(&s).unwrap();
}

#[test]
#[should_panic]
fn wrong_urlencoded2() {
  let s = Urlencoded("only half a byte %2".to_owned());

  let s = serde_json::to_string(&s).unwrap();

  let _: Urlencoded = serde_json::from_str(&s).unwrap();
}
