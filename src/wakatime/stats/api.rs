use color_eyre::{eyre::{bail, WrapErr}, Result};
use reqwest::Url;
use std::time::Duration;

use crate::wakatime::client::WakaTimeClient;
use super::types::{StatsData, StatsResponse, validate_range};

const WAKATIME_API_BASE: &str = "https://wakatime.com/api/v1";

impl WakaTimeClient {
    /// Get coding stats for a specific time range
    ///
    /// # Arguments
    /// * `range` - Time range (e.g., "last_7_days", "last_30_days", "2024", "2024-10")
    pub async fn get_stats(&self, range: &str) -> Result<StatsData> {
        // Validate range parameter first
        validate_range(range)?;

        let max_retries = 3;
        let retry_delay = Duration::from_secs(2);

        for attempt in 0..=max_retries {
            let url = match Url::parse_with_params(
                &format!("{}/users/current/stats/{}", WAKATIME_API_BASE, range),
                &[("api_key", &self.api_key)],
            ) {
                Ok(url) => url,
                Err(e) => bail!("Failed to construct WakaTime API URL: {}", e),
            };

            let response = self
                .client
                .get(url)
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

            // Check if data is up to date, retry if not
            if !stats_response.data.is_up_to_date {
                if attempt < max_retries {
                    // Wait before retrying
                    tokio::time::sleep(retry_delay * (attempt + 1)).await;
                    continue;
                } else {
                    // Max retries reached, return stale data with a warning
                    eprintln!("Warning: Stats data for range '{}' is not up to date after {} retries. Using stale data.", range, max_retries);
                    return Ok(stats_response.data);
                }
            }

            return Ok(stats_response.data);
        }

        // This should not be reached, but just in case
        bail!("Failed to get up-to-date stats after {} retries", max_retries);
    }
}
