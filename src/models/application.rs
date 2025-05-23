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

/// Application : The application the linked item is in.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    /// The name of the application. Used in conjunction with the (remote) object icon title to display a tooltip for the link's icon. The tooltip takes the format \"\\[application name\\] icon title\". Blank items are excluded from the tooltip title. If both items are blank, the icon tooltop displays as \"Web Link\". Grouping and sorting of links may place links without an application name last.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The name-spaced type of the application, used by registered rendering apps.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl Application {
    /// The application the linked item is in.
    pub fn new() -> Application {
        Application {
            name: None,
            r#type: None,
        }
    }
}

