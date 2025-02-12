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

/// WorkflowProjectIssueTypeUsagePage : A page of issue types.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowProjectIssueTypeUsagePage {
    /// Token for the next page of issue type usages.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The list of issue types.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<models::WorkflowProjectIssueTypeUsage>>,
}

impl WorkflowProjectIssueTypeUsagePage {
    /// A page of issue types.
    pub fn new() -> WorkflowProjectIssueTypeUsagePage {
        WorkflowProjectIssueTypeUsagePage {
            next_page_token: None,
            values: None,
        }
    }
}

