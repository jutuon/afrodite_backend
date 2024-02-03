use diesel::{insert_into, prelude::*, update};
use error_stack::{Result, ResultExt};
use model::{
    AccessToken, AccountId, AccountIdDb, AccountIdInternal, AccountInternal, AccountSetup,
    RefreshToken, SignInWithInfo,
};
use simple_backend_database::diesel_db::{ConnectionProvider, DieselDatabaseError};

use crate::IntoDatabaseError;

mod data;
mod sign_in_with;
mod token;

define_write_commands!(CurrentWriteAccount, CurrentSyncWriteAccount);

impl<C: ConnectionProvider> CurrentSyncWriteAccount<C> {
    pub fn data(self) -> data::CurrentSyncWriteAccountData<C> {
        data::CurrentSyncWriteAccountData::new(self.cmds)
    }

    pub fn sign_in_with(self) -> sign_in_with::CurrentSyncWriteAccountSignInWith<C> {
        sign_in_with::CurrentSyncWriteAccountSignInWith::new(self.cmds)
    }

    pub fn token(self) -> token::CurrentSyncWriteAccountToken<C> {
        token::CurrentSyncWriteAccountToken::new(self.cmds)
    }
}
