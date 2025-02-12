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
pub struct WorklogsMoveRequestBean {
    /// A list of worklog IDs.
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
    /// The issue id or key of the destination issue
    #[serde(rename = "issueIdOrKey", skip_serializing_if = "Option::is_none")]
    pub issue_id_or_key: Option<String>,
}

impl WorklogsMoveRequestBean {
    pub fn new() -> WorklogsMoveRequestBean {
        WorklogsMoveRequestBean {
            ids: None,
            issue_id_or_key: None,
        }
    }
}

