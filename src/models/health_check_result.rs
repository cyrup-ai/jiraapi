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

/// HealthCheckResult : Jira instance health check results. Deprecated and no longer returned.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// The description of the Jira health check item.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the Jira health check item.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the Jira health check item passed or failed.
    #[serde(rename = "passed", skip_serializing_if = "Option::is_none")]
    pub passed: Option<bool>,
}

impl HealthCheckResult {
    /// Jira instance health check results. Deprecated and no longer returned.
    pub fn new() -> HealthCheckResult {
        HealthCheckResult {
            description: None,
            name: None,
            passed: None,
        }
    }
}

