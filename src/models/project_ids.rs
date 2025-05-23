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

/// ProjectIds : A list of project IDs.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectIds {
    /// The IDs of projects.
    #[serde(rename = "projectIds")]
    pub project_ids: Vec<String>,
}

impl ProjectIds {
    /// A list of project IDs.
    pub fn new(project_ids: Vec<String>) -> ProjectIds {
        ProjectIds {
            project_ids,
        }
    }
}

