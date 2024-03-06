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
pub struct ProfileAttributeFilterValueUpdate {
    #[serde(rename = "accept_missing_attribute")]
    pub accept_missing_attribute: bool,
    /// Bitflags value or top level attribute value ID filter.
    #[serde(rename = "filter_part1", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filter_part1: Option<Option<i32>>,
    /// Sub level attribute value ID filter.
    #[serde(rename = "filter_part2", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filter_part2: Option<Option<i32>>,
    /// Attribute ID
    #[serde(rename = "id")]
    pub id: i32,
}

impl ProfileAttributeFilterValueUpdate {
    pub fn new(accept_missing_attribute: bool, id: i32) -> ProfileAttributeFilterValueUpdate {
        ProfileAttributeFilterValueUpdate {
            accept_missing_attribute,
            filter_part1: None,
            filter_part2: None,
            id,
        }
    }
}


