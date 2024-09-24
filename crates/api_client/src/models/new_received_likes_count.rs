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
pub struct NewReceivedLikesCount {
    #[serde(rename = "c")]
    pub c: i64,
}

impl NewReceivedLikesCount {
    pub fn new(c: i64) -> NewReceivedLikesCount {
        NewReceivedLikesCount {
            c,
        }
    }
}

