/*
 * pihka-backend
 *
 * Pihka backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HandleModerationRequest {
    #[serde(rename = "accept")]
    pub accept: bool,
}

impl HandleModerationRequest {
    pub fn new(accept: bool) -> HandleModerationRequest {
        HandleModerationRequest {
            accept,
        }
    }
}


