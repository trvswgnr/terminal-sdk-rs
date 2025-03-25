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
pub struct OrderItem {
    /// Unique object identifier. The format and length of IDs may change over time.
    #[serde(rename = "id")]
    pub id: String,
    /// Description of the item in the order.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Amount of the item in the order, in cents (USD).
    #[serde(rename = "amount")]
    pub amount: i32,
    /// Quantity of the item in the order.
    #[serde(rename = "quantity")]
    pub quantity: i32,
    /// ID of the product variant of the item in the order.
    #[serde(rename = "productVariantID", skip_serializing_if = "Option::is_none")]
    pub product_variant_id: Option<String>,
}

impl OrderItem {
    pub fn new(id: String, amount: i32, quantity: i32) -> OrderItem {
        OrderItem {
            id,
            description: None,
            amount,
            quantity,
            product_variant_id: None,
        }
    }
}

