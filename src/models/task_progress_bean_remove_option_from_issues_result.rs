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

/// TaskProgressBeanRemoveOptionFromIssuesResult : Details about a task.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskProgressBeanRemoveOptionFromIssuesResult {
    /// The description of the task.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The execution time of the task, in milliseconds.
    #[serde(rename = "elapsedRuntime")]
    pub elapsed_runtime: i64,
    /// A timestamp recording when the task was finished.
    #[serde(rename = "finished", skip_serializing_if = "Option::is_none")]
    pub finished: Option<i64>,
    /// The ID of the task.
    #[serde(rename = "id")]
    pub id: String,
    /// A timestamp recording when the task progress was last updated.
    #[serde(rename = "lastUpdate")]
    pub last_update: i64,
    /// Information about the progress of the task.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The progress of the task, as a percentage complete.
    #[serde(rename = "progress")]
    pub progress: i64,
    /// The result of the task execution.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<models::RemoveOptionFromIssuesResult>>,
    /// The URL of the task.
    #[serde(rename = "self")]
    pub param_self: String,
    /// A timestamp recording when the task was started.
    #[serde(rename = "started", skip_serializing_if = "Option::is_none")]
    pub started: Option<i64>,
    /// The status of the task.
    #[serde(rename = "status")]
    pub status: Status,
    /// A timestamp recording when the task was submitted.
    #[serde(rename = "submitted")]
    pub submitted: i64,
    /// The ID of the user who submitted the task.
    #[serde(rename = "submittedBy")]
    pub submitted_by: i64,
}

impl TaskProgressBeanRemoveOptionFromIssuesResult {
    /// Details about a task.
    pub fn new(elapsed_runtime: i64, id: String, last_update: i64, progress: i64, param_self: String, status: Status, submitted: i64, submitted_by: i64) -> TaskProgressBeanRemoveOptionFromIssuesResult {
        TaskProgressBeanRemoveOptionFromIssuesResult {
            description: None,
            elapsed_runtime,
            finished: None,
            id,
            last_update,
            message: None,
            progress,
            result: None,
            param_self,
            started: None,
            status,
            submitted,
            submitted_by,
        }
    }
}
/// The status of the task.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "ENQUEUED")]
    Enqueued,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "COMPLETE")]
    Complete,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "CANCEL_REQUESTED")]
    CancelRequested,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "DEAD")]
    Dead,
}

impl Default for Status {
    fn default() -> Status {
        Self::Enqueued
    }
}

