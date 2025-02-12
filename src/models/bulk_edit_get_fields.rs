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

/// BulkEditGetFields : Bulk Edit Get Fields Response.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkEditGetFields {
    /// The end cursor for use in pagination.
    #[serde(rename = "endingBefore", skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// List of all the fields
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<models::IssueBulkEditField>>,
    /// The start cursor for use in pagination.
    #[serde(rename = "startingAfter", skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

impl BulkEditGetFields {
    /// Bulk Edit Get Fields Response.
    pub fn new() -> BulkEditGetFields {
        BulkEditGetFields {
            ending_before: None,
            fields: None,
            starting_after: None,
        }
    }
}

