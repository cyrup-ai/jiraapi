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

/// BulkCustomFieldOptionUpdateRequest : Details of the options to update for a custom field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulkCustomFieldOptionUpdateRequest {
    /// Details of the options to update.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<models::CustomFieldOptionUpdate>>,
}

impl BulkCustomFieldOptionUpdateRequest {
    /// Details of the options to update for a custom field.
    pub fn new() -> BulkCustomFieldOptionUpdateRequest {
        BulkCustomFieldOptionUpdateRequest {
            options: None,
        }
    }
}

