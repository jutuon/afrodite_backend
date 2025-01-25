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
pub struct SoftwareUpdateStatus {
    #[serde(rename = "downloaded", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub downloaded: Option<Option<Box<models::SoftwareInfo>>>,
    #[serde(rename = "installed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub installed: Option<Option<Box<models::SoftwareInfo>>>,
    #[serde(rename = "state")]
    pub state: models::SoftwareUpdateState,
}

impl SoftwareUpdateStatus {
    pub fn new(state: models::SoftwareUpdateState) -> SoftwareUpdateStatus {
        SoftwareUpdateStatus {
            downloaded: None,
            installed: None,
            state,
        }
    }
}

