use color_eyre::{eyre::{bail, WrapErr}, Result};

use crate::wakatime::client::WakaTimeClient;
use super::types::{StatsData, StatsResponse};

const WAKATIME_API_BASE: &str = "https://wakatime.com/api/v1";

impl WakaTimeClient {
    /// Get coding stats for a specific time range
    ///
    /// # Arguments
    /// * `range` - Time range (e.g., "last_7_days", "last_30_days")
    pub async fn get_stats(&self, range: &str) -> Result<StatsData> {
        let url = format!(
            "{}/users/current/stats/{}?api_key={}",
            WAKATIME_API_BASE, range, self.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to request WakaTime API")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            bail!("WakaTime API returned error {}: {}", status, text);
        }

        let stats_response: StatsResponse = response
            .json()
            .await
            .context("Failed to parse WakaTime API response")?;

        Ok(stats_response.data)
    }
}
