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

/// UpdatePrioritiesInSchemeRequestBean : Update priorities in a scheme
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatePrioritiesInSchemeRequestBean {
    /// Priorities to add to a scheme
    #[serde(rename = "add", skip_serializing_if = "Option::is_none")]
    pub add: Option<Box<models::PrioritySchemeChangesWithoutMappings>>,
    /// Priorities to remove from a scheme
    #[serde(rename = "remove", skip_serializing_if = "Option::is_none")]
    pub remove: Option<Box<models::PrioritySchemeChangesWithoutMappings>>,
}

impl UpdatePrioritiesInSchemeRequestBean {
    /// Update priorities in a scheme
    pub fn new() -> UpdatePrioritiesInSchemeRequestBean {
        UpdatePrioritiesInSchemeRequestBean {
            add: None,
            remove: None,
        }
    }
}

