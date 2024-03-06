/*
 * pihka-backend
 *
 * Pihka backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// EventType : Identifier for event.

/// Identifier for event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "AccountStateChanged")]
    AccountStateChanged,
    #[serde(rename = "AccountCapabilitiesChanged")]
    AccountCapabilitiesChanged,
    #[serde(rename = "ProfileVisibilityChanged")]
    ProfileVisibilityChanged,
    #[serde(rename = "AccountSyncVersionChanged")]
    AccountSyncVersionChanged,
    #[serde(rename = "NewMessageReceived")]
    NewMessageReceived,
    #[serde(rename = "ReceivedLikesChanged")]
    ReceivedLikesChanged,
    #[serde(rename = "ReceivedBlocksChanged")]
    ReceivedBlocksChanged,
    #[serde(rename = "SentLikesChanged")]
    SentLikesChanged,
    #[serde(rename = "SentBlocksChanged")]
    SentBlocksChanged,
    #[serde(rename = "MatchesChanged")]
    MatchesChanged,
    #[serde(rename = "LatestViewedMessageChanged")]
    LatestViewedMessageChanged,
    #[serde(rename = "ContentProcessingStateChanged")]
    ContentProcessingStateChanged,
    #[serde(rename = "AvailableProfileAttributesChanged")]
    AvailableProfileAttributesChanged,

}

impl ToString for EventType {
    fn to_string(&self) -> String {
        match self {
            Self::AccountStateChanged => String::from("AccountStateChanged"),
            Self::AccountCapabilitiesChanged => String::from("AccountCapabilitiesChanged"),
            Self::ProfileVisibilityChanged => String::from("ProfileVisibilityChanged"),
            Self::AccountSyncVersionChanged => String::from("AccountSyncVersionChanged"),
            Self::NewMessageReceived => String::from("NewMessageReceived"),
            Self::ReceivedLikesChanged => String::from("ReceivedLikesChanged"),
            Self::ReceivedBlocksChanged => String::from("ReceivedBlocksChanged"),
            Self::SentLikesChanged => String::from("SentLikesChanged"),
            Self::SentBlocksChanged => String::from("SentBlocksChanged"),
            Self::MatchesChanged => String::from("MatchesChanged"),
            Self::LatestViewedMessageChanged => String::from("LatestViewedMessageChanged"),
            Self::ContentProcessingStateChanged => String::from("ContentProcessingStateChanged"),
            Self::AvailableProfileAttributesChanged => String::from("AvailableProfileAttributesChanged"),
        }
    }
}

impl Default for EventType {
    fn default() -> EventType {
        Self::AccountStateChanged
    }
}




