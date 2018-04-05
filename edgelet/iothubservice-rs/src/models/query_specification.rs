/*
 * IotHub Gateway Service APIs
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: Service
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// QuerySpecification : A Json query request

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuerySpecification {
    /// The query.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    query: Option<String>,
}

impl QuerySpecification {
    /// A Json query request
    pub fn new() -> QuerySpecification {
        QuerySpecification { query: None }
    }

    pub fn set_query(&mut self, query: String) {
        self.query = Some(query);
    }

    pub fn with_query(mut self, query: String) -> QuerySpecification {
        self.query = Some(query);
        self
    }

    pub fn query(&self) -> Option<&String> {
        self.query.as_ref()
    }

    pub fn reset_query(&mut self) {
        self.query = None;
    }
}
