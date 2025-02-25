//! Start and stop backend

use std::process::ExitStatus;

use error_stack::{Result, ResultExt};
use tokio::process::Command;
use tracing::warn;

use manager_config::Config;

#[derive(thiserror::Error, Debug)]
pub enum ControllerError {
    #[error("Process start failed")]
    ProcessStartFailed,

    #[error("Process wait failed")]
    ProcessWaitFailed,

    #[error("Process stdin writing failed")]
    ProcessStdinFailed,

    #[error("Command failed with exit status: {0}")]
    CommandFailed(ExitStatus),
}

pub struct BackendController<'a> {
    config: &'a Config,
}

impl<'a> BackendController<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub async fn start_backend(&self) -> Result<(), ControllerError> {
        let script = self.config.script_locations().start_backend();

        if !script.exists() {
            warn!("Script for starting the backend does not exist");
            return Ok(());
        }

        let status = Command::new("sudo")
            .arg(script)
            .status()
            .await
            .change_context(ControllerError::ProcessWaitFailed)?;

        if !status.success() {
            tracing::error!("Start backend failed with status: {:?}", status);
            return Err(ControllerError::CommandFailed(status).into());
        }

        Ok(())
    }

    pub async fn stop_backend(&self) -> Result<(), ControllerError> {
        let script = self.config.script_locations().stop_backend();

        if !script.exists() {
            warn!("Script for stopping the backend does not exist");
            return Ok(());
        }

        let status = Command::new("sudo")
            .arg(script)
            .status()
            .await
            .change_context(ControllerError::ProcessWaitFailed)?;

        if !status.success() {
            tracing::error!("Start backend failed with status: {:?}", status);
            return Err(ControllerError::CommandFailed(status).into());
        }

        Ok(())
    }
}
