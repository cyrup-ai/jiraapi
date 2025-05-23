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

/// JqlQueriesToParse : A list of JQL queries to parse.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueriesToParse {
    /// A list of queries to parse.
    #[serde(rename = "queries")]
    pub queries: Vec<String>,
}

impl JqlQueriesToParse {
    /// A list of JQL queries to parse.
    pub fn new(queries: Vec<String>) -> JqlQueriesToParse {
        JqlQueriesToParse {
            queries,
        }
    }
}

