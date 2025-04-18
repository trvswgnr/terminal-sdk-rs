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

/// PostAppRequest : Basic app information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostAppRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "redirectURI")]
    pub redirect_uri: String,
}

impl PostAppRequest {
    /// Basic app information.
    pub fn new(name: String, redirect_uri: String) -> PostAppRequest {
        PostAppRequest {
            name,
            redirect_uri,
        }
    }
}

