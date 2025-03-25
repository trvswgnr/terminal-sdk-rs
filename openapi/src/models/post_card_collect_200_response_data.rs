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

/// PostCardCollect200ResponseData : URL for collecting card information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostCardCollect200ResponseData {
    /// Temporary URL that allows a user to enter credit card details over https at terminal.shop.
    #[serde(rename = "url")]
    pub url: String,
}

impl PostCardCollect200ResponseData {
    /// URL for collecting card information.
    pub fn new(url: String) -> PostCardCollect200ResponseData {
        PostCardCollect200ResponseData {
            url,
        }
    }
}

