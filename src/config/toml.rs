use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct TomlConfig {
    pub api_key: Option<String>,
    pub readme_path: Option<String>,
    pub section_name: Option<String>,
    pub stats_range_str: Option<String>,
    pub auto_commit: Option<bool>,
    pub auto_push: Option<bool>,
    pub git_user_name: Option<String>,
    pub git_user_email: Option<String>,
    pub github_token: Option<String>,
    pub repository: Option<String>,
    pub commit_message: Option<String>,
}
