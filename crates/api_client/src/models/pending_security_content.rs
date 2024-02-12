/*
 * pihka-backend
 *
 * Pihka backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PendingSecurityContent : Security content settings which will be applied when moderation request is accepted.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PendingSecurityContent {
    #[serde(rename = "content_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content_id: Option<Option<Box<crate::models::ContentInfo>>>,
}

impl PendingSecurityContent {
    /// Security content settings which will be applied when moderation request is accepted.
    pub fn new() -> PendingSecurityContent {
        PendingSecurityContent {
            content_id: None,
        }
    }
}


