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
pub struct GetProfile200Response {
    /// User profile information.
    #[serde(rename = "data")]
    pub data: Box<models::Profile>,
}

impl GetProfile200Response {
    pub fn new(data: models::Profile) -> GetProfile200Response {
        GetProfile200Response {
            data: Box::new(data),
        }
    }
}

