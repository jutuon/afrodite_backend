/*
 * pihka-backend
 *
 * Pihka backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BackendConfig {
    #[serde(rename = "bots", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bots: Option<Option<Box<crate::models::BotConfig>>>,
}

impl BackendConfig {
    pub fn new() -> BackendConfig {
        BackendConfig {
            bots: None,
        }
    }
}


