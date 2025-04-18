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

/// Card : Credit card used for payments in the Terminal shop.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Card {
    /// Unique object identifier. The format and length of IDs may change over time.
    #[serde(rename = "id")]
    pub id: String,
    /// Brand of the card.
    #[serde(rename = "brand")]
    pub brand: String,
    #[serde(rename = "expiration")]
    pub expiration: Box<models::CardExpiration>,
    /// Last four digits of the card.
    #[serde(rename = "last4")]
    pub last4: String,
}

impl Card {
    /// Credit card used for payments in the Terminal shop.
    pub fn new(id: String, brand: String, expiration: models::CardExpiration, last4: String) -> Card {
        Card {
            id,
            brand,
            expiration: Box::new(expiration),
            last4,
        }
    }
}

