pub mod licensegate_config;

use base64::{Engine, engine::general_purpose};
use licensegate_config::LicenseGateConfig;
use reqwest::Client;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationType {
    Valid,
    NotFound,
    NotActive,
    Expired,
    LicenseScopeFailed,
    IpLimitExceeded,
    RateLimitExceeded,
    FailedChallenge,
    ServerError,
    ConnectionError,
}

#[derive(Debug, Error)]
pub enum LicenseGateError {
    #[error("server error")]
    ServerError,
    #[error("connection error")]
    ConnectionError,
    #[error("challenge verification failed")]
    ChallengeFailed,
    #[error("validation result: {0:?}")]
    Validation(ValidationType),
}

pub struct LicenseGate {
    user_id: String,
    public_rsa_key: Option<String>,
    validation_server: String,
    use_challenge: bool,
    debug_mode: bool,
    client: Arc<Client>,
}

impl LicenseGate {
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            public_rsa_key: None,
            validation_server: "https://api.licensegate.io".into(),
            use_challenge: false,
            debug_mode: false,
            client: Arc::new(Client::new()),
        }
    }

    pub fn set_public_rsa_key(mut self, key: impl Into<String>) -> Self {
        self.public_rsa_key = Some(key.into());
        self.use_challenge = true;
        self
    }

    pub fn set_validation_server(mut self, server: impl Into<String>) -> Self {
        self.validation_server = server.into();
        self
    }

    pub fn use_challenges(mut self) -> Self {
        self.use_challenge = true;
        self
    }

    pub fn debug(mut self) -> Self {
        self.debug_mode = true;
        self
    }

    pub async fn verify(
        &self,
        config: LicenseGateConfig,
    ) -> Result<ValidationType, LicenseGateError> {
        let challenge = if self.use_challenge {
            Some(format!("{}", chrono::Utc::now().timestamp_millis()))
        } else {
            None
        };

        let url = self.build_url(
            &config.license,
            config.scope,
            config.metadata,
            challenge.as_deref(),
        );

        let res = self
            .client
            .get(&url)
            .header("User-Agent", "RohitSangwan/LicenseGate-Rust")
            .send()
            .await
            .map_err(|_| LicenseGateError::ConnectionError)?;

        let status = res.status();
        let data: serde_json::Value = res
            .json()
            .await
            .map_err(|_| LicenseGateError::ServerError)?;

        if self.debug_mode {
            println!("\nRequest URL: {}", url);
            println!("Status: {}", status);
            println!("Response: {}", data);
        }

        if let Some(error) = data.get("error") {
            if self.debug_mode {
                println!("Error: {}", error);
            }
            return Err(LicenseGateError::ServerError);
        }

        let valid = data.get("valid").and_then(|v| v.as_bool()).unwrap_or(true);
        let result_str = data
            .get("result")
            .and_then(|r| r.as_str())
            .unwrap_or("SERVER_ERROR");

        let result = match result_str {
            "VALID" if !valid => ValidationType::ServerError,
            "VALID" => ValidationType::Valid,
            "NOT_FOUND" => ValidationType::NotFound,
            "NOT_ACTIVE" => ValidationType::NotActive,
            "EXPIRED" => ValidationType::Expired,
            "LICENSE_SCOPE_FAILED" => ValidationType::LicenseScopeFailed,
            "IP_LIMIT_EXCEEDED" => ValidationType::IpLimitExceeded,
            "RATE_LIMIT_EXCEEDED" => ValidationType::RateLimitExceeded,
            _ => ValidationType::ServerError,
        };

        if !valid {
            return Err(LicenseGateError::Validation(result));
        }

        if self.use_challenge {
            if let Some(signed) = data.get("signedChallenge").and_then(|s| s.as_str()) {
                if !self.verify_challenge(challenge.unwrap().as_bytes(), signed) {
                    return Err(LicenseGateError::ChallengeFailed);
                }
            } else {
                return Err(LicenseGateError::ChallengeFailed);
            }
        }

        Ok(result)
    }

    pub async fn verify_simple(&self, config: LicenseGateConfig) -> bool {
        matches!(self.verify(config).await, Ok(ValidationType::Valid))
    }

    fn build_url(
        &self,
        license_key: &str,
        scope: Option<String>,
        metadata: Option<String>,
        challenge: Option<&str>,
    ) -> String {
        let mut query = vec![];
        if let Some(meta) = metadata {
            query.push(format!("metadata={}", urlencoding::encode(&meta)));
        }
        if let Some(sc) = scope {
            query.push(format!("scope={}", urlencoding::encode(&sc)));
        }
        if let Some(ch) = challenge {
            query.push(format!("challenge={}", urlencoding::encode(ch)));
        }
        let query_str = if query.is_empty() {
            "".into()
        } else {
            format!("?{}", query.join("&"))
        };
        format!(
            "{}/license/{}/{}/verify{}",
            self.validation_server, self.user_id, license_key, query_str
        )
    }

    fn verify_challenge(&self, challenge: &[u8], signed_base64: &str) -> bool {
        use openssl::hash::MessageDigest;
        use openssl::rsa::Rsa;
        use openssl::sign::Verifier;

        if let Some(pub_key_pem) = &self.public_rsa_key {
            let pub_key_pem = Self::normalize_pem_format(pub_key_pem);
            match Rsa::public_key_from_pem(pub_key_pem.as_bytes()) {
                Ok(rsa) => {
                    let pub_key = rsa.public_key_to_pem().unwrap();
                    let pkey = openssl::pkey::PKey::public_key_from_pem(&pub_key).unwrap();
                    let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey).unwrap();
                    verifier.update(challenge).unwrap();
                    if let Ok(sig) = general_purpose::STANDARD.decode(signed_base64) {
                        return verifier.verify(&sig).unwrap_or(false);
                    }
                }
                Err(e) => {
                    if self.debug_mode {
                        eprintln!("RSA decode error: {}", e);
                    }
                }
            }
        }

        false
    }

    fn normalize_pem_format(key: &str) -> String {
        let key = key.replace("\\n", "\n").trim().to_string();
        if key.contains("-----BEGIN") {
            println!("Formatting key");
            key.replace("-----BEGIN PUBLIC KEY-----", "-----BEGIN PUBLIC KEY-----\n")
                .replace("-----END PUBLIC KEY-----", "\n-----END PUBLIC KEY-----")
        } else {
            key
        }
    }
}
