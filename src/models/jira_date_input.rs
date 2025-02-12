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
pub struct JiraDateInput {
    #[serde(rename = "formattedDate")]
    pub formatted_date: String,
}

impl JiraDateInput {
    pub fn new(formatted_date: String) -> JiraDateInput {
        JiraDateInput {
            formatted_date,
        }
    }
}

