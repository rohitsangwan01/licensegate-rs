use licensegate_rs::{LicenseGate, ValidationType, licensegate_config::LicenseGateConfig};

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
