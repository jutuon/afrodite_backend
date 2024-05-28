//! Config given as command line arguments

use std::{fmt, path::PathBuf};

use clap::{arg, command, Args, Parser, ValueEnum};
use reqwest::Url;
use simple_backend_config::args::{ImageProcessModeArgs, ServerModeArgs};

#[derive(Args, Debug, Clone)]
pub struct ArgsConfig {
    /// Print build info and quit.
    #[arg(short, long)]
    pub build_info: bool,

    #[command(flatten)]
    pub server: ServerModeArgs,

    #[command(subcommand)]
    pub mode: Option<AppMode>,
}

#[derive(Parser, Debug, Clone)]
pub enum AppMode {
    /// Run test, benchmark or bot mode
    Test(TestMode),
    /// Process received image
    ImageProcess(ImageProcessModeArgs),
    /// Print API documentation JSON to stdout
    OpenApi,
}

#[derive(Parser, Debug, Clone)]
pub struct PublicApiUrls {
    /// Base URL for account API for register and login
    #[arg(long, default_value = "http://127.0.0.1:3001", value_name = "URL")]
    pub url_register: Url,

    /// Base URL for account API
    #[arg(long, default_value = "http://127.0.0.1:3000", value_name = "URL")]
    pub url_account: Url,

    /// Base URL for profile API
    #[arg(long, default_value = "http://127.0.0.1:3000", value_name = "URL")]
    pub url_profile: Url,

    /// Base URL for media API
    #[arg(long, default_value = "http://127.0.0.1:3000", value_name = "URL")]
    pub url_media: Url,

    /// Base URL for chat API
    #[arg(long, default_value = "http://127.0.0.1:3000", value_name = "URL")]
    pub url_chat: Url,
}

#[derive(Args, Debug, Clone)]
pub struct TestMode {
    #[command(flatten)]
    pub server: ServerConfig,

    #[arg(long, value_name = "FILE")]
    pub bot_config_file: Option<PathBuf>,

    // Boolean flags
    /// Do not remove server instance database files
    #[arg(long)]
    pub no_clean: bool,

    /// Do not start new server instances
    #[arg(long)]
    pub no_servers: bool,

    /// First error quits
    #[arg(long)]
    pub early_quit: bool,

    #[command(subcommand)]
    pub mode: TestModeSubMode,
}

impl TestMode {
    pub fn bots(&self) -> u32 {
        match &self.mode {
            TestModeSubMode::Bot(c) => c.users + c.admins,
            TestModeSubMode::Benchmark(c) => c.bots,
            _ => 1,
        }
    }

    pub fn tasks(&self) -> u32 {
        match &self.mode {
            TestModeSubMode::Benchmark(c) => c.tasks,
            _ => 1,
        }
    }

    pub fn save_state(&self) -> bool {
        match &self.mode {
            TestModeSubMode::Bot(c) => c.save_state,
            TestModeSubMode::Benchmark(c) => c.save_state,
            _ => false,
        }
    }

    pub fn no_sleep(&self) -> bool {
        match &self.mode {
            TestModeSubMode::Bot(c) => c.no_sleep,
            TestModeSubMode::Benchmark(c) => !c.sleep,
            _ => false,
        }
    }

    pub fn qa_mode(&self) -> Option<&QaTestConfig> {
        match &self.mode {
            TestModeSubMode::Qa(c) => Some(c),
            _ => None,
        }
    }

    pub fn bot_mode(&self) -> Option<&BotModeConfig> {
        match &self.mode {
            TestModeSubMode::Bot(c) => Some(c),
            _ => None,
        }
    }

    pub fn selected_benchmark(&self) -> Option<&SelectedBenchmark> {
        match &self.mode {
            TestModeSubMode::Benchmark(c) => Some(&c.benchmark),
            _ => None,
        }
    }

    /// Test name which does not have whitespace
    pub fn test_name(&self) -> String {
        match &self.mode {
            TestModeSubMode::Bot(_) => "bot".to_string(),
            TestModeSubMode::Qa(_) => "qa".to_string(),
            TestModeSubMode::Benchmark(c) => format!("benchmark_{:?}", c.benchmark),
        }
    }
}

#[derive(Parser, Debug, Clone)]
pub enum TestModeSubMode {
    /// Run benchmark
    Benchmark(BenchmarkConfig),
    /// Run QA test suite
    Qa(QaTestConfig),
    /// Run bot mode
    Bot(BotModeConfig),
}

#[derive(Parser, Debug, Clone)]
pub struct ServerConfig {
    #[command(flatten)]
    pub api_urls: PublicApiUrls,

    /// Directory for test database
    #[arg(long, default_value = "tmp_databases", value_name = "DIR")]
    pub test_database: PathBuf,

    /// Start media API as microservice
    #[arg(long)]
    pub microservice_media: bool,

    /// Start profile API as microservice
    #[arg(long)]
    pub microservice_profile: bool,

    /// Start chat API as microservice
    #[arg(long)]
    pub microservice_chat: bool,

    /// Enable debug logging for server instances
    #[arg(long)]
    pub log_debug: bool,
}

#[derive(Args, Debug, Clone)]
pub struct BenchmarkConfig {
    /// Bot count per task
    #[arg(short, long, default_value = "1", value_name = "COUNT")]
    pub bots: u32,

    /// Task count
    #[arg(short, long, default_value = "1", value_name = "COUNT")]
    pub tasks: u32,

    /// Enable bot sleep time
    #[arg(long)]
    pub sleep: bool,

    /// Select benchmark
    #[arg(long, default_value = "get-profile", value_name = "NAME", value_enum)]
    pub benchmark: SelectedBenchmark,

    /// Save and load state
    #[arg(long)]
    pub save_state: bool,
}

#[derive(Args, Debug, Clone)]
pub struct QaTestConfig {
    /// Try to continue from test which name contains this text
    #[arg(long)]
    pub continue_from: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct BotModeConfig {
    /// User bot count per task
    #[arg(short, long, default_value = "1", value_name = "COUNT")]
    pub users: u32,

    /// Admin bot count per task
    #[arg(short, long, default_value = "0", value_name = "COUNT")]
    pub admins: u32,

    /// Make bots to make requests constantly
    #[arg(long)]
    pub no_sleep: bool,

    /// Save and load state
    #[arg(long)]
    pub save_state: bool,
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum SelectedBenchmark {
    GetProfile,
    GetProfileFromDatabase,
    GetProfileList,
    PostProfile,
    PostProfileToDatabase,
}

impl fmt::Display for SelectedBenchmark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
