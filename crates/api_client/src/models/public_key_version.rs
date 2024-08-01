/*
 * pihka-backend
 *
 * Pihka backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PublicKeyVersion : Version number for asymmetric encryption public key data which client defines. This allows changing client's end-to-end crypto implementation.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PublicKeyVersion {
    #[serde(rename = "version")]
    pub version: i64,
}

impl PublicKeyVersion {
    /// Version number for asymmetric encryption public key data which client defines. This allows changing client's end-to-end crypto implementation.
    pub fn new(version: i64) -> PublicKeyVersion {
        PublicKeyVersion {
            version,
        }
    }
}


