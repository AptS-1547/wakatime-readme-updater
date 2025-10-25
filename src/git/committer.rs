use color_eyre::{eyre::eyre, Result};
use git2::{Repository, Signature};
use log::info;
use std::path::Path;

pub struct GitCommitter;

impl GitCommitter {
    pub fn commit_changes<P: AsRef<Path>>(
        file_path: P,
        commit_message: &str,
        user_name: &str,
        user_email: &str,
    ) -> Result<()> {
        info!("Committing changes to Git...");

        let repo = Repository::open(".")
            .map_err(|e| eyre!("Failed to open Git repository: {}", e))?;

        // Get index
        let mut index = repo
            .index()
            .map_err(|e| eyre!("Failed to get Git index: {}", e))?;

        // Add file to staging area
        index
            .add_path(file_path.as_ref())
            .map_err(|e| eyre!("Failed to add file to staging area: {}", e))?;

        index
            .write()
            .map_err(|e| eyre!("Failed to write index: {}", e))?;

        // Create tree
        let tree_oid = index
            .write_tree()
            .map_err(|e| eyre!("Failed to create tree: {}", e))?;

        let tree = repo
            .find_tree(tree_oid)
            .map_err(|e| eyre!("Failed to find tree: {}", e))?;

        // Get HEAD
        let head = repo
            .head()
            .map_err(|e| eyre!("Failed to get HEAD: {}", e))?;

        let parent_commit = head
            .peel_to_commit()
            .map_err(|e| eyre!("Failed to get parent commit: {}", e))?;

        // Create signature
        let signature = Signature::now(user_name, user_email)
            .map_err(|e| eyre!("Failed to create signature: {}", e))?;

        // Create commit
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            commit_message,
            &tree,
            &[&parent_commit],
        )
        .map_err(|e| eyre!("Failed to create commit: {}", e))?;

        info!("Changes committed successfully!");
        Ok(())
    }
}
