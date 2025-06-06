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

/// IssueTypeScreenSchemeUpdateDetails : Details of an issue type screen scheme.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemeUpdateDetails {
    /// The description of the issue type screen scheme. The maximum length is 255 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the issue type screen scheme. The name must be unique. The maximum length is 255 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl IssueTypeScreenSchemeUpdateDetails {
    /// Details of an issue type screen scheme.
    pub fn new() -> IssueTypeScreenSchemeUpdateDetails {
        IssueTypeScreenSchemeUpdateDetails {
            description: None,
            name: None,
        }
    }
}

