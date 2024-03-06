/*
 * pihka-backend
 *
 * Pihka backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Location : Location in latitude and longitude. The values are not NaN, infinity or negative infinity.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "latitude")]
    pub latitude: f64,
    #[serde(rename = "longitude")]
    pub longitude: f64,
}

impl Location {
    /// Location in latitude and longitude. The values are not NaN, infinity or negative infinity.
    pub fn new(latitude: f64, longitude: f64) -> Location {
        Location {
            latitude,
            longitude,
        }
    }
}


