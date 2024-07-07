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
pub struct LimitedActionResult {
    #[serde(rename = "status")]
    pub status: crate::models::LimitedActionStatus,
}

impl LimitedActionResult {
    pub fn new(status: crate::models::LimitedActionStatus) -> LimitedActionResult {
        LimitedActionResult {
            status,
        }
    }
}


