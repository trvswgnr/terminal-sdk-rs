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

/// PostOrderRequest : Order information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostOrderRequest {
    /// Product variants to include in the order, along with their quantities.
    #[serde(rename = "variants")]
    pub variants: std::collections::HashMap<String, i32>,
    /// Card ID.
    #[serde(rename = "cardID")]
    pub card_id: String,
    /// Shipping address ID.
    #[serde(rename = "addressID")]
    pub address_id: String,
}

impl PostOrderRequest {
    /// Order information.
    pub fn new(variants: std::collections::HashMap<String, i32>, card_id: String, address_id: String) -> PostOrderRequest {
        PostOrderRequest {
            variants,
            card_id,
            address_id,
        }
    }
}

