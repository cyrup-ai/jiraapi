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

/// NotificationSchemeEvent : Details about a notification scheme event.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationSchemeEvent {
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<Box<models::NotificationEvent>>,
    #[serde(rename = "notifications", skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<models::EventNotification>>,
}

impl NotificationSchemeEvent {
    /// Details about a notification scheme event.
    pub fn new() -> NotificationSchemeEvent {
        NotificationSchemeEvent {
            event: None,
            notifications: None,
        }
    }
}

