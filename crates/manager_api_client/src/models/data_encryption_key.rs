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
pub struct DataEncryptionKey {
    /// Base64 key
    #[serde(rename = "key")]
    pub key: String,
}

impl DataEncryptionKey {
    pub fn new(key: String) -> DataEncryptionKey {
        DataEncryptionKey {
            key,
        }
    }
}

