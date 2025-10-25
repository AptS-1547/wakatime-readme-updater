use color_eyre::{eyre::{bail, WrapErr}, Result};
use regex::Regex;
use std::fs;
use std::path::Path;

pub struct ReadmeUpdater;

impl ReadmeUpdater {
    pub fn update<P: AsRef<Path>>(path: P, section_name: &str, new_content: &str) -> Result<bool> {
        let path = path.as_ref();

        // Read README
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        // Build regex pattern
        let start_marker = format!("<!--START_SECTION:{}-->", section_name);
        let end_marker = format!("<!--END_SECTION:{}-->", section_name);

        let pattern = format!(
            r"(?s){}.*?{}",
            regex::escape(&start_marker),
            regex::escape(&end_marker)
        );

        let re = Regex::new(&pattern).context("Failed to create regex")?;

        // Check if markers exist
        if !re.is_match(&content) {
            bail!(
                "Markers {} and {} not found in {}",
                start_marker,
                end_marker,
                path.display()
            );
        }

        // Replace content
        let replacement = format!("{}\n{}\n{}", start_marker, new_content, end_marker);
        let new_readme = re.replace(&content, replacement.as_str()).to_string();

        // Check if content changed
        if new_readme == content {
            return Ok(false);
        }

        // Write file
        fs::write(path, new_readme).with_context(|| format!("Failed to write file: {}", path.display()))?;

        Ok(true)
    }
}
