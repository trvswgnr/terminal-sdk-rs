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

/// OrderAmount : The subtotal and shipping amounts of the order.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderAmount {
    /// Shipping amount of the order, in cents (USD).
    #[serde(rename = "shipping")]
    pub shipping: i32,
    /// Subtotal amount of the order, in cents (USD).
    #[serde(rename = "subtotal")]
    pub subtotal: i32,
}

impl OrderAmount {
    /// The subtotal and shipping amounts of the order.
    pub fn new(shipping: i32, subtotal: i32) -> OrderAmount {
        OrderAmount {
            shipping,
            subtotal,
        }
    }
}

