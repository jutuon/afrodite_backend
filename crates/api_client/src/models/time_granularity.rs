/*
 * pihka-backend
 *
 * Pihka backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeGranularity {
    #[serde(rename = "Minutes")]
    Minutes,
    #[serde(rename = "Hours")]
    Hours,

}

impl ToString for TimeGranularity {
    fn to_string(&self) -> String {
        match self {
            Self::Minutes => String::from("Minutes"),
            Self::Hours => String::from("Hours"),
        }
    }
}

impl Default for TimeGranularity {
    fn default() -> TimeGranularity {
        Self::Minutes
    }
}




