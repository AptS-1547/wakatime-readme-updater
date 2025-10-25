use color_eyre::{eyre::{bail, WrapErr}, Result};
use std::fs;
use std::path::Path;

pub struct ReadmeValidator;

impl ReadmeValidator {
    pub fn ensure_section_exists<P: AsRef<Path>>(path: P, section_name: &str) -> Result<()> {
        let path = path.as_ref();

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        let start_marker = format!("<!--START_SECTION:{}-->", section_name);
        let end_marker = format!("<!--END_SECTION:{}-->", section_name);

        if content.contains(&start_marker) && content.contains(&end_marker) {
            return Ok(());
        }

        bail!(
            "Required markers are missing in README. Please add:\n{}\n{}\n",
            start_marker,
            end_marker
        )
    }
}
