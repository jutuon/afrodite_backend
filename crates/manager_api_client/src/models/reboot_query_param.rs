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

/// RebootQueryParam : Reboot computer directly after software update.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RebootQueryParam {
    #[serde(rename = "reboot")]
    pub reboot: bool,
}

impl RebootQueryParam {
    /// Reboot computer directly after software update.
    pub fn new(reboot: bool) -> RebootQueryParam {
        RebootQueryParam {
            reboot,
        }
    }
}

