use clap::Parser;
use std::path::PathBuf;

use super::styles::clap_styles;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(styles = clap_styles())]
pub struct Cli {
    /// WakaTime API Key
    #[arg(long, env = "INPUT_WAKATIME_API_KEY")]
    pub api_key: Option<String>,

    /// Path to README file
    #[arg(long, env = "INPUT_README_PATH", default_value = "README.md")]
    pub readme_path: PathBuf,

    /// Section name in README to update
    #[arg(long, env = "INPUT_SECTION_NAME", default_value = "waka")]
    pub section_name: String,

    /// Path to config file
    #[arg(long, env = "INPUT_CONFIG_PATH", default_value = "wakatime-updater.toml")]
    pub config_path: PathBuf,

    /// Number of days to fetch stats for
    #[arg(long, env = "INPUT_STATS_RANGE", default_value = "7")]
    pub stats_range: u32,

    /// Whether to commit changes automatically
    #[arg(long, env = "INPUT_AUTO_COMMIT")]
    pub auto_commit: Option<bool>,

    /// Whether to push changes automatically
    #[arg(long, env = "INPUT_AUTO_PUSH")]
    pub auto_push: Option<bool>,

    /// Git commit username
    #[arg(long, env = "INPUT_COMMIT_USERNAME")]
    pub git_user_name: Option<String>,

    /// Git commit email
    #[arg(long, env = "INPUT_COMMIT_EMAIL")]
    pub git_user_email: Option<String>,

    /// GitHub token for pushing (optional, uses SSH if not provided)
    #[arg(long, env = "INPUT_GH_TOKEN")]
    pub github_token: Option<String>,

    /// Git repository to clone (optional, for use without checkout action)
    #[arg(long, env = "INPUT_REPOSITORY")]
    pub repository: Option<String>,

    /// Git commit message
    #[arg(
        long,
        env = "INPUT_COMMIT_MESSAGE",
        default_value = "📊 Updated WakaTime stats"
    )]
    pub commit_message: String,
}
