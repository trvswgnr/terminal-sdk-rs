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

/// PutProfileRequest : The user's updated profile information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutProfileRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "email")]
    pub email: String,
}

impl PutProfileRequest {
    /// The user's updated profile information.
    pub fn new(name: String, email: String) -> PutProfileRequest {
        PutProfileRequest {
            name,
            email,
        }
    }
}

