mod cli;
mod config;
mod formatter;
mod git;
mod readme;
mod wakatime;

use color_eyre::Result;
use clap::Parser;
use cli::Cli;
use config::Config;
use formatter::StatsFormatter;
use git::{GitCloner, GitCommitter, GitPusher};
use log::info;
use readme::{ReadmeUpdater, ReadmeValidator};
use wakatime::WakaTimeClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize color_eyre for better error reporting
    color_eyre::install()?;

    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Load configuration
    let config = Config::from_cli(cli)?;

    // Ensure we're in a git repository (clone if needed)
    GitCloner::ensure_repository(config.repository.clone(), config.github_token.clone())?;

    info!("Starting WakaTime stats update...");

    // Ensure README has required section markers
    ReadmeValidator::ensure_section_exists(&config.readme_path, &config.section_name)?;

    // Create WakaTime client
    let client = WakaTimeClient::new(config.api_key.clone());

    // Fetch stats
    info!("Fetching WakaTime data...");
    let range = format!("last_{}_days", config.stats_range);
    let stats = client.get_stats(&range).await?;

    // Format stats
    let formatted_stats = StatsFormatter::format(&stats);

    // Update README
    info!("Updating README...");
    let updated =
        ReadmeUpdater::update(&config.readme_path, &config.section_name, &formatted_stats)?;

    if updated {
        info!("README updated successfully!");

        // Auto-commit if enabled
        if config.auto_commit {
            GitCommitter::commit_changes(
                &config.readme_path,
                &config.commit_message,
                &config.git_user_name,
                &config.git_user_email,
            )?;

            // Auto-push if enabled
            if config.auto_push {
                GitPusher::push_changes(config.github_token.clone())?;
            }
        }
    } else {
        info!("No changes detected in README.");
    }

    Ok(())
}
