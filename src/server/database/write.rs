use error_stack::Result;
use serde::Serialize;

use crate::{
    api::model::{Account, AccountIdLight, AccountSetup, ApiKey, Profile},
    config::Config,
    server::database::{sqlite::SqliteWriteHandle, DatabaseError},
    utils::ErrorConversion,
};

use super::{
    current::write::SqliteWriteCommands,
    history::write::HistoryWriteCommands,
    sqlite::{HistoryUpdateJson, SqliteUpdateJson},
    utils::GetReadWriteCmd,
};

#[derive(Debug, Clone)]
pub enum WriteCmd {
    AccountId(AccountIdLight),
    Profile(AccountIdLight),
    ApiKey(AccountIdLight),
    AccountState(AccountIdLight),
    AccountSetup(AccountIdLight),
}

impl std::fmt::Display for WriteCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Write command: {:?}", self))
    }
}

pub struct WriteCommands {
    sqlite_database_write: SqliteWriteHandle,
    history_write: SqliteWriteHandle,
    id: AccountIdLight,
}

impl WriteCommands {
    pub fn new(
        id: AccountIdLight,
        sqlite_database_write: SqliteWriteHandle,
        history_write: SqliteWriteHandle,
    ) -> Self {
        Self {
            id,
            sqlite_database_write,
            history_write,
        }
    }

    pub async fn register(
        &mut self,
        id: AccountIdLight,
        config: &Config,
    ) -> Result<(), DatabaseError> {
        let account_state = Account::default();
        let account_setup = AccountSetup::default();
        let profile = Profile::default();

        self.current()
            .store_account_id(id)
            .await
            .with_info_lazy(|| WriteCmd::AccountId(id))?;

        if config.components().account {
            self.current()
                .store_account(id, &account_state)
                .await
                .with_write_cmd_info::<Account>(id)?;

            self.current()
                .store_account_setup(id, &account_setup)
                .await
                .with_write_cmd_info::<AccountSetup>(id)?;
        }

        if config.components().profile {
            self.current()
                .store_profile(id, &profile)
                .await
                .with_write_cmd_info::<Profile>(id)?;
        }

        Ok(())
    }

    pub async fn update_current_api_key(&mut self, _key: &ApiKey) -> Result<(), DatabaseError> {
        todo!("add to api key to database")
    }

    pub(super) fn current(&self) -> SqliteWriteCommands {
        SqliteWriteCommands::new(&self.sqlite_database_write)
    }

    pub(super) fn history(&self) -> HistoryWriteCommands {
        HistoryWriteCommands::new(&self.sqlite_database_write)
    }

    pub async fn update_json<
        T: GetReadWriteCmd + Serialize + Clone + Send + SqliteUpdateJson + HistoryUpdateJson + 'static,
    >(
        &mut self,
        data: &T,
    ) -> Result<(), DatabaseError> {
        data.update_json(self.id, &self.current())
            .await
            .with_write_cmd_info::<T>(self.id)
    }
}
