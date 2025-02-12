/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT-dc4387b5f5a71cc1c35f02124338ae55f24e0954
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimpleErrorCollection {
    /// The list of error messages produced by this operation. For example, \"input parameter 'key' must be provided\"
    #[serde(rename = "errorMessages", skip_serializing_if = "Option::is_none")]
    pub error_messages: Option<Vec<String>>,
    /// The list of errors by parameter returned by the operation. For example,\"projectKey\": \"Project keys must start with an uppercase letter, followed by one or more uppercase alphanumeric characters.\"
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "httpStatusCode", skip_serializing_if = "Option::is_none")]
    pub http_status_code: Option<i32>,
}

impl SimpleErrorCollection {
    pub fn new() -> SimpleErrorCollection {
        SimpleErrorCollection {
            error_messages: None,
            errors: None,
            http_status_code: None,
        }
    }
}

