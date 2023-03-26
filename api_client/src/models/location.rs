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
pub struct Location {
    #[serde(rename = "latitude")]
    pub latitude: f32,
    #[serde(rename = "longitude")]
    pub longitude: f32,
}

impl Location {
    pub fn new(latitude: f32, longitude: f32) -> Location {
        Location {
            latitude,
            longitude,
        }
    }
}


