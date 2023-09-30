/*
 * pihka-backend
 *
 * Pihka backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// MapTileX : X coordinate of slippy map tile.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MapTileX {
    #[serde(rename = "x")]
    pub x: i32,
}

impl MapTileX {
    /// X coordinate of slippy map tile.
    pub fn new(x: i32) -> MapTileX {
        MapTileX {
            x,
        }
    }
}


