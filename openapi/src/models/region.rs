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

/// Region : A Terminal shop user's region.
/// A Terminal shop user's region.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "na")]
    Na,

}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Eu => write!(f, "eu"),
            Self::Na => write!(f, "na"),
        }
    }
}

impl Default for Region {
    fn default() -> Region {
        Self::Eu
    }
}

