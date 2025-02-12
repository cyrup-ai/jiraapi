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

/// JiraWorkflowStatus : Details of a status.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraWorkflowStatus {
    /// The description of the status.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the status.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the status.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Box<models::WorkflowScope>>,
    /// The category of the status.
    #[serde(rename = "statusCategory", skip_serializing_if = "Option::is_none")]
    pub status_category: Option<StatusCategory>,
    /// The reference of the status.
    #[serde(rename = "statusReference", skip_serializing_if = "Option::is_none")]
    pub status_reference: Option<String>,
    /// Deprecated. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/changelog/#CHANGE-2298) for details.  The `statuses.usages` expand is an optional parameter that can be used when reading and updating statuses in Jira. It provides additional information about the projects and issue types associated with the requested statuses.
    #[serde(rename = "usages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub usages: Option<Option<Vec<models::ProjectIssueTypes>>>,
}

impl JiraWorkflowStatus {
    /// Details of a status.
    pub fn new() -> JiraWorkflowStatus {
        JiraWorkflowStatus {
            description: None,
            id: None,
            name: None,
            scope: None,
            status_category: None,
            status_reference: None,
            usages: None,
        }
    }
}
/// The category of the status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusCategory {
    #[serde(rename = "TODO")]
    Todo,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "DONE")]
    Done,
}

impl Default for StatusCategory {
    fn default() -> StatusCategory {
        Self::Todo
    }
}

