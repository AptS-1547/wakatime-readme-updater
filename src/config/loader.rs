use crate::cli::Cli;
use color_eyre::{eyre::{ContextCompat, WrapErr}, Result};
use git2::Repository;
use std::{env, path::PathBuf};

use super::toml::TomlConfig;

#[derive(Debug)]
pub struct Config {
    pub api_key: String,
    pub readme_path: PathBuf,
    pub section_name: String,
    pub stats_range_str: String,
    pub auto_commit: bool,
    pub auto_push: bool,
    pub git_user_name: String,
    pub git_user_email: String,
    pub github_token: Option<String>,
    pub repository: Option<String>,
    pub commit_message: String,
}

impl Config {
    pub fn from_cli(cli: Cli) -> Result<Self> {
        // Try to read TOML config file
        let toml_config = if cli.config_path.exists() {
            let content = std::fs::read_to_string(&cli.config_path)
                .context("Failed to read config file")?;
            toml::from_str::<TomlConfig>(&content).context("Failed to parse config file")?
        } else {
            TomlConfig::default()
        };

        // Priority: CLI/ENV (INPUT_*) > TOML
        let api_key = cli
            .api_key
            .or(toml_config.api_key)
            .context("API key not provided. Please provide it via CLI argument, environment variable, or config file")?;

        let readme_path = if cli.readme_path.to_str() == Some("README.md") {
            toml_config
                .readme_path
                .map(PathBuf::from)
                .unwrap_or(cli.readme_path)
        } else {
            cli.readme_path
        };

        let section_name = if cli.section_name == "waka" {
            toml_config.section_name.unwrap_or(cli.section_name)
        } else {
            cli.section_name
        };

        let stats_range_str = if cli.stats_range_str == "last_7_days" {
            toml_config.stats_range_str.unwrap_or(cli.stats_range_str)
        } else {
            cli.stats_range_str
        };

        let auto_commit = cli
            .auto_commit
            .or(toml_config.auto_commit)
            .unwrap_or(false);

        let auto_push = cli
            .auto_push
            .or(toml_config.auto_push)
            .unwrap_or(false);

        // Get git user name: CLI/ENV > TOML > Git Config > Default
        let git_user_name = cli
            .git_user_name
            .or(toml_config.git_user_name)
            .or_else(|| {
                Repository::open(".")
                    .ok()
                    .and_then(|repo| repo.config().ok())
                    .and_then(|config| config.get_string("user.name").ok())
            })
            .unwrap_or_else(|| "github-actions[bot]".to_string());

        // Get git user email: CLI/ENV > TOML > Git Config > Default
        let git_user_email = cli
            .git_user_email
            .or(toml_config.git_user_email)
            .or_else(|| {
                Repository::open(".")
                    .ok()
                    .and_then(|repo| repo.config().ok())
                    .and_then(|config| config.get_string("user.email").ok())
            })
            .unwrap_or_else(|| "github-actions[bot]@users.noreply.github.com".to_string());

        let github_token = cli.github_token.or(toml_config.github_token);

        let commit_message = if cli.commit_message == "📊 Updated WakaTime stats" {
            toml_config.commit_message.unwrap_or(cli.commit_message)
        } else {
            cli.commit_message
        };

        // Get repository: CLI/ENV > TOML > GITHUB_REPOSITORY (GitHub Actions)
        let repository = cli
            .repository
            .or(toml_config.repository)
            .or_else(|| env::var("GITHUB_REPOSITORY").ok())
            .filter(|s| !s.is_empty());

        Ok(Config {
            api_key,
            readme_path,
            section_name,
            stats_range_str,
            auto_commit,
            auto_push,
            git_user_name,
            git_user_email,
            github_token,
            repository,
            commit_message,
        })
    }
}
