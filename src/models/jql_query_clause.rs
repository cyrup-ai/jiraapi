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

/// JqlQueryClause : A JQL query clause.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueryClause {
    /// The list of nested clauses.
    #[serde(rename = "clauses")]
    pub clauses: Vec<models::JqlQueryClause>,
    /// The operator applied to the field.
    #[serde(rename = "operator")]
    pub operator: Operator,
    #[serde(rename = "field")]
    pub field: Box<models::JqlQueryField>,
    #[serde(rename = "operand")]
    pub operand: Box<models::JqlQueryClauseOperand>,
    /// The list of time predicates.
    #[serde(rename = "predicates")]
    pub predicates: Vec<models::JqlQueryClauseTimePredicate>,
}

impl JqlQueryClause {
    /// A JQL query clause.
    pub fn new(clauses: Vec<models::JqlQueryClause>, operator: Operator, field: models::JqlQueryField, operand: models::JqlQueryClauseOperand, predicates: Vec<models::JqlQueryClauseTimePredicate>) -> JqlQueryClause {
        JqlQueryClause {
            clauses,
            operator,
            field: Box::new(field),
            operand: Box::new(operand),
            predicates,
        }
    }
}
/// The operator applied to the field.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "changed")]
    Changed,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::Changed
    }
}

