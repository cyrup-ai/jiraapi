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

/// SuggestedMappingsForProjectsRequestBean : Details of changes to a priority scheme's projects that require suggested priority mappings.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestedMappingsForProjectsRequestBean {
    /// The ids of projects being added to the scheme.
    #[serde(rename = "add", skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<i64>>,
}

impl SuggestedMappingsForProjectsRequestBean {
    /// Details of changes to a priority scheme's projects that require suggested priority mappings.
    pub fn new() -> SuggestedMappingsForProjectsRequestBean {
        SuggestedMappingsForProjectsRequestBean {
            add: None,
        }
    }
}

