use color_eyre::{eyre::eyre, Result};
use git2::{
    build::RepoBuilder, Config as GitConfig, Cred, FetchOptions, RemoteCallbacks, Repository,
    RepositoryInitOptions,
};
use log::info;
use std::{env, fs};

pub struct GitCloner;

impl GitCloner {
    /// Configure git safe.directory for current directory
    fn configure_safe_directory() {
        if let Ok(current_dir) = env::current_dir() {
            if let Some(current_dir_str) = current_dir.to_str() {
                if let Ok(mut config) = GitConfig::open_default() {
                    let _ = config.set_str("safe.directory", current_dir_str);
                }
            }
        }
    }

    /// Check if current directory is a git repository
    pub fn is_git_repo() -> bool {
        // Try to configure safe.directory first
        Self::configure_safe_directory();
        Repository::open(".").is_ok()
    }

    /// Check if current directory is empty
    fn is_directory_empty() -> bool {
        fs::read_dir(".")
            .map(|mut entries| entries.next().is_none())
            .unwrap_or(false)
    }

    /// Clone repository if needed (when not using checkout action)
    pub fn ensure_repository(repository: Option<String>, token: Option<String>) -> Result<()> {
        // If already in a git repo, nothing to do
        if Self::is_git_repo() {
            info!("Already in a git repository");
            return Ok(());
        }

        // If no repository specified, can't clone
        let repo_url = repository
            .filter(|s| !s.is_empty())
            .ok_or_else(|| {
                eyre!(
                    "Not in a git repository and no repository parameter provided. \
                    Either use checkout action or provide repository parameter."
                )
            })?;

        info!("Not in a git repository, cloning: {}", repo_url);

        // Build clone URL
        let clone_url = if repo_url.starts_with("git://github.com/") {
            // Convert git:// to https:// for token authentication
            repo_url.replace("git://", "https://")
        } else if repo_url.contains('/') && !repo_url.starts_with("http") {
            // Convert owner/repo to https://github.com/owner/repo.git
            format!("https://github.com/{}.git", repo_url)
        } else if repo_url.starts_with("https://github.com/") {
            if !repo_url.ends_with(".git") {
                format!("{}.git", repo_url)
            } else {
                repo_url.clone()
            }
        } else {
            repo_url.clone()
        };

        // Set up credentials callback
        let mut callbacks = RemoteCallbacks::new();
        let token_clone = token.clone();
        callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
            if let Some(ref token) = token_clone {
                Cred::userpass_plaintext("x-access-token", token)
            } else {
                Err(git2::Error::from_str("No authentication provided"))
            }
        });

        // If directory is not empty, use init + remote + fetch approach
        if !Self::is_directory_empty() {
            info!("Directory not empty, using init + fetch approach");
            Self::init_and_fetch(&clone_url, token)?;
        } else {
            info!("Directory is empty, using clone approach");
            // Configure fetch options
            let mut fetch_options = FetchOptions::new();
            fetch_options.remote_callbacks(callbacks);
            fetch_options.depth(1); // Shallow clone

            // Clone the repository
            RepoBuilder::new()
                .fetch_options(fetch_options)
                .clone(&clone_url, std::path::Path::new("."))
                .map_err(|e| eyre!("Failed to clone repository: {}", e))?;
        }

        info!("Repository cloned successfully");
        Ok(())
    }

    /// Initialize repo and fetch from remote (for non-empty directories)
    fn init_and_fetch(clone_url: &str, token: Option<String>) -> Result<()> {
        // Initialize repository (or open if already exists)
        let repo = Repository::init_opts(".", RepositoryInitOptions::new().initial_head("main"))
            .map_err(|e| eyre!("Failed to init repository: {}", e))?;

        // Get or add remote
        let mut remote = match repo.find_remote("origin") {
            Ok(remote) => {
                info!("Remote 'origin' already exists, using existing remote");
                remote
            }
            Err(_) => {
                info!("Adding remote 'origin'");
                repo.remote("origin", clone_url)
                    .map_err(|e| eyre!("Failed to add remote: {}", e))?
            }
        };

        // Set up credentials callback
        let mut callbacks = RemoteCallbacks::new();
        let token_clone = token;
        callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
            if let Some(ref token) = token_clone {
                Cred::userpass_plaintext("x-access-token", token)
            } else {
                Err(git2::Error::from_str("No authentication provided"))
            }
        });

        // Configure fetch options
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        fetch_options.depth(1); // Shallow fetch

        // Fetch from remote
        remote
            .fetch(&["main"], Some(&mut fetch_options), None)
            .or_else(|_| {
                // If main doesn't exist, try master
                remote.fetch(&["master"], Some(&mut fetch_options), None)
            })
            .map_err(|e| eyre!("Failed to fetch from remote: {}", e))?;

        // Get the branch name
        let fetch_head = repo
            .find_reference("FETCH_HEAD")
            .map_err(|e| eyre!("Failed to find FETCH_HEAD: {}", e))?;

        let fetch_commit = repo
            .reference_to_annotated_commit(&fetch_head)
            .map_err(|e| eyre!("Failed to get fetch commit: {}", e))?;

        // Reset to fetched commit (hard reset)
        let commit = repo
            .find_commit(fetch_commit.id())
            .map_err(|e| eyre!("Failed to find commit: {}", e))?;

        repo.reset(
            commit.as_object(),
            git2::ResetType::Hard,
            None,
        )
        .map_err(|e| eyre!("Failed to reset to commit: {}", e))?;

        // Set up branch tracking
        let branch_name = repo
            .head()
            .ok()
            .and_then(|h| h.shorthand().map(String::from))
            .unwrap_or_else(|| "main".to_string());

        repo.branch(&branch_name, &commit, true)
            .ok();

        Ok(())
    }
}
