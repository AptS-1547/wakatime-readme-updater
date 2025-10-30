use serde::Deserialize;
use color_eyre::eyre::{bail, Result};

#[derive(Debug, Deserialize)]
pub struct StatsResponse {
    pub data: StatsData,
}

#[derive(Debug, Deserialize)]
pub struct StatsData {
    pub languages: Vec<StatItem>,
    pub editors: Vec<StatItem>,
    pub operating_systems: Vec<StatItem>,
    pub projects: Vec<StatItem>,
    pub total_seconds: f64,
    pub is_up_to_date: bool,
}

#[derive(Debug, Deserialize)]
pub struct StatItem {
    pub name: String,
    pub percent: f64,
    pub text: String,
    #[allow(dead_code)]
    pub total_seconds: f64,
}

/// Validate WakaTime API range parameter
pub fn validate_range(range: &str) -> Result<()> {
    // Check for predefined ranges
    let predefined_ranges = [
        "last_7_days",
        "last_30_days",
        "last_6_months",
        "last_year",
        "all_time"
    ];

    if predefined_ranges.contains(&range) {
        return Ok(());
    }

    // Check for YYYY format (year)
    if range.len() == 4 && range.chars().all(|c| c.is_ascii_digit()) {
        return Ok(());
    }

    // Check for YYYY-MM format (year-month)
    if range.len() == 7 {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() == 2 {
            let year = parts[0];
            let month = parts[1];

            if year.len() == 4 && year.chars().all(|c| c.is_ascii_digit()) &&
               month.len() == 2 && month.chars().all(|c| c.is_ascii_digit()) {

                if let Ok(month_num) = month.parse::<u32>() {
                    if month_num >= 1 && month_num <= 12 {
                        return Ok(());
                    }
                }
            }
        }
    }

    bail!("Invalid range format: '{}'. Expected: YYYY, YYYY-MM, or one of: {}",
          range, predefined_ranges.join(", "));
}
