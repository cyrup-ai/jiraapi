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

/// FieldConfigurationSchemeProjects : Project list with assigned field configuration schema.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldConfigurationSchemeProjects {
    #[serde(rename = "fieldConfigurationScheme", skip_serializing_if = "Option::is_none")]
    pub field_configuration_scheme: Option<Box<models::FieldConfigurationScheme>>,
    /// The IDs of projects using the field configuration scheme.
    #[serde(rename = "projectIds")]
    pub project_ids: Vec<String>,
}

impl FieldConfigurationSchemeProjects {
    /// Project list with assigned field configuration schema.
    pub fn new(project_ids: Vec<String>) -> FieldConfigurationSchemeProjects {
        FieldConfigurationSchemeProjects {
            field_configuration_scheme: None,
            project_ids,
        }
    }
}

