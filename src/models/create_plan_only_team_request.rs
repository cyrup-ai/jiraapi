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
pub struct CreatePlanOnlyTeamRequest {
    /// The capacity for the plan-only team.
    #[serde(rename = "capacity", skip_serializing_if = "Option::is_none")]
    pub capacity: Option<f64>,
    /// The ID of the issue source for the plan-only team.
    #[serde(rename = "issueSourceId", skip_serializing_if = "Option::is_none")]
    pub issue_source_id: Option<i64>,
    /// The account IDs of the plan-only team members.
    #[serde(rename = "memberAccountIds", skip_serializing_if = "Option::is_none")]
    pub member_account_ids: Option<Vec<String>>,
    /// The plan-only team name.
    #[serde(rename = "name")]
    pub name: String,
    /// The planning style for the plan-only team. This must be \"Scrum\" or \"Kanban\".
    #[serde(rename = "planningStyle")]
    pub planning_style: PlanningStyle,
    /// The sprint length for the plan-only team.
    #[serde(rename = "sprintLength", skip_serializing_if = "Option::is_none")]
    pub sprint_length: Option<i64>,
}

impl CreatePlanOnlyTeamRequest {
    pub fn new(name: String, planning_style: PlanningStyle) -> CreatePlanOnlyTeamRequest {
        CreatePlanOnlyTeamRequest {
            capacity: None,
            issue_source_id: None,
            member_account_ids: None,
            name,
            planning_style,
            sprint_length: None,
        }
    }
}
/// The planning style for the plan-only team. This must be \"Scrum\" or \"Kanban\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlanningStyle {
    #[serde(rename = "Scrum")]
    Scrum,
    #[serde(rename = "Kanban")]
    Kanban,
}

impl Default for PlanningStyle {
    fn default() -> PlanningStyle {
        Self::Scrum
    }
}

