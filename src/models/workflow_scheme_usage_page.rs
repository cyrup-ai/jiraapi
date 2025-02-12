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

/// WorkflowSchemeUsagePage : A page of workflow schemes.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowSchemeUsagePage {
    /// Token for the next page of issue type usages.
    #[serde(rename = "nextPageToken", skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// The list of workflow schemes.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<models::WorkflowSchemeUsage>>,
}

impl WorkflowSchemeUsagePage {
    /// A page of workflow schemes.
    pub fn new() -> WorkflowSchemeUsagePage {
        WorkflowSchemeUsagePage {
            next_page_token: None,
            values: None,
        }
    }
}

