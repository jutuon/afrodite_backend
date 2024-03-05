use axum::{
    extract::{Path, State},
    Extension, Router,
};
use model::{AccountId, AccountIdInternal, Profile, ProfileUpdate, ProfileUpdateInternal};
use simple_backend::create_counters;
use simple_backend_utils::IntoReportFromString;

use crate::{
    api::{
        db_write,
        utils::{Json, StatusCode},
    }, app::{GetAccessTokens, GetAccounts, GetConfig, GetInternalApi, ReadData, WriteData}, data::DataError, internal_api
};

// TODO: Add timeout for database commands

pub const PATH_GET_PROFILE: &str = "/profile_api/profile/:account_id";

// TODO: Add possibility to get profile if it is private and match wants it.

/// Get account's current profile.
///
/// Profile can include version UUID which can be used for caching.
///
/// # Access
/// Public profile access requires `view_public_profiles` capability.
/// Public and private profile access requires `admin_view_all_profiles`
/// capablility.
///
/// # Microservice notes
/// If account feature is set as external service then cached capability
/// information from account service is used for access checks.
#[utoipa::path(
    get,
    path = "/profile_api/profile/{account_id}",
    params(AccountId),
    responses(
        (status = 200, description = "Get current profile.", body = Profile),
        (status = 401, description = "Unauthorized."),
        (
            status = 500,
            description = "Profile does not exist, is private or other server error.",
        ),
    ),
    security(("access_token" = [])),
)]
pub async fn get_profile<
    S: ReadData + GetAccounts + GetAccessTokens + GetInternalApi + WriteData + GetConfig,
>(
    State(state): State<S>,
    Extension(account_id): Extension<AccountIdInternal>,
    Path(requested_profile): Path<AccountId>,
) -> Result<Json<Profile>, StatusCode> {
    PROFILE.get_profile.incr();

    // TODO: Change return type to GetProfileResult, because
    //       current style spams errors to logs.
    // TODO: check capablities so that admin can view all profiles

    let requested_profile = state.accounts().get_internal_id(requested_profile).await?;

    if account_id.as_id() == requested_profile.as_id() {
        return state
            .read()
            .profile()
            .profile(requested_profile)
            .await
            .map_err(Into::into)
            .map(|p| p.into());
    }

    let visibility = state.read().common().account(requested_profile).await?.profile_visibility().is_currently_public();

    if visibility {
        state
            .read()
            .profile()
            .profile(requested_profile)
            .await
            .map_err(Into::into)
            .map(|p| p.into())
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub const PATH_POST_PROFILE: &str = "/profile_api/profile";

/// Update profile information.
///
/// Writes the profile to the database only if it is changed.
///
/// TODO: string lenght validation, limit saving new profiles
/// TODO: return the new proifle. Edit: is this really needed?
#[utoipa::path(
    post,
    path = "/profile_api/profile",
    request_body = ProfileUpdate,
    responses(
        (status = 200, description = "Update profile"),
        (status = 401, description = "Unauthorized."),
        (
            status = 500,
            description = "Profile validation in route handler failed or database error."
        ),
    ),
    security(("access_token" = [])),
)]
pub async fn post_profile<S: GetConfig + GetAccessTokens + WriteData + ReadData>(
    State(state): State<S>,
    Extension(account_id): Extension<AccountIdInternal>,
    Json(profile): Json<ProfileUpdate>,
) -> Result<(), StatusCode> {
    PROFILE.post_profile.incr();

    let profile = profile
        .validate(state.config().profile_attributes())
        .into_error_string(DataError::NotAllowed)?;
    let old_profile = state.read().profile().profile(account_id).await?;

    if profile.equals_with(&old_profile) {
        return Ok(());
    }

    let new = ProfileUpdateInternal::new(profile);

    db_write!(state, move |cmds| cmds.profile().profile(account_id, new))
}

pub fn profile_data_router(s: crate::app::S) -> Router {
    use axum::routing::{get, post};

    use crate::app::S;

    Router::new()
        .route(PATH_GET_PROFILE, get(get_profile::<S>))
        .route(PATH_POST_PROFILE, post(post_profile::<S>))
        .with_state(s)
}

create_counters!(
    ProfileCounters,
    PROFILE,
    PROFILE_DATA_COUNTERS_LIST,
    get_profile,
    post_profile,
);
