use serde::{Deserialize, Serialize};

use deserialize_custom_strings::{
  deserialize_email, deserialize_phone_number,
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct PhoneNumber(
  #[serde(deserialize_with = "deserialize_phone_number")] String,
);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Email(#[serde(deserialize_with = "deserialize_email")] String);

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
fn email1() {
  let email = Email("Test@Test.De".to_owned());

  let email = serde_json::to_string(&email).unwrap();

  let email: Email = serde_json::from_str(&email).unwrap();

  assert_eq!(email, Email("test@test.de".to_owned()));
}
