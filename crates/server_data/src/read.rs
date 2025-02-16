use common_admin::ReadCommandsCommonAdmin;

use self::common::ReadCommandsCommon;
use crate::db_manager::{InternalReading, ReadAccessProvider};

pub mod common;
mod common_admin;

pub trait GetReadCommandsCommon<'a> {
    fn common(self) -> ReadCommandsCommon<'a>;
    fn common_admin(self) -> ReadCommandsCommonAdmin<'a>;
}

impl<'a, C: ReadAccessProvider<'a>> GetReadCommandsCommon<'a> for C {
    fn common(self) -> ReadCommandsCommon<'a> {
        ReadCommandsCommon::new(self.handle())
    }
    fn common_admin(self) -> ReadCommandsCommonAdmin<'a> {
        ReadCommandsCommonAdmin::new(self.handle())
    }
}

pub trait DbRead {
    async fn db_read<
        T: FnOnce(
                database::DbReadMode<'_>,
            ) -> error_stack::Result<R, database::DieselDatabaseError>
            + Send
            + 'static,
        R: Send + 'static,
    >(
        &self,
        cmd: T,
    ) -> error_stack::Result<R, database::DieselDatabaseError>;
}

impl<I: InternalReading> DbRead for I {
    async fn db_read<
        T: FnOnce(
                database::DbReadMode<'_>,
            ) -> error_stack::Result<R, database::DieselDatabaseError>
            + Send
            + 'static,
        R: Send + 'static,
    >(
        &self,
        cmd: T,
    ) -> error_stack::Result<R, database::DieselDatabaseError> {
        self.db_read_raw(cmd).await
    }
}
