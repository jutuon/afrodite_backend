use crate::{
    api::model::{
        Account,
        ApiKey, AccountId, AccountIdLight, Profile, AccountSetup,
    },
    utils::ErrorConversion,
};



#[derive(Debug, Clone)]
pub enum ReadCmd {
    AccountApiKey(AccountIdLight),
    AccountState(AccountIdLight),
    AccountSetup(AccountIdLight),
    Accounts,
    Profile(AccountIdLight),
}

impl std::fmt::Display for ReadCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Read command: {:?}", self))
    }
}
