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

/// CustomFieldConfigurations : Details of configurations for a custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomFieldConfigurations {
    /// The list of custom field configuration details.
    #[serde(rename = "configurations")]
    pub configurations: Vec<models::ContextualConfiguration>,
}

impl CustomFieldConfigurations {
    /// Details of configurations for a custom field.
    pub fn new(configurations: Vec<models::ContextualConfiguration>) -> CustomFieldConfigurations {
        CustomFieldConfigurations {
            configurations,
        }
    }
}

