/*
 * dating-app-backend
 *
 * Dating app backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// EventType : Identifier for event.
/// Identifier for event.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "AccountStateChanged")]
    AccountStateChanged,
    #[serde(rename = "AccountPermissionsChanged")]
    AccountPermissionsChanged,
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
    #[serde(rename = "ProfileChanged")]
    ProfileChanged,
    #[serde(rename = "NewsCountChanged")]
    NewsCountChanged,
    #[serde(rename = "InitialContentModerationCompleted")]
    InitialContentModerationCompleted,
    #[serde(rename = "MediaContentChanged")]
    MediaContentChanged,

}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AccountStateChanged => write!(f, "AccountStateChanged"),
            Self::AccountPermissionsChanged => write!(f, "AccountPermissionsChanged"),
            Self::ProfileVisibilityChanged => write!(f, "ProfileVisibilityChanged"),
            Self::AccountSyncVersionChanged => write!(f, "AccountSyncVersionChanged"),
            Self::NewMessageReceived => write!(f, "NewMessageReceived"),
            Self::ReceivedLikesChanged => write!(f, "ReceivedLikesChanged"),
            Self::ReceivedBlocksChanged => write!(f, "ReceivedBlocksChanged"),
            Self::SentLikesChanged => write!(f, "SentLikesChanged"),
            Self::SentBlocksChanged => write!(f, "SentBlocksChanged"),
            Self::MatchesChanged => write!(f, "MatchesChanged"),
            Self::LatestViewedMessageChanged => write!(f, "LatestViewedMessageChanged"),
            Self::ContentProcessingStateChanged => write!(f, "ContentProcessingStateChanged"),
            Self::AvailableProfileAttributesChanged => write!(f, "AvailableProfileAttributesChanged"),
            Self::ProfileChanged => write!(f, "ProfileChanged"),
            Self::NewsCountChanged => write!(f, "NewsCountChanged"),
            Self::InitialContentModerationCompleted => write!(f, "InitialContentModerationCompleted"),
            Self::MediaContentChanged => write!(f, "MediaContentChanged"),
        }
    }
}

impl Default for EventType {
    fn default() -> EventType {
        Self::AccountStateChanged
    }
}

