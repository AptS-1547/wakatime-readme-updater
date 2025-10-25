use color_eyre::{eyre::eyre, Result};
use git2::{Cred, PushOptions, RemoteCallbacks, Repository};
use log::info;

pub struct GitPusher;

impl GitPusher {
    pub fn push_changes(github_token: Option<String>) -> Result<()> {
        info!("Pushing changes to remote...");

        let repo = Repository::open(".")
            .map_err(|e| eyre!("Failed to open Git repository: {}", e))?;

        // Get the current branch
        let head = repo
            .head()
            .map_err(|e| eyre!("Failed to get HEAD: {}", e))?;

        let branch_name = head
            .shorthand()
            .ok_or_else(|| eyre!("Failed to get branch name"))?;

        // Get remote
        let remote_name = "origin";
        let remote = repo
            .find_remote(remote_name)
            .map_err(|e| eyre!("Failed to find remote 'origin': {}", e))?;

        // If token is provided, convert SSH URL to HTTPS for pushing
        let original_url = remote.url().unwrap_or("");
        let should_convert_url = github_token.is_some() && original_url.starts_with("git@github.com:");

        // Create appropriate remote for pushing
        let mut push_remote = if should_convert_url {
            // Convert git@github.com:owner/repo.git -> https://github.com/owner/repo.git
            let https_url = original_url
                .replace("git@github.com:", "https://github.com/")
                .to_string();

            info!("Converting SSH URL to HTTPS for token authentication");
            // Create an anonymous remote with HTTPS URL
            repo.remote_anonymous(&https_url)
                .map_err(|e| eyre!("Failed to create anonymous remote: {}", e))?
        } else {
            remote
        };

        // Set up credentials callback
        let mut callbacks = RemoteCallbacks::new();
        let token_clone = github_token.clone();
        callbacks.credentials(move |_url, username_from_url, _allowed_types| {
            // Try GitHub token first if provided
            if let Some(ref token) = token_clone {
                return Cred::userpass_plaintext("x-access-token", token);
            }

            // Try SSH key
            if let Some(username) = username_from_url {
                if let Ok(cred) = Cred::ssh_key_from_agent(username) {
                    return Ok(cred);
                }
            }

            // Try default SSH key
            Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
        });

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);

        // Push to remote
        let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
        push_remote
            .push(&[&refspec], Some(&mut push_options))
            .map_err(|e| eyre!("Failed to push changes: {}", e))?;

        info!("Changes pushed successfully to remote!");
        Ok(())
    }
}
