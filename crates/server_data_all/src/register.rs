
use std::{ops::DerefMut, sync::Arc};

use config::Config;
use database::{
    ConnectionProvider,
    current::{
        write::TransactionConnection,
    }, PoolObject, TransactionError
};
use model::{
    Account, AccountId, AccountIdInternal, AccountInternal, AccountSetup, EmailAddress, Profile,
    SharedStateRaw, SignInWithInfo,
};

use crate::load::DbDataToCacheLoader;

use server_data::{
    index::{LocationIndexIteratorHandle, LocationIndexWriteHandle}, write::{WriteCommandsProvider}, IntoDataError
};
use server_data::{result::Result, DataError};


pub struct RegisterAccount<C: WriteCommandsProvider> {
    cmds: C,
}

impl <C: WriteCommandsProvider> RegisterAccount<C> {
    pub fn new(cmds: C) -> Self {
        Self { cmds }
    }

    pub async fn register(
        &self,
        id_light: AccountId,
        sign_in_with_info: SignInWithInfo,
        email: Option<EmailAddress>,
    ) -> Result<AccountIdInternal, DataError> {
        let config = self.cmds.write_cmds().config.clone();
        let id: AccountIdInternal = self.cmds.write_cmds()
            .db_transaction_with_history(move |transaction, history_conn| {
                Self::register_db_action(
                    config,
                    id_light,
                    sign_in_with_info,
                    email,
                    transaction,
                    history_conn,
                )
            })
            .await?;

            DbDataToCacheLoader::load_account_from_db(
                self.cmds.write_cmds().cache,
                id,
                self.cmds.write_cmds().config,
                &self.cmds.write_cmds().current_write_handle.to_read_handle(),
                LocationIndexIteratorHandle::new(self.cmds.write_cmds().location_index),
                LocationIndexWriteHandle::new(self.cmds.write_cmds().location_index),
            )
            .await
            .into_data_error(id)?;

        Ok(id)
    }

    pub fn register_db_action(
        config: Arc<Config>,
        id_light: AccountId,
        sign_in_with_info: SignInWithInfo,
        email: Option<EmailAddress>,
        mut transaction: TransactionConnection<'_>,
        mut history_conn: PoolObject,
    ) -> std::result::Result<AccountIdInternal, TransactionError> {
        let account = Account::default();
        let account_setup = AccountSetup::default();

        let mut conn = &mut transaction;

        // No transaction for history as it does not matter if some default
        // data will be left there if there is some error.
        let mut history = database_account::history::write::HistorySyncWriteCommands::new(history_conn.as_mut());

        // Common
        let mut current = database::current::write::CurrentSyncWriteCommands::new(conn.conn());
        let id = current.common().insert_account_id(id_light)?;
        current.common().token().insert_access_token(id, None)?;
        current.common().token().insert_refresh_token(id, None)?;
        current
            .common()
            .state()
            .insert_default_account_capabilities(id)?;
        current
            .common()
            .state()
            .insert_shared_state(id, SharedStateRaw::default())?;

        // Common history
        history.account().insert_account_id(id)?;

        if config.components().account {
            let mut current = database_account::current::write::CurrentSyncWriteCommands::new(conn.conn());
            current
                .account()
                .data()
                .insert_account(id, AccountInternal::default())?;
            current
                .account()
                .data()
                .insert_account_setup(id, &account_setup)?;
            current
                .account()
                .sign_in_with()
                .insert_sign_in_with_info(id, &sign_in_with_info)?;
            if let Some(email) = email {
                current.account().data().update_account_email(id, &email)?;
            }

            // Account history
            history.account().insert_account(id, &account)?;
            history.account().insert_account_setup(id, &account_setup)?;
        }

        if config.components().profile {
            let mut current = database_profile::current::write::CurrentSyncWriteCommands::new(conn.conn());
            let mut history = database_profile::history::write::HistorySyncWriteCommands::new(history_conn.deref_mut());
            let profile = current.profile().data().insert_profile(id)?;
            current.profile().data().insert_profile_state(id)?;

            // Profile history
            let attributes = current
                .read()
                .profile()
                .data()
                .profile_attribute_values(id)?;
            let profile = Profile::new(profile, attributes);
            history.profile().insert_profile(id, &profile)?;
        }

        if config.components().media {
            let mut current = database_media::current::write::CurrentSyncWriteCommands::new(conn.conn());
            current.media().insert_media_state(id)?;

            current
                .media()
                .media_content()
                .insert_current_account_media(id)?;
        }

        if config.components().chat {
            let mut current = database_chat::current::write::CurrentSyncWriteCommands::new(conn.conn());
            current.chat().insert_chat_state(id)?;
        }

        Ok(id)
    }
}
