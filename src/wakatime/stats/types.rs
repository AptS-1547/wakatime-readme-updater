use serde::Deserialize;

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
}

#[derive(Debug, Deserialize)]
pub struct StatItem {
    pub name: String,
    pub percent: f64,
    pub text: String,
    #[allow(dead_code)]
    pub total_seconds: f64,
}
