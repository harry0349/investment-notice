use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockData {
    pub date: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyAnalysis {
    pub date: DateTime<Utc>,
    pub current_price: f64,
    pub previous_price: f64,
    pub price_change_pct: f64,
    pub relative_to_high: f64,
    pub relative_to_low: f64,
    pub historical_high: f64,
    pub historical_low: f64,
    pub volume: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeeklyAnalysis {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub start_price: f64,
    pub end_price: f64,
    pub weekly_change_pct: f64,
    pub highest_price: f64,
    pub highest_date: DateTime<Utc>,
    pub lowest_price: f64,
    pub lowest_date: DateTime<Utc>,
    pub average_volume: f64,
    pub total_volume: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyAnalysis {
    pub year: i32,
    pub month: u32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub start_price: f64,
    pub end_price: f64,
    pub monthly_change_pct: f64,
    pub highest_price: f64,
    pub highest_date: DateTime<Utc>,
    pub lowest_price: f64,
    pub lowest_date: DateTime<Utc>,
    pub average_volume: f64,
    pub total_volume: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    pub code: String,
    pub data: Vec<StockData>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    pub to_emails: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeminiConfig {
    pub api_key: String,
    pub model: String,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            smtp_server: "smtp.gmail.com".to_string(),
            smtp_port: 587,
            username: "".to_string(),
            password: "".to_string(),
            from_email: "".to_string(),
            to_emails: vec![],
        }
    }
}

impl Default for GeminiConfig {
    fn default() -> Self {
        Self {
            api_key: "".to_string(),
            model: "gemini-pro".to_string(),
        }
    }
}
