use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use reqwest::Client;
use tokio::{
    select,
    sync::{mpsc, watch},
    time::sleep,
};

use error_stack::Result;

use tracing::{error, log::warn};

use crate::{
    api::model::{AccountId, ApiKey, Profile, AccountIdLight},
    client::{ApiClient, HttpRequestError, PublicApiUrls},
    config::args::{Test, TestMode},
};

static COUNTERS: Counters = Counters::new();

pub struct Bot {
    bot_id: u32,
    id: Option<AccountIdLight>,
    config: Arc<TestMode>,
    api: ApiClient,
    _bot_running_handle: mpsc::Sender<()>,
}

impl Bot {
    pub fn spawn(
        bot_id: u32,
        urls: Arc<PublicApiUrls>,
        config: Arc<TestMode>,
        id: impl Into<Option<AccountIdLight>>,
        bot_quit_receiver: watch::Receiver<()>,
        _bot_running_handle: mpsc::Sender<()>,
    ) {
        let bot = Self {
            bot_id,
            id: id.into(),
            config,
            api: ApiClient::new(Client::new(), urls),
            _bot_running_handle,
        };

        tokio::spawn(bot.run(bot_quit_receiver));
    }

    pub async fn run(self, mut bot_quit_receiver: watch::Receiver<()>) {
        loop {
            select! {
                result = bot_quit_receiver.changed() => {
                    if result.is_err() {
                        break
                    }
                }
                result = self.run_bot() => {
                    if let Err(e) = result {
                        error!("Bot {} returned error: {:?}", self.bot_id, e);
                    }
                    break;
                }
            }
        }
    }

    async fn run_bot(&self) -> Result<(), HttpRequestError> {
        let id = if let Some(id) = self.id.as_ref() {
            id.clone()
        } else {
            self.api.account().register().await?
        };

        let key = self.api.account().login(id).await?;

        let mut update_profile_timer = Timer::new(Duration::from_millis(1000));
        let mut print_info_timer = Timer::new(Duration::from_millis(1000));

        loop {
            self.run_normal_test(
                id,
                &key,
                &mut update_profile_timer,
                self.config.print_speed && print_info_timer.passed() && self.bot_id == 1,
            )
            .await?;

            if !self.config.forever {
                break;
            }
        }

        Ok(())
    }

    async fn run_normal_test(
        &self,
        id: AccountIdLight,
        key: &ApiKey,
        update_profile_timer: &mut Timer,
        print_info: bool,
    ) -> Result<(), HttpRequestError> {
        if !self.config.no_sleep {
            sleep(Duration::from_millis(1000)).await;
        }

        let time = Instant::now();

        if self.config.update_profile && update_profile_timer.passed() {
            let profile = rand::random::<u32>();
            let profile = Profile::new(format!("{}", profile));
            self.api
                .profile()
                .post_profile(key.clone(), profile)
                .await?;

            if print_info {
                warn!("post_profile: {:?}", time.elapsed());
            }
        }

        let time = Instant::now();
        match self.config.test {
            Test::Normal => {
                self.api
                    .profile()
                    .get_profile(key.clone(), id)
                    .await?
            }
            Test::Default => {
                self.api
                    .profile()
                    .get_default_profile(key.clone(), id)
                    .await?
            }
        };
        COUNTERS.inc_get_profile();

        if print_info {
            warn!(
                "get_profile: {:?}, total: {}",
                time.elapsed(),
                COUNTERS.reset_get_profile()
            );
        }

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct Counters {
    get_profile: AtomicU64,
}

impl Counters {
    pub const fn new() -> Self {
        Self {
            get_profile: AtomicU64::new(0),
        }
    }

    pub fn inc_get_profile(&self) {
        self.get_profile.fetch_add(1, Ordering::Relaxed);
    }

    pub fn reset_get_profile(&self) -> u64 {
        self.get_profile.swap(0, Ordering::Relaxed)
    }
}

pub struct Timer {
    previous: Instant,
    time: Duration,
}

impl Timer {
    pub fn new(time: Duration) -> Self {
        Self {
            previous: Instant::now(),
            time,
        }
    }

    pub fn passed(&mut self) -> bool {
        if self.previous.elapsed() >= self.time {
            self.previous = Instant::now();
            true
        } else {
            false
        }
    }
}

pub struct AvgTime {
    previous: Instant,
    total: u64,
    counter: u64,
    calculate_avg_when_couter: u64,
    current_avg: Duration,
}

impl AvgTime {
    pub fn new(calculate_avg_when_couter: u64) -> Self {
        Self {
            previous: Instant::now(),
            total: 0,
            counter: 0,
            calculate_avg_when_couter,
            current_avg: Duration::from_micros(0),
        }
    }

    pub fn track(&mut self) {
        self.previous = Instant::now();
    }

    pub fn complete(&mut self) {
        let time = self.previous.elapsed();
        self.total += time.as_micros() as u64;
        self.counter += 1;

        if self.counter >= self.calculate_avg_when_couter {
            self.current_avg = Duration::from_micros(self.total / self.counter);

            self.counter = 0;
            self.total = 0;
        }
    }

    pub fn current_avg(&self) -> Duration {
        self.current_avg
    }
}
