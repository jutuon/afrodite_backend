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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemInfo {
    #[serde(rename = "info")]
    pub info: Vec<models::CommandOutput>,
    #[serde(rename = "name")]
    pub name: String,
}

impl SystemInfo {
    pub fn new(info: Vec<models::CommandOutput>, name: String) -> SystemInfo {
        SystemInfo {
            info,
            name,
        }
    }
}

