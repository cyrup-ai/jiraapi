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

/// ProjectIssueTypeHierarchy : The hierarchy of issue types within a project.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectIssueTypeHierarchy {
    /// Details of an issue type hierarchy level.
    #[serde(rename = "hierarchy", skip_serializing_if = "Option::is_none")]
    pub hierarchy: Option<Vec<models::ProjectIssueTypesHierarchyLevel>>,
    /// The ID of the project.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
}

impl ProjectIssueTypeHierarchy {
    /// The hierarchy of issue types within a project.
    pub fn new() -> ProjectIssueTypeHierarchy {
        ProjectIssueTypeHierarchy {
            hierarchy: None,
            project_id: None,
        }
    }
}

