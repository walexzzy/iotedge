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
pub struct VariantValueDateTime {
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(rename = "ObjectValue", skip_serializing_if = "Option::is_none")]
    object_value: Option<Value>,
    #[serde(rename = "IsNull", skip_serializing_if = "Option::is_none")]
    is_null: Option<bool>,
}

impl VariantValueDateTime {
    pub fn new() -> VariantValueDateTime {
        VariantValueDateTime {
            value: None,
            object_value: None,
            is_null: None,
        }
    }

    pub fn set_value(&mut self, value: String) {
        self.value = Some(value);
    }

    pub fn with_value(mut self, value: String) -> VariantValueDateTime {
        self.value = Some(value);
        self
    }

    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub fn reset_value(&mut self) {
        self.value = None;
    }

    pub fn set_object_value(&mut self, object_value: Value) {
        self.object_value = Some(object_value);
    }

    pub fn with_object_value(mut self, object_value: Value) -> VariantValueDateTime {
        self.object_value = Some(object_value);
        self
    }

    pub fn object_value(&self) -> Option<&Value> {
        self.object_value.as_ref()
    }

    pub fn reset_object_value(&mut self) {
        self.object_value = None;
    }

    pub fn set_is_null(&mut self, is_null: bool) {
        self.is_null = Some(is_null);
    }

    pub fn with_is_null(mut self, is_null: bool) -> VariantValueDateTime {
        self.is_null = Some(is_null);
        self
    }

    pub fn is_null(&self) -> Option<&bool> {
        self.is_null.as_ref()
    }

    pub fn reset_is_null(&mut self) {
        self.is_null = None;
    }
}
