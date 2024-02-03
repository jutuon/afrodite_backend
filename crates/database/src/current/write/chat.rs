use diesel::{delete, insert_into, prelude::*, update};
use error_stack::Result;
use model::{
    AccountIdInternal, AccountInteractionInternal, AccountInteractionState, PendingMessageId,
};
use simple_backend_database::diesel_db::{ConnectionProvider, DieselDatabaseError};
use simple_backend_utils::current_unix_time;

use crate::{current::read::CurrentSyncReadCommands, IntoDatabaseError, TransactionError};

mod interaction;
mod message;

define_write_commands!(CurrentWriteChat, CurrentSyncWriteChat);

impl<C: ConnectionProvider> CurrentSyncWriteChat<C> {
    pub fn interaction(self) -> interaction::CurrentSyncWriteChatInteraction<C> {
        interaction::CurrentSyncWriteChatInteraction::new(self.cmds)
    }

    pub fn message(self) -> message::CurrentSyncWriteChatMessage<C> {
        message::CurrentSyncWriteChatMessage::new(self.cmds)
    }
}
