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

/// UserFilter : Filter for a User Picker (single) custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserFilter {
    /// Whether the filter is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// User groups autocomplete suggestion users must belong to. If not provided, the default values are used. A maximum of 10 groups can be provided.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// Roles that autocomplete suggestion users must belong to. If not provided, the default values are used. A maximum of 10 roles can be provided.
    #[serde(rename = "roleIds", skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<i64>>,
}

impl UserFilter {
    /// Filter for a User Picker (single) custom field.
    pub fn new(enabled: bool) -> UserFilter {
        UserFilter {
            enabled,
            groups: None,
            role_ids: None,
        }
    }
}

