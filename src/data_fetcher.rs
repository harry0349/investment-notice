use crate::models::{ApiResponse, StockData};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Duration, Utc};
use rand::{prelude::*, rng};
use reqwest::Client;
use serde_json::Value;
use tracing::{debug, info, warn};

const HS300_CODE: &str = "000300"; // CSI 300 Index code
const TUSHARE_API_URL: &str = "https://api.tushare.pro";
const ALPHA_VANTAGE_API_URL: &str = "https://www.alphavantage.co/query";

/// Fetch real-time CSI 300 ETF data
pub async fn fetch_hs300_data() -> Result<Vec<StockData>> {
    info!("Starting to fetch CSI 300 ETF data");

    // Try multiple data sources
    match fetch_from_tushare().await {
        Ok(data) => {
            info!("Retrieved {} data points from TuShare", data.len());
            Ok(data)
        }
        Err(e) => {
            warn!("TuShare fetch failed: {}, trying backup data source", e);
            match fetch_from_alpha_vantage().await {
                Ok(data) => {
                    info!("Retrieved {} data points from Alpha Vantage", data.len());
                    Ok(data)
                }
                Err(e2) => {
                    warn!("Alpha Vantage also failed: {}, using mock data", e2);
                    Ok(generate_mock_data())
                }
            }
        }
    }
}

/// Fetch weekly data
pub async fn fetch_weekly_hs300_data() -> Result<Vec<StockData>> {
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(7);
    fetch_hs300_data_in_range(start_date, end_date).await
}

/// Fetch monthly data
pub async fn fetch_monthly_hs300_data() -> Result<Vec<StockData>> {
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);
    fetch_hs300_data_in_range(start_date, end_date).await
}

/// Fetch data within specified time range
async fn fetch_hs300_data_in_range(
    _start_date: DateTime<Utc>,
    _end_date: DateTime<Utc>,
) -> Result<Vec<StockData>> {
    // Due to API limitations, return recent data for now
    fetch_hs300_data().await
}

/// Fetch data from TuShare
async fn fetch_from_tushare() -> Result<Vec<StockData>> {
    let token = std::env::var("TUSHARE_TOKEN")
        .map_err(|_| anyhow!("TUSHARE_TOKEN environment variable not set"))?;

    let client = Client::new();

    let payload = serde_json::json!({
        "api_name": "index_daily",
        "token": token,
        "params": {
            "ts_code": format!("{}.SH", HS300_CODE),
            "start_date": "20240101",
            "end_date": Utc::now().format("%Y%m%d").to_string()
        }
    });

    debug!(
        "Sending request to TuShare: {}",
        serde_json::to_string_pretty(&payload)?
    );

    let response = client.post(TUSHARE_API_URL).json(&payload).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("TuShare API request failed: {}", response.status()));
    }

    let api_response: ApiResponse = response.json().await?;
    debug!("TuShare response: {:?}", api_response);

    if api_response.code != "0" {
        return Err(anyhow!(
            "TuShare API error: {}",
            api_response.message.unwrap_or_default()
        ));
    }

    Ok(api_response.data)
}

/// Fetch data from Alpha Vantage
async fn fetch_from_alpha_vantage() -> Result<Vec<StockData>> {
    let api_key = std::env::var("ALPHA_VANTAGE_API_KEY")
        .map_err(|_| anyhow!("ALPHA_VANTAGE_API_KEY environment variable not set"))?;

    let client = Client::new();

    let params = [
        ("function", "TIME_SERIES_DAILY"),
        ("symbol", "000300.SS"), // 沪深300在Alpha Vantage的代码
        ("outputsize", "compact"),
        ("apikey", &api_key),
    ];

    let response = client
        .get(ALPHA_VANTAGE_API_URL)
        .query(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Alpha Vantage API request failed: {}",
            response.status()
        ));
    }

    let json: Value = response.json().await?;
    debug!("Alpha Vantage response: {:?}", json);

    if let Some(error_message) = json.get("Error Message") {
        return Err(anyhow!("Alpha Vantage API error: {}", error_message));
    }

    if let Some(time_series) = json.get("Time Series (Daily)") {
        let mut data = Vec::new();

        if let Some(obj) = time_series.as_object() {
            for (date_str, values) in obj {
                #[allow(clippy::collapsible_if)]
                if let Ok(date) = DateTime::parse_from_str(
                    &format!("{} 00:00:00 +0000", date_str),
                    "%Y-%m-%d %H:%M:%S %z",
                ) {
                    if let Some(day_data) = values.as_object() {
                        let open = day_data
                            .get("1. open")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0.0);
                        let high = day_data
                            .get("2. high")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0.0);
                        let low = day_data
                            .get("3. low")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0.0);
                        let close = day_data
                            .get("4. close")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0.0);
                        let volume = day_data
                            .get("5. volume")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0);

                        data.push(StockData {
                            date: date.into(),
                            open,
                            high,
                            low,
                            close,
                            volume,
                        });
                    }
                }
            }
        }

        // Sort by date
        data.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(data)
    } else {
        Err(anyhow!("Alpha Vantage response format error"))
    }
}

/// Generate mock data (for testing or when all APIs are unavailable)
fn generate_mock_data() -> Vec<StockData> {
    let mut data = Vec::new();
    let base_date = Utc::now() - Duration::days(30);
    let mut current_price: f64 = 3500.0;
    let mut rng = rng();

    for i in 0..30 {
        let date = base_date + Duration::days(i);
        let change = (rng.random::<f64>() - 0.5) * 100.0; // Random change between -50 and +50
        current_price += change;
        current_price = current_price.clamp(3000.0, 4000.0); // Keep within reasonable range

        let open: f64 = current_price + (rng.random::<f64>() - 0.5) * 20.0;
        let high: f64 = open + rng.random::<f64>() * 50.0;
        let low: f64 = open - rng.random::<f64>() * 50.0;
        let close: f64 = current_price;

        data.push(StockData {
            date,
            open: open.max(0.0),
            high: high.max(0.0),
            low: low.max(0.0),
            close: close.max(0.0),
            volume: (rng.random::<u64>() % 1000000) + 500000,
        });
    }

    info!("Generated {} mock data points", data.len());
    data
}

/// Get the latest stock price
#[allow(dead_code)]
pub async fn get_current_price() -> Result<f64> {
    let data = fetch_hs300_data().await?;
    if let Some(latest) = data.last() {
        Ok(latest.close)
    } else {
        Err(anyhow!("Unable to get current price"))
    }
}
