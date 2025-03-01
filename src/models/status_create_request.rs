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

/// StatusCreateRequest : Details of the statuses being created and their scope.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusCreateRequest {
    #[serde(rename = "scope")]
    pub scope: Box<models::StatusScope>,
    /// Details of the statuses being created.
    #[serde(rename = "statuses")]
    pub statuses: Vec<models::StatusCreate>,
}

impl StatusCreateRequest {
    /// Details of the statuses being created and their scope.
    pub fn new(scope: models::StatusScope, statuses: Vec<models::StatusCreate>) -> StatusCreateRequest {
        StatusCreateRequest {
            scope: Box::new(scope),
            statuses,
        }
    }
}

