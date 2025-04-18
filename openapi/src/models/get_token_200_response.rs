/*
 * Terminal API
 *
 * The Terminal API gives you access to the same API that powers the award winning Terminal SSH shop (`ssh terminal.shop`).
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetToken200Response {
    /// List of personal access tokens.
    #[serde(rename = "data")]
    pub data: Vec<models::Token>,
}

impl GetToken200Response {
    pub fn new(data: Vec<models::Token>) -> GetToken200Response {
        GetToken200Response {
            data,
        }
    }
}

