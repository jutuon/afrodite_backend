use error_stack::{FutureExt, Result, ResultExt};
use model::{
    AccessToken, Account, AccountId, AccountIdInternal, AccountSetup, GoogleAccountId,
    RefreshToken, SignInWithInfo, EventToClient,
};
use tokio_stream::StreamExt;

use super::{
    super::{cache::DatabaseCache, file::utils::FileDir, DataError},
    ReadCommands,
};
use crate::{data::IntoDataError, event::{EventSender, EventMode}};

define_read_commands!(ReadCommandsCommon);

impl ReadCommandsCommon<'_> {
    pub async fn access_event_mode<T>(
        &self,
        id: AccountId,
        action: impl FnOnce(&EventMode) -> T,
    ) -> Result<T, DataError> {
        self
            .cache()
            .read_cache(id, move |entry| {
                action(&entry.current_event_connection)
            })
            .await
            .into_data_error(id)
    }
}
