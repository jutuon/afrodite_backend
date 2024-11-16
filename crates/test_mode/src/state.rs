//! Save and load state
//!

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StateData {
    pub test_name: String,
    pub bot_states: Vec<BotPersistentState>,
}

impl StateData {
    pub fn find_matching(&self, task: u32, bot: u32) -> Option<&BotPersistentState> {
        self.bot_states
            .iter()
            .find(|s| s.task == task && s.bot == bot)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotPersistentState {
    pub account_id: simple_backend_utils::UuidBase64Url,
    pub task: u32,
    pub bot: u32,
}
