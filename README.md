# base64-string-rs

A Rust crate for Base64 string.

[![Workflow Status](https://github.com/j5ik2o/base64-string-rs/workflows/Rust/badge.svg)](https://github.com/j5ik2o/base64-string-rs/actions?query=workflow%3A%22Rust%22)
[![crates.io](https://img.shields.io/crates/v/base64-string-rs.svg)](https://crates.io/crates/base64-string-rs)
[![docs.rs](https://docs.rs/base64-string-rs/badge.svg)](https://docs.rs/base64-string-rs)
[![dependency status](https://deps.rs/repo/github/j5ik2o/base64-string-rs/status.svg)](https://deps.rs/repo/github/j5ik2o/base64-string-rs)
[![tokei](https://tokei.rs/b1/github/j5ik2o/base64-string-rs)](https://github.com/XAMPPRocky/tokei)

## Install to Cargo.toml

Add this to your `Cargo.toml`:

```toml
[dependencies]
base64-string-rs = "<<version>>"
```

## About Base64String

Base64String is a string type in Base64 format that contains meta-information about the encoding.

## Usage

```rust
use base64_string_rs::Base64StringFactory;

let str = "0123ABC";
let factory = Base64StringFactory::default();
let encoded = factory.encode_from_string(str);
println!("encoded = {}", encoded);
// encoded = Base64String(value = MDEyM0FCQw, url_safe = false, padding = false)
let decoded = encoded.decode_to_string().unwrap();
println!("decoded = {}", decoded); // 0123ABC
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
