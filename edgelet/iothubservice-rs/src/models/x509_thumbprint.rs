/*
 * IotHub Gateway Service APIs
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: Service
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct X509Thumbprint {
    #[serde(rename = "primaryThumbprint", skip_serializing_if = "Option::is_none")]
    primary_thumbprint: Option<String>,
    #[serde(rename = "secondaryThumbprint", skip_serializing_if = "Option::is_none")]
    secondary_thumbprint: Option<String>,
}

impl X509Thumbprint {
    pub fn new() -> X509Thumbprint {
        X509Thumbprint {
            primary_thumbprint: None,
            secondary_thumbprint: None,
        }
    }

    pub fn set_primary_thumbprint(&mut self, primary_thumbprint: String) {
        self.primary_thumbprint = Some(primary_thumbprint);
    }

    pub fn with_primary_thumbprint(mut self, primary_thumbprint: String) -> X509Thumbprint {
        self.primary_thumbprint = Some(primary_thumbprint);
        self
    }

    pub fn primary_thumbprint(&self) -> Option<&String> {
        self.primary_thumbprint.as_ref()
    }

    pub fn reset_primary_thumbprint(&mut self) {
        self.primary_thumbprint = None;
    }

    pub fn set_secondary_thumbprint(&mut self, secondary_thumbprint: String) {
        self.secondary_thumbprint = Some(secondary_thumbprint);
    }

    pub fn with_secondary_thumbprint(mut self, secondary_thumbprint: String) -> X509Thumbprint {
        self.secondary_thumbprint = Some(secondary_thumbprint);
        self
    }

    pub fn secondary_thumbprint(&self) -> Option<&String> {
        self.secondary_thumbprint.as_ref()
    }

    pub fn reset_secondary_thumbprint(&mut self) {
        self.secondary_thumbprint = None;
    }
}
