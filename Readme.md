# LicenseGate-Rust

[![Crates.io Version](https://img.shields.io/crates/v/licensegate-rs)](https://crates.io/crates/licensegate-rs)
[![docs.rs page](https://docs.rs/licensegate-rs/badge.svg)](https://docs.rs/licensegate-rs)
[![Crates.io Downloads](https://img.shields.io/crates/d/licensegate-rs)](https://crates.io/crates/licensegate-rs)
[![Crates.io License](https://img.shields.io/crates/l/licensegate-rs)](https://crates.io/crates/licensegate-rs)

The`licensegate-rs` is an unofficial Rust SDK for integrating with the [Licensegate](https://licensegate.io/) licensing service

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
licensegate = "0.1.0"

# Or use git version
# licensegate = { git = "https://github.com/rohitsangwan01/licensegate-rs" }
```

## Example

Run example

```sh
cargo run --example sample
```

## Usage

```rust
use licensegate::{LicenseGateVerifier, LicenseGateConfig, ValidationType};

#[tokio::main]
async fn main() {
    let user_id = "Enter USER_ID";
    let license_key = "ENTER_LICENSE_KEY";

    let licensegate = LicenseGate::new(user_id);

    let result = licensegate
        .verify(LicenseGateConfig::new(license_key))
        .await;

    match result {
        Ok(ValidationType::Valid) => println!("The key is valid."),
        Ok(reason) => println!("The key is invalid. Reason: {:?}", reason),
        Err(e) => eprintln!("Connection or server error: {:?}", e),
    }
}
```

### With Custom Server URL

```rust
let licensegate = LicenseGate::new(user_id).set_validation_server("server");
```

### With Public RSA Key (Challenge Verification)

```rust
let licensegate = LicenseGate::new(user_id).set_public_rsa_key("RSA_KEY");
```

### With Scope

```rust
licensegate.verify(LicenseGateConfig::new(license_key).set_scope("scope")).await;
```
