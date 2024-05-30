use database::{define_current_write_commands, DieselDatabaseError};
use diesel::{delete, insert_into, prelude::*, ExpressionMethods};
use error_stack::Result;
use model::AccountIdInternal;
use simple_backend_utils::current_unix_time;

use super::ConnectionProvider;
use crate::IntoDatabaseError;

define_current_write_commands!(CurrentWriteProfileFavorite, CurrentSyncWriteProfileFavorite);

impl<C: ConnectionProvider> CurrentSyncWriteProfileFavorite<C> {
    pub fn insert_favorite_profile(
        &mut self,
        id: AccountIdInternal,
        favorite: AccountIdInternal,
    ) -> Result<(), DieselDatabaseError> {
        use model::schema::favorite_profile::dsl::*;

        let time = current_unix_time();

        insert_into(favorite_profile)
            .values((
                account_id.eq(id.as_db_id()),
                favorite_account_id.eq(favorite.as_db_id()),
                unix_time.eq(time),
            ))
            .execute(self.conn())
            .into_db_error(id)?;

        Ok(())
    }

    pub fn remove_favorite_profile(
        &mut self,
        id: AccountIdInternal,
        favorite: AccountIdInternal,
    ) -> Result<(), DieselDatabaseError> {
        use model::schema::favorite_profile::dsl::*;

        delete(favorite_profile)
            .filter(account_id.eq(id.as_db_id()))
            .filter(favorite_account_id.eq(favorite.as_db_id()))
            .execute(self.conn())
            .into_db_error(id)?;

        Ok(())
    }
}
