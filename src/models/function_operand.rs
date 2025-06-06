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

/// FunctionOperand : An operand that is a function. See [Advanced searching - functions reference](https://confluence.atlassian.com/x/dwiiLQ) for more information about JQL functions.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionOperand {
    /// The list of function arguments.
    #[serde(rename = "arguments")]
    pub arguments: Vec<String>,
    /// Encoded operand, which can be used directly in a JQL query.
    #[serde(rename = "encodedOperand", skip_serializing_if = "Option::is_none")]
    pub encoded_operand: Option<String>,
    /// The name of the function.
    #[serde(rename = "function")]
    pub function: String,
}

impl FunctionOperand {
    /// An operand that is a function. See [Advanced searching - functions reference](https://confluence.atlassian.com/x/dwiiLQ) for more information about JQL functions.
    pub fn new(arguments: Vec<String>, function: String) -> FunctionOperand {
        FunctionOperand {
            arguments,
            encoded_operand: None,
            function,
        }
    }
}

