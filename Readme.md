# LicenseGate-Rust

[![Crates.io Version](https://img.shields.io/crates/v/licensegate-rs)](https://crates.io/crates/licensegate-rs)
[![docs.rs page](https://docs.rs/licensegate-rs/badge.svg)](https://docs.rs/licensegate-rs)
[![Crates.io Downloads](https://img.shields.io/crates/d/licensegate-rs)](https://crates.io/crates/licensegate-rs)
[![Crates.io License](https://img.shields.io/crates/l/licensegate-rs)](https://crates.io/crates/licensegate-rs)

`licensegate-rs` is an **unofficial Rust SDK** for integrating with the [LicenseGate](https://licensegate.io/) licensing and activation service.

<a href="https://licensegate.io/">
    <img src="https://licensegate.io/images/logo.svg" width="200" alt="LicenseGate Logo">
</a>

---

## ✨ Features

- License key verification with LicenseGate
- Challenge-response (RSA public key) validation
- Custom license scopes
- Configurable LicenseGate server URL
- Async-friendly API using `tokio`

---

## 📦 Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
licensegate-rs = "0.1.0"
```

## 🚀 Quick Start

Here’s a minimal example for verifying a license key:

```rust
use licensegate::{LicenseGate, LicenseGateConfig, ValidationType};

#[tokio::main]
async fn main() {
    let user_id = "ENTER_USER_ID";
    let license_key = "ENTER_LICENSE_KEY";

    let licensegate = LicenseGate::new(user_id);

    match licensegate.verify(LicenseGateConfig::new(license_key)).await {
        Ok(ValidationType::Valid) => println!("The key is valid."),
        Ok(reason) => println!("The key is invalid. Reason: {:?}", reason),
        Err(e) => eprintln!("Connection or server error: {:?}", e),
    }
}
```

## ⚙️ Advanced Usage

### Custom LicenseGate Server URL

```rust
let licensegate = LicenseGate::new(user_id)
    .set_validation_server("https://your.custom.server");
```

### Public RSA Key for Challenge Verification

```rust
let licensegate = LicenseGate::new(user_id)
    .set_public_rsa_key("PUBLIC_RSA_KEY");
```

## License Key with Scoped Access

```rust
let config = LicenseGateConfig::new(license_key)
    .set_scope("pro");
let result = licensegate.verify(config).await;
```

## 📂 Run Example

To test with the provided example:

```bash
cargo run --example sample
```

## 📚 Documentation

Find full API docs on [docs.rs](https://docs.rs/licensegate-rs/)

## 📝 License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT)
