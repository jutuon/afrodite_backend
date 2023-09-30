/*
 * pihka-backend
 *
 * Pihka backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ResetDataQueryParam : Reset data related to some software.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetDataQueryParam {
    #[serde(rename = "reset_data")]
    pub reset_data: bool,
}

impl ResetDataQueryParam {
    /// Reset data related to some software.
    pub fn new(reset_data: bool) -> ResetDataQueryParam {
        ResetDataQueryParam {
            reset_data,
        }
    }
}


