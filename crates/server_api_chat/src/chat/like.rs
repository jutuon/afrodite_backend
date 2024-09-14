use axum::{extract::State, Extension, Router};
use model::{AccountId, AccountIdInternal, LimitedActionResult, LimitedActionStatus, ReceivedLikesPage, SentLikesPage};
use obfuscate_api_macro::obfuscate_api;
use server_data_chat::{read::GetReadChatCommands, write::GetWriteCommandsChat};
use simple_backend::create_counters;

use super::super::utils::{Json, StatusCode};
use crate::{
    app::{GetAccounts, ReadData, StateBase, WriteData},
    db_write_multiple,
};

#[obfuscate_api]
const PATH_POST_SEND_LIKE: &str = "/chat_api/send_like";

/// Send a like to some account. If both will like each other, then
/// the accounts will be a match.
#[utoipa::path(
    post,
    path = PATH_POST_SEND_LIKE,
    request_body(content = AccountId),
    responses(
        (status = 200, description = "Success.", body = LimitedActionResult),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error."),
    ),
    security(("access_token" = [])),
)]
pub async fn post_send_like<S: GetAccounts + WriteData>(
    State(state): State<S>,
    Extension(id): Extension<AccountIdInternal>,
    Json(requested_profile): Json<AccountId>,
) -> Result<Json<LimitedActionResult>, StatusCode> {
    CHAT.post_send_like.incr();

    // TODO(prod): Check is profile public and is age ok.

    let requested_profile = state.get_internal_id(requested_profile).await?;

    let action_status = db_write_multiple!(state, move |cmds| {
        let unlimited_likes_enabled_for_both = cmds
            .read()
            .chat()
            .unlimited_likes_are_enabled_for_both(id, requested_profile)
            .await?;

        let allow_action = if unlimited_likes_enabled_for_both {
            true
        } else {
            cmds
                .chat()
                .modify_chat_limits(id, |limits| limits.like_limit.is_limit_not_reached())
                .await??
        };

        if allow_action {
            let changes = cmds
                .chat()
                .like_or_match_profile(id, requested_profile)
                .await?;
            cmds.events()
                .handle_chat_state_changes(changes.sender)
                .await?;
            cmds.events()
                .handle_chat_state_changes(changes.receiver)
                .await?;
        }

        if unlimited_likes_enabled_for_both {
            Ok(LimitedActionStatus::Success)
        } else {
            let status = cmds
                .chat()
                .modify_chat_limits(id, |limits| limits.like_limit.increment_if_possible())
                .await??
                .to_action_status();
            Ok(status)
        }
    })?;

    Ok(LimitedActionResult { status: action_status }.into())
}

#[obfuscate_api]
const PATH_GET_SENT_LIKES: &str = "/chat_api/sent_likes";

/// Get sent likes.
///
/// Profile will not be returned if:
///
/// - Profile is hidden (not public)
/// - Profile is blocked
/// - Profile is a match
#[utoipa::path(
    get,
    path = PATH_GET_SENT_LIKES,
    responses(
        (status = 200, description = "Success.", body = SentLikesPage),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error."),
    ),
    security(("access_token" = [])),
)]
pub async fn get_sent_likes<S: ReadData>(
    State(state): State<S>,
    Extension(id): Extension<AccountIdInternal>,
) -> Result<Json<SentLikesPage>, StatusCode> {
    CHAT.get_sent_likes.incr();

    let page = state.read().chat().all_sent_likes(id).await?;
    Ok(page.into())
}

// TODO(prod): Add pagination to chat related lists. Pagination
// iterator shows latest items first. Some ID key will be used as a
// iterator starting point so that newer items do not make older items
// to appear again in next pages. The sync version will be changed to
// invalidate the pagination iterator.

// TODO(prod): Remove received blocks from API and make liking and message
// sending to seem like it succeeded even if the profile owner has blocked
// you.

// TODO(prod): Encryption public key management for chats.

// TODO(prod): Add endless likes support. If user has enabled endless likes, it
// will be possible to send likes without any limits to those users who also
// have the same setting enabled. Profile needs info are endless likes enabled.

// TODO(prod): Limit likes so that only one normal like can be sent per day.

// TODO(prod): Add profile last seen time to profiles.

// TODO(prod): Store date and time when account was created.

#[obfuscate_api]
const PATH_GET_RECEIVED_LIKES: &str = "/chat_api/received_likes";

/// Get received likes.
///
/// Profile will not be returned if:
/// - Profile is blocked
/// - Profile is a match
#[utoipa::path(
    get,
    path = PATH_GET_RECEIVED_LIKES,
    responses(
        (status = 200, description = "Success.", body = ReceivedLikesPage),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error."),
    ),
    security(("access_token" = [])),
)]
pub async fn get_received_likes<S: ReadData>(
    State(state): State<S>,
    Extension(id): Extension<AccountIdInternal>,
) -> Result<Json<ReceivedLikesPage>, StatusCode> {
    CHAT.get_received_likes.incr();

    let page = state.read().chat().all_received_likes(id).await?;
    Ok(page.into())
}

#[obfuscate_api]
const PATH_DELETE_LIKE: &str = "/chat_api/delete_like";

/// Delete sent like.
///
/// Delete will not work if profile is a match.
#[utoipa::path(
    delete,
    path = PATH_DELETE_LIKE,
    request_body(content = AccountId),
    responses(
        (status = 200, description = "Success.", body = LimitedActionResult),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error."),
    ),
    security(("access_token" = [])),
)]
pub async fn delete_like<S: GetAccounts + WriteData>(
    State(state): State<S>,
    Extension(id): Extension<AccountIdInternal>,
    Json(requested_profile): Json<AccountId>,
) -> Result<Json<LimitedActionResult>, StatusCode> {
    CHAT.delete_like.incr();

    let requested_profile = state.get_internal_id(requested_profile).await?;

    let action_status = db_write_multiple!(state, move |cmds| {
        let allow_action = cmds
            .chat()
            .modify_chat_limits(id, |limits| limits.delete_like_limit.is_limit_not_reached())
            .await??;

        if allow_action {
            let changes = cmds
                .chat()
                .delete_like_or_block(id, requested_profile)
                .await?;
            cmds.events()
                .handle_chat_state_changes(changes.sender)
                .await?;
            cmds.events()
                .handle_chat_state_changes(changes.receiver)
                .await?;
        }

        let status = cmds
            .chat()
            .modify_chat_limits(id, |limits| limits.delete_like_limit.increment_if_possible())
            .await??
            .to_action_status();
        Ok(status)
    })?;

    Ok(LimitedActionResult { status: action_status }.into())
}

pub fn like_router<S: StateBase + GetAccounts + WriteData + ReadData>(s: S) -> Router {
    use axum::routing::{delete, get, post};

    Router::new()
        .route(PATH_POST_SEND_LIKE_AXUM, post(post_send_like::<S>))
        .route(PATH_GET_SENT_LIKES_AXUM, get(get_sent_likes::<S>))
        .route(PATH_GET_RECEIVED_LIKES_AXUM, get(get_received_likes::<S>))
        .route(PATH_DELETE_LIKE_AXUM, delete(delete_like::<S>))
        .with_state(s)
}

create_counters!(
    ChatCounters,
    CHAT,
    CHAT_LIKE_COUNTERS_LIST,
    post_send_like,
    get_sent_likes,
    get_received_likes,
    delete_like,
);
