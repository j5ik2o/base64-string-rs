//! # About Base64String
//!
//! Base64String is a string type in Base64 format that contains meta-information about the encoding.
//!
//! # Usage
//!
//! ```rust
//! use base64_string_rs::Base64StringFactory;
//!
//! let str = "0123ABC";
//! let factory = Base64StringFactory::default();
//! let encoded = factory.encode_from_string(str);
//! println!("encoded = {}", encoded);
//! // encoded = Base64String(value = MDEyM0FCQw, url_safe = false, padding = false)
//! let decoded = encoded.decode_to_string().unwrap();
//! println!("decoded = {}", decoded); // 0123ABC
//! # assert_eq!(decoded, str);
//! ```
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::fmt;
use std::fmt::{Debug, Formatter};

use anyhow::Result;
use base64::{CharacterSet, Config};
use num_bigint::{BigInt, Sign};

#[derive(Debug, Clone)]
pub enum Endian {
  LE,
  BE,
}

#[derive(Debug, Clone)]
pub struct Base64StringFactory {
  url_safe: bool,
  padding: bool,
}

impl Default for Base64StringFactory {
  fn default() -> Self {
    Self::new(false, false)
  }
}

impl Base64StringFactory {
  pub fn new(url_safe: bool, padding: bool) -> Self {
    Self { url_safe, padding }
  }

  #[inline]
  fn encode<T: AsRef<[u8]>>(self, input: T) -> Base64String {
    let cs = if self.url_safe {
      CharacterSet::UrlSafe
    } else {
      CharacterSet::Standard
    };
    let config = Config::new(cs, self.padding);
    let result = base64::encode_config(input, config);
    Base64String::new(result, self.url_safe, self.padding)
  }

  pub fn encode_from_string(self, input: &str) -> Base64String {
    self.encode(input)
  }

  pub fn encode_from_bytes(self, input: &[u8]) -> Base64String {
    self.encode(input)
  }

  pub fn encode_with_endian_from_bigint(self, input: &BigInt, endian: Endian) -> Base64String {
    let (_, bytes) = match endian {
      Endian::BE => input.to_bytes_be(),
      Endian::LE => input.to_bytes_le(),
    };
    self.encode_from_bytes(&bytes)
  }
}

#[derive(Debug, Clone)]
pub struct Base64String {
  value: String,
  url_safe: bool,
  padding: bool,
}

#[derive(Debug, Clone)]
pub enum Base64DecodeError {
  DecodeError(String),
}

impl fmt::Display for Base64String {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Base64String(value = {}, url_safe = {}, padding = {})",
      self.value, self.url_safe, self.padding
    )
  }
}

impl Base64String {
  pub fn new(value: String, url_safe: bool, padding: bool) -> Self {
    Self {
      value,
      url_safe,
      padding,
    }
  }

  pub const fn is_url_safe(&self) -> bool {
    self.url_safe
  }

  pub const fn is_padding(&self) -> bool {
    self.padding
  }

  pub fn len(&self) -> usize {
    self.value.len()
  }

  pub fn to_value(&self) -> &str {
    &self.value
  }

  pub fn decode_to_bytes(self) -> Result<Vec<u8>> {
    let cs = if self.url_safe {
      CharacterSet::UrlSafe
    } else {
      CharacterSet::Standard
    };
    let config = Config::new(cs, self.padding);
    match base64::decode_config(self.value, config) {
      Ok(r) => Ok(r),
      Err(r) => Err(r)?,
    }
  }

  pub fn decode_to_string(self) -> Result<String> {
    let v = self.decode_to_bytes()?;
    match String::from_utf8(v) {
      Ok(r) => Ok(r),
      Err(r) => Err(r.utf8_error())?,
    }
  }

  pub fn decode_with_endian_to_bigint(self, endian: Endian) -> Result<BigInt> {
    let bytes = self.clone().decode_to_bytes()?;
    let bigint = match endian {
      Endian::BE => BigInt::from_bytes_be(Sign::Plus, &bytes),
      Endian::LE => BigInt::from_bytes_le(Sign::Plus, &bytes),
    };
    Ok(bigint)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let str = "0123ABC";
    let factory = Base64StringFactory::default();
    let encoded = factory.encode_from_string(str);
    println!("encoded = {}", encoded);
    let decoded = encoded.decode_to_string().unwrap();
    println!("decoded = {}", decoded);
    assert_eq!(decoded, str);
  }

  #[quickcheck]
  fn encode_decode_string(s: String) {
    let factory = Base64StringFactory::new(false, false);
    let encoded = factory.encode_from_string(&s);
    println!("{}", encoded);
    let decoded = encoded.decode_to_string().ok().unwrap();
    assert_eq!(decoded, s);
  }

  #[quickcheck]
  fn encode_decode_bytes(s: Vec<u8>) {
    let factory = Base64StringFactory::new(false, false);
    let encoded = factory.encode_from_bytes(&s);
    let decoded = encoded.decode_to_bytes().ok().unwrap();
    assert_eq!(decoded, s);
  }

  #[quickcheck]
  fn encode_decode_bigint_with_endian(n: u128) {
    let s = BigInt::from(n);
    let factory = Base64StringFactory::new(false, false);
    let encoded = factory.encode_with_endian_from_bigint(&s, Endian::BE);
    let decoded = encoded.decode_with_endian_to_bigint(Endian::BE).ok().unwrap();
    assert_eq!(decoded, s);
  }
}
