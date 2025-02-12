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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueBulkTransitionForWorkflow {
    /// Indicates whether all the transitions of this workflow are available in the transitions list or not.
    #[serde(rename = "isTransitionsFiltered", skip_serializing_if = "Option::is_none")]
    pub is_transitions_filtered: Option<bool>,
    /// List of issue keys from the request which are associated with this workflow.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<String>>,
    /// List of transitions available for issues from the request which are associated with this workflow.   **This list includes only those transitions that are common across the issues in this workflow and do not involve any additional field updates.** 
    #[serde(rename = "transitions", skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<models::SimplifiedIssueTransition>>,
}

impl IssueBulkTransitionForWorkflow {
    pub fn new() -> IssueBulkTransitionForWorkflow {
        IssueBulkTransitionForWorkflow {
            is_transitions_filtered: None,
            issues: None,
            transitions: None,
        }
    }
}

