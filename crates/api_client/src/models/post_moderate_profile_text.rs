/*
 * afrodite-backend
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
pub struct PostModerateProfileText {
    #[serde(rename = "accept")]
    pub accept: bool,
    #[serde(rename = "id")]
    pub id: Box<models::AccountId>,
    /// If true, ignore accept, rejected_category, rejected_details and move the text to waiting for human moderation state.
    #[serde(rename = "move_to_human", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub move_to_human: Option<Option<bool>>,
    #[serde(rename = "rejected_category", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rejected_category: Option<Option<Box<models::ProfileTextModerationRejectedReasonCategory>>>,
    #[serde(rename = "rejected_details", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rejected_details: Option<Option<Box<models::ProfileTextModerationRejectedReasonDetails>>>,
    #[serde(rename = "text")]
    pub text: String,
}

impl PostModerateProfileText {
    pub fn new(accept: bool, id: models::AccountId, text: String) -> PostModerateProfileText {
        PostModerateProfileText {
            accept,
            id: Box::new(id),
            move_to_human: None,
            rejected_category: None,
            rejected_details: None,
            text,
        }
    }
}

