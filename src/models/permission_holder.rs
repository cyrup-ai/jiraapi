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

/// PermissionHolder : Details of a user, group, field, or project role that holds a permission. See [Holder object](../api-group-permission-schemes/#holder-object) in *Get all permission schemes* for more information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PermissionHolder {
    /// Expand options that include additional permission holder details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// As a group's name can change, use of `value` is recommended. The identifier associated withthe `type` value that defines the holder of the permission.
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    /// The type of permission holder.
    #[serde(rename = "type")]
    pub r#type: String,
    /// The identifier associated with the `type` value that defines the holder of the permission.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl PermissionHolder {
    /// Details of a user, group, field, or project role that holds a permission. See [Holder object](../api-group-permission-schemes/#holder-object) in *Get all permission schemes* for more information.
    pub fn new(r#type: String) -> PermissionHolder {
        PermissionHolder {
            expand: None,
            parameter: None,
            r#type,
            value: None,
        }
    }
}

