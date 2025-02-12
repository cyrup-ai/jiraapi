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

/// WorkflowTransitionRulesDetails : Details about a workflow configuration update request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowTransitionRulesDetails {
    #[serde(rename = "workflowId")]
    pub workflow_id: Box<models::WorkflowId>,
    /// The list of connect workflow rule IDs.
    #[serde(rename = "workflowRuleIds")]
    pub workflow_rule_ids: Vec<String>,
}

impl WorkflowTransitionRulesDetails {
    /// Details about a workflow configuration update request.
    pub fn new(workflow_id: models::WorkflowId, workflow_rule_ids: Vec<String>) -> WorkflowTransitionRulesDetails {
        WorkflowTransitionRulesDetails {
            workflow_id: Box::new(workflow_id),
            workflow_rule_ids,
        }
    }
}

