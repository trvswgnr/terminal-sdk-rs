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
pub struct GetApp200Response {
    /// List of apps.
    #[serde(rename = "data")]
    pub data: Vec<models::App>,
}

impl GetApp200Response {
    pub fn new(data: Vec<models::App>) -> GetApp200Response {
        GetApp200Response {
            data,
        }
    }
}

