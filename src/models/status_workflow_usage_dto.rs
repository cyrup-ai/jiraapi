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

/// StatusWorkflowUsageDto : Workflows using the status.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusWorkflowUsageDto {
    /// The status ID.
    #[serde(rename = "statusId", skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
    #[serde(rename = "workflows", skip_serializing_if = "Option::is_none")]
    pub workflows: Option<Box<models::StatusWorkflowUsagePage>>,
}

impl StatusWorkflowUsageDto {
    /// Workflows using the status.
    pub fn new() -> StatusWorkflowUsageDto {
        StatusWorkflowUsageDto {
            status_id: None,
            workflows: None,
        }
    }
}

