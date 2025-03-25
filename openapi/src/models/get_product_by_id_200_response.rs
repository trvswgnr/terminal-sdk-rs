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
pub struct GetProductById200Response {
    /// The requested product.
    #[serde(rename = "data")]
    pub data: Box<models::Product>,
}

impl GetProductById200Response {
    pub fn new(data: models::Product) -> GetProductById200Response {
        GetProductById200Response {
            data: Box::new(data),
        }
    }
}

