/*
 * dating-app-backend
 *
 * Dating app backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountIdDbValue {
    #[serde(rename = "account_db_id")]
    pub account_db_id: i64,
}

impl AccountIdDbValue {
    pub fn new(account_db_id: i64) -> AccountIdDbValue {
        AccountIdDbValue {
            account_db_id,
        }
    }
}

