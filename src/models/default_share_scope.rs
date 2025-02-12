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

/// DefaultShareScope : Details of the scope of the default sharing for new filters and dashboards.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultShareScope {
    /// The scope of the default sharing for new filters and dashboards:   *  `AUTHENTICATED` Shared with all logged-in users.  *  `GLOBAL` Shared with all logged-in users. This shows as `AUTHENTICATED` in the response.  *  `PRIVATE` Not shared with any users.
    #[serde(rename = "scope")]
    pub scope: Scope,
}

impl DefaultShareScope {
    /// Details of the scope of the default sharing for new filters and dashboards.
    pub fn new(scope: Scope) -> DefaultShareScope {
        DefaultShareScope {
            scope,
        }
    }
}
/// The scope of the default sharing for new filters and dashboards:   *  `AUTHENTICATED` Shared with all logged-in users.  *  `GLOBAL` Shared with all logged-in users. This shows as `AUTHENTICATED` in the response.  *  `PRIVATE` Not shared with any users.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Scope {
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "AUTHENTICATED")]
    Authenticated,
    #[serde(rename = "PRIVATE")]
    Private,
}

impl Default for Scope {
    fn default() -> Scope {
        Self::Global
    }
}

