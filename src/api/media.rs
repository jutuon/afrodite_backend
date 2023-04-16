pub mod data;
pub mod internal;

use axum::response::{Response, IntoResponse};
use axum::{Json, TypedHeader};
use axum::body::{Bytes, StreamBody};
use axum::extract::{Path, BodyStream};

use headers::{Header, HeaderName, ContentType};
use hyper::StatusCode;

use tokio::stream;
use tokio_stream::Stream;
use tracing::error;

use crate::server::database::file::file::ImageSlot;

use self::super::model::SlotId;

use self::data::{ImageFileName, NewModerationRequest, ContentId, ModerationRequest, ModerationList};

use super::utils::ApiKeyHeader;
use super::{ReadDatabase, GetApiKeys, WriteDatabase};
use super::model::AccountIdLight;

pub const PATH_GET_IMAGE: &str = "/media_api/image/:account_id/:image_file";

/// Get profile image
#[utoipa::path(
    get,
    path = "/media_api/image/{account_id}/{image_file}",
    responses(
        (status = 200, description = "Get image file.", content_type = "image/jpeg"),
        (status = 401, description = "Unauthorized."),
        (status = 500),
    ),
    security(("api_key" = [])),
)]
pub async fn get_image<S: ReadDatabase>(
    Path(account_id): Path<AccountIdLight>,
    Path(content_id): Path<ContentId>,
    state: S,
) -> Result<(TypedHeader<ContentType>, Vec<u8>), StatusCode> {
    // TODO: Add access restrictions.

    // TODO: Change to use stream when error handling is improved in future axum
    // version. Or check will the connection be closed if there is an error. And
    // set content lenght? Or use ServeFile service from tower middleware.

    let data = state.read_database().image(account_id, content_id).await.map_err(|e| {
        error!("{}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((TypedHeader(ContentType::jpeg()), data))
}

pub const PATH_MODERATION_REQUEST: &str = "/media_api/moderation/request";

/// Get current moderation request.
///
#[utoipa::path(
    get,
    path = "/media_api/moderation/request",
    responses(
        (status = 200, description = "Get moderation request was successfull.", body = NewModerationRequest),
        (status = 304, description = "No moderation request found."),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error."),
    ),
    security(("api_key" = [])),
)]
pub async fn get_moderation_request<S: ReadDatabase + GetApiKeys>(
    TypedHeader(api_key): TypedHeader<ApiKeyHeader>,
    state: S,
) -> Result<NewModerationRequest, StatusCode> {
    let account_id = state
        .api_keys()
        .api_key_exists(api_key.key())
        .await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let request = state.read_database().moderation_request(account_id).await.map_err(|e| {
        error!("{}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
        .ok_or(StatusCode::NOT_MODIFIED)?;

    Ok(request)
}

/// Create new or override old moderation request.
///
/// Make sure that moderation request has content IDs which points to your own
/// image slots.
///
#[utoipa::path(
    put,
    path = "/media_api/moderation/request",
    request_body(content = NewModerationRequest),
    responses(
        (status = 200, description = "Sending or updating new image moderation request was successfull."),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error or request content was invalid."),
    ),
    security(("api_key" = [])),
)]
pub async fn put_moderation_request<S: WriteDatabase + GetApiKeys>(
    TypedHeader(api_key): TypedHeader<ApiKeyHeader>,
    Json(moderation_request): Json<NewModerationRequest>,
    state: S,
) -> Result<(), StatusCode> {
    let account_id = state
        .api_keys()
        .api_key_exists(api_key.key())
        .await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    state.write_database(account_id.as_light()).lock().await.set_moderation_request(account_id, moderation_request).await.map_err(|e| {
        error!("{}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

pub const PATH_MODERATION_REQUEST_SLOT: &str = "/media_api/moderation/request/slot/:slot_id";

/// Set image to moderation request slot.
///
/// Slots from 0 to 2 are available.
///
/// TODO: resize and check images at some point
///
#[utoipa::path(
    put,
    path = "/media_api/moderation/request/slot/{slot_id}",
    request_body(content = String, content_type = "image/jpeg"),
    responses(
        (status = 200, description = "Sending or updating new image moderation request was successfull.", body = ContentId),
        (status = 401, description = "Unauthorized."),
        (status = 406, description = "Unknown slot ID."),
        (status = 500, description = "Internal server error."),
    ),
    security(("api_key" = [])),
)]
pub async fn put_image_to_moderation_slot<S: GetApiKeys + WriteDatabase>(
    TypedHeader(api_key): TypedHeader<ApiKeyHeader>,
    Path(slot_number): Path<u8>,
    image: BodyStream,
    state: S,
) -> Result<Json<ContentId>, StatusCode> {
    let account_id = state
        .api_keys()
        .api_key_exists(api_key.key())
        .await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let slot = match slot_number {
        0 => ImageSlot::Image1,
        1 => ImageSlot::Image2,
        2 => ImageSlot::Image3,
        _ => return Err(StatusCode::NOT_ACCEPTABLE),
    };

    let content_id = state.write_database(account_id.as_light()).lock().await
        .save_to_slot(account_id, slot, image)
        .await
        .map_err(|e| {
            error!("Error: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(content_id.into())
}

pub const PATH_ADMIN_MODERATION_PAGE_NEXT: &str =
    "/media_api/admin/moderation/page/next";

/// Get current list of moderation requests in my moderation queue.
/// Additional requests will be added to my queue if necessary.
///
/// ## Access
///
/// Account with `admin_moderate_images` capability is required to access this
/// route.
///
#[utoipa::path(
    patch,
    path = "/media_api/admin/moderation/page/next",
    responses(
        (status = 200, description = "Get moderation request list was successfull.", body = ModerationList),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error."),
    ),
    security(("api_key" = [])),
)]
pub async fn patch_moderation_request_list<S: WriteDatabase + GetApiKeys>(
    TypedHeader(api_key): TypedHeader<ApiKeyHeader>,
    state: S,
) -> Result<Json<ModerationList>, StatusCode> {
    let account_id = state
        .api_keys()
        .api_key_exists(api_key.key())
        .await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let data = state.write_database(account_id.as_light()).lock().await
        .moderation_get_list_and_create_new_if_necessary(account_id)
        .await
        .map_err(|e| {
            error!("{}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(ModerationList { list: data}.into())
}


pub const PATH_ADMIN_MODERATION_HANDLE_REQUEST: &str =
    "/media_api/admin/moderation/handle_request/:request_id";

/// Handle moderation request.
///
/// ## Access
///
/// Account with `admin_moderate_images` capability is required to access this
/// route.
///
#[utoipa::path(
    post,
    path = "/media_api/admin/moderation/handle_request/{request_id}",
    request_body(content = HandleModerationRequest),
    responses(
        (status = 200, description = "Handling moderation request was successfull."),
        (status = 401, description = "Unauthorized."),
        (status = 404, description = "Request ID does not exists."),
        (status = 406, description = "Already handled."),
        (status = 500, description = "Internal server error."),
    ),
    security(("api_key" = [])),
)]
pub async fn post_handle_moderation_request<S: ReadDatabase>(
    Path(request_id): Path<uuid::Uuid>,
    Json(moderation_request): Json<NewModerationRequest>,
    _state: S,
) -> Result<(), StatusCode> {
    Err(StatusCode::NOT_ACCEPTABLE)
}
