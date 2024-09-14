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
pub struct ProfileAttributeValue {
    /// Attribute ID
    #[serde(rename = "id")]
    pub id: i32,
    /// - First value is bitflags value or top level attribute value ID or first number list value. - Second value is sub level attribute value ID or second number list value. - Third and rest are number list values.  The number list values are in ascending order.
    #[serde(rename = "v")]
    pub v: Vec<i32>,
}

impl ProfileAttributeValue {
    pub fn new(id: i32, v: Vec<i32>) -> ProfileAttributeValue {
        ProfileAttributeValue {
            id,
            v,
        }
    }
}

