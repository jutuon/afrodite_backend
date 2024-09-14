/*
 * pihka-backend
 *
 * Pihka backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetProfileQueryParam {
    /// If requested profile is not public, allow getting the profile data if the requested profile is a match.
    #[serde(rename = "is_match", skip_serializing_if = "Option::is_none")]
    pub is_match: Option<bool>,
    /// Profile version UUID
    #[serde(rename = "v", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub v: Option<Option<uuid::Uuid>>,
}

impl GetProfileQueryParam {
    pub fn new() -> GetProfileQueryParam {
        GetProfileQueryParam {
            is_match: None,
            v: None,
        }
    }
}

