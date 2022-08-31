use serde::Deserialize;

use deserialize_custom_strings::custom_strings::{
  deserialize_credit_card_number, deserialize_email,
  deserialize_phone_number, deserialize_url,
};

#[derive(Deserialize, Debug, PartialEq)]
struct PhoneNumber(
  #[serde(deserialize_with = "deserialize_phone_number")] String,
);

#[derive(Deserialize, Debug, PartialEq)]
struct Email(#[serde(deserialize_with = "deserialize_email")] String);

#[derive(Deserialize, Debug, PartialEq)]
struct Urlencoded(
  #[serde(deserialize_with = "deserialize_url")] String,
);

#[derive(Deserialize, Debug, PartialEq)]
struct CreditCardNumber(
  #[serde(deserialize_with = "deserialize_credit_card_number")]
  String,
);

#[test]
fn phone_number1() {
  let phone_number = "+49 1571 231 2312".to_owned();

  let phone_number = serde_json::to_string(&phone_number).unwrap();

  let phone_number: PhoneNumber =
    serde_json::from_str(&phone_number).unwrap();

  assert_eq!(phone_number, PhoneNumber("+4915712312312".to_owned()));
}

#[test]
fn phone_number2() {
  let phone_number = "+49 175/3323-6724".to_owned();

  let phone_number = serde_json::to_string(&phone_number).unwrap();

  let phone_number: PhoneNumber =
    serde_json::from_str(&phone_number).unwrap();

  assert_eq!(phone_number, PhoneNumber("+4917533236724".to_owned()));
}

#[test]
fn phone_number3() {
  let phone_number = "++49175//3323+67  24".to_owned();

  let phone_number = serde_json::to_string(&phone_number).unwrap();

  let phone_number: PhoneNumber =
    serde_json::from_str(&phone_number).unwrap();

  assert_eq!(phone_number, PhoneNumber("+4917533236724".to_owned()));
}

#[test]
fn phone_number4() {
  let phone_number = "+49 221 345 46".to_owned();

  let phone_number = serde_json::to_string(&phone_number).unwrap();

  let phone_number: PhoneNumber =
    serde_json::from_str(&phone_number).unwrap();

  assert_eq!(phone_number, PhoneNumber("+4922134546".to_owned()));
}

/// Test cases taken from the validator crate:
/// https://github.com/Keats/validator/blob/44cc91749c675985468e59e126d76465fc675fb5/validator/src/validation/email.rs#L86
///
#[test]
fn valid_emails() {
  let valid_emails = vec![
    "email@here.com",
    "weirder-email@here.and.there.com",
    r#"!def!xyz%abc@example.com"#,
    "email@[127.0.0.1]",
    "email@[2001:dB8::1]",
    "email@[2001:dB8:0:0:0:0:0:1]",
    "email@[::fffF:127.0.0.1]",
    "example@valid-----hyphens.com",
    "example@valid-with-hyphens.com",
    "test@domain.with.idn.tld.उदाहरण.परीक्षा",
    "a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.atm",
    "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbb.atm",
    "abc@bar",
    "ABC@BAR",
    "       email@127.0.0.1     ",
    "\n\t  a@b.com\n",
    "\na@[127.0.0.1]\n",
  ];

  for email in valid_emails {
    let email = serde_json::to_string(email).unwrap();

    assert!(serde_json::from_str::<Email>(&email).is_ok());
  }
}

/// Test cases taken from the validator crate:
/// https://github.com/Keats/validator/blob/44cc91749c675985468e59e126d76465fc675fb5/validator/src/validation/email.rs#L86
///
#[test]
fn invalid_emails() {
  let invalid_emails = vec![
    r#""test@test"@example.com"#,
    "a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "",
    "abc",
    "abc@",
    "a @x.cz",
    "abc@.com",
    "something@@somewhere.com",
    "email@[127.0.0.256]",
    "email@[2001:db8::12345]",
    "email@[2001:db8:0:0:0:0:1]",
    "email@[::ffff:127.0.0.256]",
    "example@invalid-.com",
    "example@-invalid.com",
    "example@invalid.com-",
    "example@inv-.alid-.com",
    "example@inv-.-alid.com",
    r#"test@example.com\n\n<script src="x.js">"#,
    r#""\\\011"@here.com"#,
    r#""\\\012"@here.com"#,
    "trailingdot@shouldfail.com.",
    "a\n@b.com",
    r#""test@test"\n@example.com"#,
    "John.Doe@exam_ple.com",
  ];

  for email in invalid_emails {
    let email = serde_json::to_string(email).unwrap();

    assert!(serde_json::from_str::<Email>(&email).is_err());
  }
}

#[test]
fn urlencoded1() {
  let s = "https://something.de%2F".to_owned();

  let s = serde_json::to_string(&s).unwrap();

  let s: Urlencoded = serde_json::from_str(&s).unwrap();

  assert_eq!(s, Urlencoded("https://something.de/".to_owned()));
}

#[test]
fn urlencoded2() {
  let s = "http:/%2fsome-ü-halfway-ü-decent.com%2Fstring".to_owned();

  let s = serde_json::to_string(&s).unwrap();

  let s: Urlencoded = serde_json::from_str(&s).unwrap();

  assert_eq!(
    s,
    Urlencoded(
      "http://some-ü-halfway-ü-decent.com/string".to_owned()
    )
  );
}

#[test]
#[should_panic]
fn wrong_urlencoded1() {
  let s = "http://not-a-byte.com/%2G".to_owned();

  let s = serde_json::to_string(&s).unwrap();

  let _: Urlencoded = serde_json::from_str(&s).unwrap();
}

#[test]
#[should_panic]
fn wrong_urlencoded2() {
  let s = "http://only.half.a.byte%2".to_owned();

  let s = serde_json::to_string(&s).unwrap();

  let _: Urlencoded = serde_json::from_str(&s).unwrap();
}

/// Card numbers taken from:
/// https://www.paypalobjects.com/en_AU/vhelp/paypalmanager_help/credit_card_numbers.htm
///
#[test]
fn credit_card_numbers() {
  let cards = [
    "3782 8224 631 0005",
    "37144 96353 98431",
    "378734493671000      ",
    " 56105910\n81018250 ",
    "305 69309 025 904",
    "385 20000023237",
    "6011 11111 1111117",
    "6011000990139424",
    "3530111333300000",
    "35660020 20360505",
    "5555555555554444",
    "5105105105105100",
    "41111111\n\n\n11111111",
    "4012888888881881",
    "4222222222222",
    "5019717010103742",
  ];

  for card in cards {
    let card = serde_json::to_string(card).unwrap();

    assert!(serde_json::from_str::<CreditCardNumber>(&card).is_ok());
  }
}
