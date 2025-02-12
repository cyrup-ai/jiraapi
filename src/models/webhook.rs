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

/// Webhook : A webhook.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Webhook {
    /// The Jira events that trigger the webhook.
    #[serde(rename = "events")]
    pub events: Vec<Events>,
    /// The date after which the webhook is no longer sent. Use [Extend webhook life](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-webhooks/#api-rest-api-3-webhook-refresh-put) to extend the date.
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<i64>,
    /// A list of field IDs. When the issue changelog contains any of the fields, the webhook `jira:issue_updated` is sent. If this parameter is not present, the app is notified about all field updates.
    #[serde(rename = "fieldIdsFilter", skip_serializing_if = "Option::is_none")]
    pub field_ids_filter: Option<Vec<String>>,
    /// The ID of the webhook.
    #[serde(rename = "id")]
    pub id: i64,
    /// A list of issue property keys. A change of those issue properties triggers the `issue_property_set` or `issue_property_deleted` webhooks. If this parameter is not present, the app is notified about all issue property updates.
    #[serde(rename = "issuePropertyKeysFilter", skip_serializing_if = "Option::is_none")]
    pub issue_property_keys_filter: Option<Vec<String>>,
    /// The JQL filter that specifies which issues the webhook is sent for.
    #[serde(rename = "jqlFilter")]
    pub jql_filter: String,
}

impl Webhook {
    /// A webhook.
    pub fn new(events: Vec<Events>, id: i64, jql_filter: String) -> Webhook {
        Webhook {
            events,
            expiration_date: None,
            field_ids_filter: None,
            id,
            issue_property_keys_filter: None,
            jql_filter,
        }
    }
}
/// The Jira events that trigger the webhook.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Events {
    #[serde(rename = "jira:issue_created")]
    JiraColonIssueCreated,
    #[serde(rename = "jira:issue_updated")]
    JiraColonIssueUpdated,
    #[serde(rename = "jira:issue_deleted")]
    JiraColonIssueDeleted,
    #[serde(rename = "comment_created")]
    CommentCreated,
    #[serde(rename = "comment_updated")]
    CommentUpdated,
    #[serde(rename = "comment_deleted")]
    CommentDeleted,
    #[serde(rename = "issue_property_set")]
    IssuePropertySet,
    #[serde(rename = "issue_property_deleted")]
    IssuePropertyDeleted,
}

impl Default for Events {
    fn default() -> Events {
        Self::JiraColonIssueCreated
    }
}

