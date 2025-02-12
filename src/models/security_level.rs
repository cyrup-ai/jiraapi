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

/// SecurityLevel : Details of an issue level security item.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityLevel {
    /// The description of the issue level security item.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the issue level security item.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the issue level security item is the default.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The ID of the issue level security scheme.
    #[serde(rename = "issueSecuritySchemeId", skip_serializing_if = "Option::is_none")]
    pub issue_security_scheme_id: Option<String>,
    /// The name of the issue level security item.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the issue level security item.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl SecurityLevel {
    /// Details of an issue level security item.
    pub fn new() -> SecurityLevel {
        SecurityLevel {
            description: None,
            id: None,
            is_default: None,
            issue_security_scheme_id: None,
            name: None,
            param_self: None,
        }
    }
}

