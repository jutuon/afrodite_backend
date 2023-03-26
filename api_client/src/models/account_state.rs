/*
 * pihka-backend
 *
 * Pihka backend API
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountState {
    #[serde(rename = "InitialSetup")]
    InitialSetup,
    #[serde(rename = "Normal")]
    Normal,
    #[serde(rename = "Banned")]
    Banned,
    #[serde(rename = "PendingDeletion")]
    PendingDeletion,

}

impl ToString for AccountState {
    fn to_string(&self) -> String {
        match self {
            Self::InitialSetup => String::from("InitialSetup"),
            Self::Normal => String::from("Normal"),
            Self::Banned => String::from("Banned"),
            Self::PendingDeletion => String::from("PendingDeletion"),
        }
    }
}

impl Default for AccountState {
    fn default() -> AccountState {
        Self::InitialSetup
    }
}




