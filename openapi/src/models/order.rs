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

/// Order : An order from the Terminal shop.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// Unique object identifier. The format and length of IDs may change over time.
    #[serde(rename = "id")]
    pub id: String,
    /// Zero-based index of the order for this user only.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "shipping")]
    pub shipping: Box<models::OrderShipping>,
    #[serde(rename = "amount")]
    pub amount: Box<models::OrderAmount>,
    #[serde(rename = "tracking")]
    pub tracking: Box<models::OrderTracking>,
    /// Items in the order.
    #[serde(rename = "items")]
    pub items: Vec<models::OrderItem>,
}

impl Order {
    /// An order from the Terminal shop.
    pub fn new(id: String, shipping: models::OrderShipping, amount: models::OrderAmount, tracking: models::OrderTracking, items: Vec<models::OrderItem>) -> Order {
        Order {
            id,
            index: None,
            shipping: Box::new(shipping),
            amount: Box::new(amount),
            tracking: Box::new(tracking),
            items,
        }
    }
}

