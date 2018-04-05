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
pub struct ServiceStatistics {
    #[serde(rename = "connectedDeviceCount", skip_serializing_if = "Option::is_none")]
    connected_device_count: Option<i64>,
}

impl ServiceStatistics {
    pub fn new() -> ServiceStatistics {
        ServiceStatistics {
            connected_device_count: None,
        }
    }

    pub fn set_connected_device_count(&mut self, connected_device_count: i64) {
        self.connected_device_count = Some(connected_device_count);
    }

    pub fn with_connected_device_count(mut self, connected_device_count: i64) -> ServiceStatistics {
        self.connected_device_count = Some(connected_device_count);
        self
    }

    pub fn connected_device_count(&self) -> Option<&i64> {
        self.connected_device_count.as_ref()
    }

    pub fn reset_connected_device_count(&mut self) {
        self.connected_device_count = None;
    }
}
