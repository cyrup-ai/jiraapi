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

/// BulkProjectPermissionGrants : List of project permissions and the projects and issues those permissions grant access to.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkProjectPermissionGrants {
    /// IDs of the issues the user has the permission for.
    #[serde(rename = "issues")]
    pub issues: Vec<i64>,
    /// A project permission,
    #[serde(rename = "permission")]
    pub permission: String,
    /// IDs of the projects the user has the permission for.
    #[serde(rename = "projects")]
    pub projects: Vec<i64>,
}

impl BulkProjectPermissionGrants {
    /// List of project permissions and the projects and issues those permissions grant access to.
    pub fn new(issues: Vec<i64>, permission: String, projects: Vec<i64>) -> BulkProjectPermissionGrants {
        BulkProjectPermissionGrants {
            issues,
            permission,
            projects,
        }
    }
}

