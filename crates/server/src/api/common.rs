//! Common routes to all microservices
//!

use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use axum_extra::TypedHeader;
use model::{
    AccessToken, AccountIdInternal, AccountSyncVersion, AuthPair, BackendVersion, ChatStateRaw, EventToClient, EventToClientInternal, RefreshToken, SpecialEventToClient, SyncCheckDataType, SyncCheckResult, SyncDataVersionFromClient, SyncVersionFromClient, SyncVersionUtils
};
use simple_backend::{create_counters, event, web_socket::WebSocketManager};
use simple_backend_utils::IntoReportFromString;
use tracing::{error, info};
pub use utils::api::PATH_CONNECT;

use super::{
    super::app::{BackendVersionProvider, GetAccessTokens, ReadData, WriteData},
    utils::{AccessTokenHeader, Json, StatusCode},
};
use crate::{app::GetConfig, db_write, result::{Result, WrappedContextExt, WrappedResultExt}};

pub const PATH_GET_VERSION: &str = "/common_api/version";

/// Get backend version.
#[utoipa::path(
    get,
    path = "/common_api/version",
    security(),
    responses(
        (status = 200, description = "Version information.", body = BackendVersion),
    )
)]
pub async fn get_version<S: BackendVersionProvider>(
    State(state): State<S>,
) -> Json<BackendVersion> {
    COMMON.get_version.incr();
    state.backend_version().into()
}

// TODO(prod): Check access and refresh key lenghts.

// ------------------------- WebSocket -------------------------

/// Connect to server using WebSocket after getting refresh and access tokens.
/// Connection is required as API access is allowed for connected clients.
///
/// Protocol:
/// 1. Client sends version information as Binary message, where
///    - u8: Client WebSocket protocol version (currently 0).
///    - u8: Client type number. (0 = Android, 1 = iOS, 255 = Test mode bot)
///    - u16: Client Major version.
///    - u16: Client Minor version.
///    - u16: Client Patch version.
///
///    The u16 values are in little endian byte order.
/// 2. Client sends current refresh token as Binary message.
/// 3. If server supports the client, the server sends next refresh token
///    as Binary message.
///    If server does not support the client, the server sends Text message
///    and closes the connection.
/// 4. Server sends new access token as Text message.
///    (At this point API can be used.)
/// 5. Client sends list of current data sync versions as Binary message, where
///    items are [u8; 2] and the first u8 of an item is the data type number
///    and the second u8 of an item is the sync version number for that data.
///    If client does not have any version of the data, the client should
///    send 255 as the version number.
///
///    Available data types:
///    - 0: Account
/// 6. Server starts to send JSON events as Text messages.
///
/// The new access token is valid until this WebSocket is closed.
///
#[utoipa::path(
    get,
    path = "/common_api/connect",
    responses(
        (status = 101, description = "Switching protocols."),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error. TODO: can be removed?"),
    ),
    security(("access_token" = [])),
)]
pub async fn get_connect_websocket<
    S: WriteData + ReadData + GetAccessTokens + GetConfig + Send + Sync + 'static,
>(
    State(state): State<S>,
    websocket: WebSocketUpgrade,
    TypedHeader(access_token): TypedHeader<AccessTokenHeader>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ws_manager: WebSocketManager,
) -> std::result::Result<impl IntoResponse, StatusCode> {
    COMMON.get_connect_websocket.incr();

    // NOTE: This handler does not have authentication layer enabled, so
    // authentication must be done manually.

    let id = state
        .access_tokens()
        .access_token_exists(access_token.key())
        .await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(websocket.on_upgrade(move |socket| handle_socket(socket, addr, id, state, ws_manager)))
}

async fn handle_socket<S: WriteData + ReadData + GetConfig>(
    socket: WebSocket,
    address: SocketAddr,
    id: AccountIdInternal,
    state: S,
    mut ws_manager: WebSocketManager,
) {
    let quit_lock = if let Some(quit_lock) = ws_manager.get_ongoing_ws_connection_quit_lock().await
    {
        quit_lock
    } else {
        return;
    };

    tokio::select! {
        _ = ws_manager.server_quit_detected() => {
            // TODO: Probably sessions should be ended when server quits?
            //       Test does this code path work with client.
            let result = state.write(move |cmds| async move {
                cmds.common()
                    .end_connection_session(id)
                    .await
            }).await;

            if let Err(e) = result {
                error!("server quit end_connection_session, {e:?}")
            }
        },
        r = handle_socket_result(socket, address, id, &state) => {
            match r {
                Ok(()) => {
                    let result = state.write(move |cmds| async move {
                        cmds.common()
                            .end_connection_session(id)
                            .await
                    }).await;

                    if let Err(e) = result {
                        error!("end_connection_session, {e:?}")
                    }
                },
                Err(e) => {
                    error!("handle_socket_result: {e:?}");

                    let result = state.write(move |cmds| async move {
                        cmds.common().logout(id).await
                    }).await;

                    if let Err(e) = result {
                        error!("logout, {e:?}")
                    }
                }
            }
        }
    }

    drop(quit_lock);
}

#[derive(thiserror::Error, Debug)]
pub enum WebSocketError {
    #[error("Receive error")]
    Receive,
    #[error("Client sent something unsupported")]
    ProtocolError,
    #[error("Client version is unsupported")]
    ClientVersionUnsupported,
    #[error("Received wrong refresh token")]
    ReceiveWrongRefreshToken,
    #[error("Websocket data sending error")]
    Send,
    #[error("Websocket closing failed")]
    Close,
    #[error("Data serialization error")]
    Serialize,

    // Database errors
    #[error("Database: No refresh token")]
    DatabaseNoRefreshToken,
    #[error("Invalid refresh token in database")]
    InvalidRefreshTokenInDatabase,
    #[error("Database: account logout failed")]
    DatabaseLogoutFailed,
    #[error("Database: saving new tokens failed")]
    DatabaseSaveTokens,
    #[error("Database: Account state query failed")]
    DatabaseAccountStateQuery,
    #[error("Database: Chat state query failed")]
    DatabaseChatStateQuery,
    #[error("Database: Pending messages query failed")]
    DatabasePendingMessagesQuery,

    // Event errors
    #[error("Event channel creation failed")]
    EventChannelCreationFailed,

    // Sync
    #[error("Account data version number reset failed")]
    AccountDataVersionResetFailed,
    #[error("Chat data version number reset failed")]
    ChatDataVersionResetFailed,
}

async fn handle_socket_result<S: WriteData + ReadData + GetConfig>(
    mut socket: WebSocket,
    address: SocketAddr,
    id: AccountIdInternal,
    state: &S,
) -> Result<(), WebSocketError> {

    // Receive protocol version byte.
    let client_is_supported = match socket
        .recv()
        .await
        .ok_or(WebSocketError::Receive.report())?
        .change_context(WebSocketError::Receive)?
        {
            Message::Binary(version) => {
                match version.as_slice() {
                    [0, info_bytes @ ..] => {
                        let info = model::WebSocketClientInfo::parse(info_bytes)
                            .into_error_string(WebSocketError::ProtocolError)?;
                        // TODO: remove after client is tested to work with the
                        // new protocol
                        info!("{:#?}", info);
                        // In the future there is possibility to blacklist some
                        // old client versions.
                        true
                    }
                    _ => return Err(WebSocketError::ProtocolError.report()),
                }
            }
            _ => return Err(WebSocketError::ProtocolError.report()),
        };

    let current_refresh_token = state
        .read()
        .account()
        .account_refresh_token(id)
        .await
        .change_context(WebSocketError::DatabaseNoRefreshToken)?
        .ok_or(WebSocketError::DatabaseNoRefreshToken.report())?
        .bytes()
        .change_context(WebSocketError::InvalidRefreshTokenInDatabase)?;

    // Refresh token check.
    match socket
        .recv()
        .await
        .ok_or(WebSocketError::Receive.report())?
        .change_context(WebSocketError::Receive)?
    {
        Message::Binary(refresh_token) => {
            if refresh_token != current_refresh_token {
                // Returning error does the logout, so it is not needed here.
                // For this case the logout is needed to prevent refresh
                // token quessing.
                return Err(WebSocketError::ReceiveWrongRefreshToken.report());
            }
        }
        _ => return Err(WebSocketError::ProtocolError.report()),
    };

    if !client_is_supported {
        socket
            .send(Message::Text(String::new()))
            .await
            .change_context(WebSocketError::Send)?;
        socket.close()
            .await
            .change_context(WebSocketError::Close)?;
        return Err(WebSocketError::ClientVersionUnsupported.report());
    }

    // Refresh check was successful, so the new refresh token can be sent.

    let (new_refresh_token, new_refresh_token_bytes) = RefreshToken::generate_new_with_bytes();
    let new_access_token = AccessToken::generate_new();

    socket
        .send(Message::Binary(new_refresh_token_bytes))
        .await
        .change_context(WebSocketError::Send)?;

    let new_access_token_cloned = new_access_token.clone();
    state
        .write(move |cmds| async move {
            cmds.common()
                .set_new_auth_pair(
                    id,
                    AuthPair {
                        access: new_access_token_cloned,
                        refresh: new_refresh_token,
                    },
                    Some(address),
                )
                .await
        })
        .await
        .change_context(WebSocketError::DatabaseSaveTokens)?;

    socket
        .send(Message::Text(new_access_token.into_string()))
        .await
        .change_context(WebSocketError::Send)?;

    // Receive sync data version list
    let data_sync_versions = match socket
        .recv()
        .await
        .ok_or(WebSocketError::Receive.report())?
        .change_context(WebSocketError::Receive)?
        {
            Message::Binary(sync_data_version_list) => {
                SyncDataVersionFromClient::parse_sync_data_list(&sync_data_version_list)
                    .into_error_string(WebSocketError::ProtocolError)?
            }
            _ => return Err(WebSocketError::ProtocolError.report()),
        };

    let mut event_receiver = state
        .write(
            move |cmds| async move { cmds.common().init_connection_session_events(id.uuid).await },
        )
        .await
        .change_context(WebSocketError::DatabaseSaveTokens)?;

    sync_data_with_client_if_needed(state, &mut socket, id, data_sync_versions).await?;
    send_new_messages_event_if_needed(state, &mut socket, id).await?;

    loop {
        tokio::select! {
            result = socket.recv() => {
                match result {
                    Some(Err(_)) | None => break,
                    Some(Ok(value)) => {
                        // TODO: Fix possible CPU usage bug here.
                        // Replace continue with break?
                        error!("Unexpected value: {:?}, from: {}", value, address);
                        continue;
                    },
                }
            }
            event = event_receiver.recv() => {
                match event {
                    Some(event) => {
                        let event = serde_json::to_string(&event)
                            .change_context(WebSocketError::Serialize)?;
                        socket.send(Message::Text(event))
                            .await
                            .change_context(WebSocketError::Send)?;
                    },
                    None => (),
                }
            }
        }
    }

    Ok(())
}

async fn sync_data_with_client_if_needed<S: WriteData + ReadData + GetConfig>(
    state: &S,
    socket: &mut WebSocket,
    id: AccountIdInternal,
    sync_versions: Vec<SyncDataVersionFromClient>,
) -> Result<(), WebSocketError> {
    let chat_state = state
        .read()
        .chat()
        .chat_state(id)
        .await
        .change_context(WebSocketError::DatabaseChatStateQuery)?;

    for version in sync_versions {
        match version.data_type {
            SyncCheckDataType::Account =>
                if state.config().components().account {
                    handle_account_data_sync(
                        state,
                        socket,
                        id,
                        version.version,
                    ).await?;
                }
            SyncCheckDataType::ReveivedBlocks =>
                if state.config().components().chat {
                    handle_chat_state_version_check(
                        state,
                        socket,
                        id,
                        version.version,
                        chat_state.clone(),
                        |s| &mut s.received_blocks_sync_version,
                        EventToClientInternal::ReceivedBlocksChanged,
                    ).await?;
                }
            SyncCheckDataType::ReveivedLikes =>
                if state.config().components().chat {
                    handle_chat_state_version_check(
                        state,
                        socket,
                        id,
                        version.version,
                        chat_state.clone(),
                        |s| &mut s.received_likes_sync_version,
                        EventToClientInternal::ReceivedLikesChanged,
                    ).await?;
                }
            SyncCheckDataType::SentBlocks =>
                if state.config().components().chat {
                    handle_chat_state_version_check(
                        state,
                        socket,
                        id,
                        version.version,
                        chat_state.clone(),
                        |s| &mut s.sent_blocks_sync_version,
                        EventToClientInternal::SentBlocksChanged,
                    ).await?;
                }
            SyncCheckDataType::SentLikes =>
                if state.config().components().chat {
                    handle_chat_state_version_check(
                        state,
                        socket,
                        id,
                        version.version,
                        chat_state.clone(),
                        |s| &mut s.sent_likes_sync_version,
                        EventToClientInternal::SentLikesChanged,
                    ).await?;
                }
            SyncCheckDataType::Matches =>
                if state.config().components().chat {
                    handle_chat_state_version_check(
                        state,
                        socket,
                        id,
                        version.version,
                        chat_state.clone(),
                        |s| &mut s.matches_sync_version,
                        EventToClientInternal::MatchesChanged,
                    ).await?;
                }
        }
    }

    Ok(())
}

async fn handle_account_data_sync<S: WriteData + ReadData>(
    state: &S,
    socket: &mut WebSocket,
    id: AccountIdInternal,
    sync_version: SyncVersionFromClient,
) -> Result<(), WebSocketError> {
    let account = state
        .read()
        .common()
        .account(id)
        .await
        .change_context(WebSocketError::DatabaseAccountStateQuery)?;

    let account = match account.sync_version().check_is_sync_required(sync_version) {
        SyncCheckResult::DoNothing => return Ok(()),
        SyncCheckResult::ResetVersionAndSync => {
            state.write(
                move |cmds| async move {
                    cmds.account().reset_syncable_account_data_version(id).await
                },
            )
                .await
                .change_context(WebSocketError::AccountDataVersionResetFailed)?;

            state
                .read()
                .common()
                .account(id)
                .await
                .change_context(WebSocketError::DatabaseAccountStateQuery)?
        }
        SyncCheckResult::Sync => account,
    };

    send_event(
        socket,
        EventToClientInternal::AccountStateChanged(
            account.state()
        )
    ).await?;

    send_event(
        socket,
        EventToClientInternal::AccountCapabilitiesChanged(
            account.capablities().clone()
        )
    ).await?;

    send_event(
        socket,
        EventToClientInternal::ProfileVisibilityChanged(
            account.profile_visibility()
        )
    ).await?;

    // This must be the last to make sure that client has
    // reveived all sync data.
    send_event(
        socket,
        SpecialEventToClient::AccountSyncVersionChanged(
            account.sync_version()
        )
    ).await?;

    Ok(())
}

async fn handle_chat_state_version_check<S: WriteData + ReadData, T: SyncVersionUtils>(
    state: &S,
    socket: &mut WebSocket,
    id: AccountIdInternal,
    sync_version: SyncVersionFromClient,
    mut chat_state: ChatStateRaw,
    getter: impl Fn(&mut ChatStateRaw) -> &mut T + Send + 'static,
    event: EventToClientInternal,
) -> Result<(), WebSocketError> {
    let check_this_version = getter(&mut chat_state);
    match check_this_version.check_is_sync_required(sync_version) {
        SyncCheckResult::DoNothing => return Ok(()),
        SyncCheckResult::ResetVersionAndSync => {
            state.write(
                move |cmds| async move {
                    cmds.chat().modify_chat_state(id, move |s| {
                        let version_to_be_reseted = getter(s);
                        *version_to_be_reseted = Default::default();
                    }).await
                },
            )
                .await
                .change_context(WebSocketError::ChatDataVersionResetFailed)?;
        }
        SyncCheckResult::Sync => (),
    };

    send_event(socket, event).await?;

    Ok(())
}

async fn send_event(
    socket: &mut WebSocket,
    event: impl Into<EventToClient>,
) -> Result<(), WebSocketError> {
    let event: EventToClient = event.into();
    let event = serde_json::to_string(&event).change_context(WebSocketError::Serialize)?;
    socket
        .send(Message::Text(event))
        .await
        .change_context(WebSocketError::Send)?;

    Ok(())
}

async fn send_new_messages_event_if_needed<S: WriteData + ReadData + GetConfig>(
    state: &S,
    socket: &mut WebSocket,
    id: AccountIdInternal,
) -> Result<(), WebSocketError> {
    if state.config().components().chat {
        let pending_messages = state
            .read()
            .chat()
            .all_pending_messages(id)
            .await
            .change_context(WebSocketError::DatabasePendingMessagesQuery)?;

        if !pending_messages.messages.is_empty() {
            send_event(
                socket,
                EventToClientInternal::NewMessageReceived
            ).await?;
        }
    }

    Ok(())
}

create_counters!(
    CommonCounters,
    COMMON,
    COMMON_COUNTERS_LIST,
    get_version,
    get_connect_websocket,
);
