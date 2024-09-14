use axum::{extract::State, Extension, Router};
use model::{AccountIdInternal, Location};
use obfuscate_api_macro::obfuscate_api;
use server_data_profile::{read::GetReadProfileCommands, write::GetWriteCommandsProfile};
use simple_backend::create_counters;

use crate::{
    app::{GetAccessTokens, ReadData, StateBase, WriteData},
    db_write,
    utils::{Json, StatusCode},
};

#[obfuscate_api]
const PATH_GET_LOCATION: &str = "/profile_api/location";

/// Get location for account which makes this request.
#[utoipa::path(
    get,
    path = PATH_GET_LOCATION,
    responses(
        (status = 200, description = "Get successfull.", body = Location),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error."),
    ),
    security(("access_token" = [])),
)]
pub async fn get_location<S: GetAccessTokens + ReadData>(
    State(state): State<S>,
    Extension(account_id): Extension<AccountIdInternal>,
) -> Result<Json<Location>, StatusCode> {
    PROFILE.get_location.incr();

    let location = state.read().profile().profile_location(account_id).await?;
    Ok(location.into())
}

#[obfuscate_api]
const PATH_PUT_LOCATION: &str = "/profile_api/location";

/// Update location for account which makes this request.
#[utoipa::path(
    put,
    path = PATH_PUT_LOCATION,
    request_body = Location,
    responses(
        (status = 200, description = "Update successfull."),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error."),
    ),
    security(("access_token" = [])),
)]
pub async fn put_location<S: GetAccessTokens + WriteData>(
    State(state): State<S>,
    Extension(account_id): Extension<AccountIdInternal>,
    Json(location): Json<Location>,
) -> Result<(), StatusCode> {
    PROFILE.put_location.incr();

    db_write!(state, move |cmds| cmds
        .profile()
        .profile_update_location(account_id, location))
}

pub fn location_router<S: StateBase + GetAccessTokens + WriteData + ReadData>(s: S) -> Router {
    use axum::routing::{get, put};

    Router::new()
        .route(PATH_GET_LOCATION_AXUM, get(get_location::<S>))
        .route(PATH_PUT_LOCATION_AXUM, put(put_location::<S>))
        .with_state(s)
}

create_counters!(
    ProfileCounters,
    PROFILE,
    PROFILE_LOCATION_COUNTERS_LIST,
    get_location,
    put_location,
);
