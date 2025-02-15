use axum::{extract::{Query, State}, Extension};
use model::{ReportQueryParams, UpdateReportResult};
use model_profile::{AccountIdInternal, ProfileReport, ReportProfileText, UpdateProfileReport};
use server_api::{create_open_api_router, S};
use server_data_profile::{read::GetReadProfileCommands, write::GetWriteCommandsProfile};
use simple_backend::create_counters;

use crate::{
    app::{GetAccounts, ReadData, WriteData},
    db_write,
    utils::{Json, StatusCode},
};

// TODO(prod): Remove unused report APIs

const PATH_GET_PROFILE_REPORT: &str = "/profile_api/profile_report";

/// Get profile report
#[utoipa::path(
    get,
    path = PATH_GET_PROFILE_REPORT,
    params(ReportQueryParams),
    responses(
        (status = 200, description = "Successfull.", body = ProfileReport),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error."),
    ),
    security(("access_token" = [])),
)]
pub async fn get_profile_report(
    State(state): State<S>,
    Extension(account_id): Extension<AccountIdInternal>,
    Query(report): Query<ReportQueryParams>,
) -> Result<Json<ProfileReport>, StatusCode> {
    PROFILE.get_profile_report.incr();

    let target = state.get_internal_id(report.target).await?;

    let report = state.read().profile().report().get_report(
        account_id,
        target,
    ).await?;

    Ok(report.into())
}

const PATH_POST_PROFILE_REPORT: &str = "/profile_api/profile_report";

/// Update profile report.
///
/// If profile text is reported and it is bot moderated, the text's
/// moderation state changes to
/// [model_profile::ProfileTextModerationState::WaitingHumanModeration].
#[utoipa::path(
    post,
    path = PATH_POST_PROFILE_REPORT,
    request_body = UpdateProfileReport,
    responses(
        (status = 200, description = "Successfull.", body = UpdateReportResult),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error."),
    ),
    security(("access_token" = [])),
)]
pub async fn post_profile_report(
    State(state): State<S>,
    Extension(account_id): Extension<AccountIdInternal>,
    Json(update): Json<UpdateProfileReport>,
) -> Result<Json<UpdateReportResult>, StatusCode> {
    PROFILE.post_profile_report.incr();

    let target = state.get_internal_id(update.target).await?;

    let result = db_write!(state, move |cmds| cmds
        .profile()
        .report()
        .update_report(account_id, target, update.content))?;

    Ok(result.into())
}

const PATH_POST_REPORT_PROFILE_TEXT: &str = "/profile_api/report_profile_text";

/// Report profile text
///
/// If profile text is reported and it is bot moderated, the text's
/// moderation state changes to
/// [model_profile::ProfileTextModerationState::WaitingHumanModeration].
#[utoipa::path(
    post,
    path = PATH_POST_REPORT_PROFILE_TEXT,
    request_body = ReportProfileText,
    responses(
        (status = 200, description = "Successfull.", body = UpdateReportResult),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error."),
    ),
    security(("access_token" = [])),
)]
pub async fn post_report_profile_text(
    State(state): State<S>,
    Extension(account_id): Extension<AccountIdInternal>,
    Json(update): Json<ReportProfileText>,
) -> Result<Json<UpdateReportResult>, StatusCode> {
    PROFILE.post_report_profile_text.incr();

    let target = state.get_internal_id(update.target).await?;

    let result = db_write!(state, move |cmds| cmds
        .profile()
        .report()
        .report_profile_text(account_id, target, update.profile_text))?;

    Ok(result.into())
}


create_open_api_router!(
        fn router_profile_report,
        get_profile_report,
        post_profile_report,
        post_report_profile_text,
);

create_counters!(
    ProfileCounters,
    PROFILE,
    PROFILE_REPORT_COUNTERS_LIST,
    get_profile_report,
    post_profile_report,
    post_report_profile_text,
);
