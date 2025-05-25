#[derive(Debug, Default)]
pub struct LicenseGateConfig {
    pub license: String,
    pub scope: Option<String>,
    pub metadata: Option<String>,
}

impl LicenseGateConfig {
    pub fn new(license: impl Into<String>) -> Self {
        Self {
            license: license.into(),
            scope: None,
            metadata: None,
        }
    }

    pub fn set_scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    pub fn set_validation_server(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }
}
