use std::{collections::HashMap, marker};

use tokio_stream::StreamExt;
use error_stack::{Result, ResultExt};

use crate::{
    api::core::{user::{UserId, ApiKey}, profile::Profile},
    server::database::{}, utils::{ErrorResultExt, ErrorConversion}
};

use super::{git::{util::{GitUserDirPath, DatabasePath}, GitDatabaseOperationHandle, read::GitDatabaseReadCommands, GitDatabase, write::GitDatabaseWriteCommands, file::CoreFileNoHistory}, sqlite::{SqliteReadHandle, SqliteWriteHandle, read::SqliteReadCommands}, DatabaseError};
use crate::utils::IntoReportExt;


#[derive(Debug, Clone)]
pub enum ReadCmd {
    UserApiKey(UserId),
    Users,
    UserProfile(UserId),
}

impl std::fmt::Display for ReadCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Read command: {:?}", self))
    }
}


pub struct ReadCommands<'a> {
    git_repositories: &'a DatabasePath,
    sqlite: SqliteReadCommands<'a>,
}

impl <'a> ReadCommands<'a> {
    pub fn new(
        git_repositories: &'a DatabasePath,
        sqlite: &'a SqliteReadHandle,
    ) -> Self {
        Self {
            git_repositories,
            sqlite: SqliteReadCommands::new(sqlite),
        }
    }

    pub async fn user_api_key(&self, user_id: &UserId) -> Result<Option<ApiKey>, DatabaseError> {
        self.git(user_id).api_key().await
            .into_db_error_with_info_lazy(|| ReadCmd::UserApiKey(user_id.clone()))
    }

    pub async fn users<T: FnMut(UserId)>(&self, mut handler: T) -> Result<(), DatabaseError> {
        let mut users = self.sqlite().users();
        while let Some(user_id) = users.try_next().await
            .into_db_error_with_info(ReadCmd::Users)? {
            handler(user_id)
        }

        Ok(())
    }

    pub async fn user_profile(&self, user_id: &UserId) -> Result<Profile, DatabaseError> {
        self.sqlite().user_profile(user_id).await
            .into_db_error_with_info_lazy(|| ReadCmd::UserProfile(user_id.clone()))
    }

    pub(super) fn git(&self, user_id: &UserId) -> GitDatabaseReadCommands {
        self.git_repositories.user_git_dir(user_id).read()
    }

    pub(super) fn sqlite(&self) -> &SqliteReadCommands {
        &self.sqlite
    }
}
