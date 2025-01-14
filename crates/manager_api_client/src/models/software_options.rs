/*
 * afrodite-manager
 *
 * Afrodite manager API
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SoftwareOptions {
    #[serde(rename = "Backend")]
    Backend,

}

impl std::fmt::Display for SoftwareOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Backend => write!(f, "Backend"),
        }
    }
}

impl Default for SoftwareOptions {
    fn default() -> SoftwareOptions {
        Self::Backend
    }
}

