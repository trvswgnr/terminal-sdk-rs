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
pub struct GetSubscriptionById200Response {
    /// Subscription information.
    #[serde(rename = "data")]
    pub data: Box<models::Subscription>,
}

impl GetSubscriptionById200Response {
    pub fn new(data: models::Subscription) -> GetSubscriptionById200Response {
        GetSubscriptionById200Response {
            data: Box::new(data),
        }
    }
}

