use crate::models::{DailyAnalysis, MonthlyAnalysis, WeeklyAnalysis};
use anyhow::Result;

use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

#[derive(Debug, Serialize, Deserialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Part {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Candidate {
    content: Content,
}

/// Generate daily analysis report
pub async fn generate_daily_analysis(analysis: &DailyAnalysis) -> Result<String> {
    let prompt = format!(
        "You are a professional stock analyst. Please analyze the following CSI 300 ETF data:\n\n\
        Date: {}\n\
        Current Price: {:.2} CNY\n\
        Price Change: {:.2}%\n\
        Relative to High: {:.2}%\n\
        Relative to Low: {:.2}%\n\
        Historical High: {:.2} CNY\n\
        Historical Low: {:.2} CNY\n\
        Volume: {}\n\n\
        Please provide professional investment advice including:\n\
        1. Market trend analysis\n\
        2. Risk assessment\n\
        3. Investment recommendations\n\
        4. Key points to watch\n\n\
        Please respond in English, maintaining professionalism and objectivity.",
        analysis.date.format("%Y-%m-%d"),
        analysis.current_price,
        analysis.price_change_pct,
        analysis.relative_to_high,
        analysis.relative_to_low,
        analysis.historical_high,
        analysis.historical_low,
        analysis.volume
    );

    generate_gemini_response(&prompt).await
}

/// Generate weekly analysis report
pub async fn generate_weekly_analysis(analysis: &WeeklyAnalysis) -> Result<String> {
    let prompt = format!(
        "You are a professional stock analyst. Please analyze the following CSI 300 ETF weekly data:\n\n\
        Period: {} to {}\n\
        Start Price: {:.2} CNY\n\
        End Price: {:.2} CNY\n\
        Weekly Change: {:.2}%\n\
        Highest: {:.2} CNY ({})\n\
        Lowest: {:.2} CNY ({})\n\
        Average Volume: {:.0}\n\
        Total Volume: {}\n\n\
        Please analyze this week's market performance including:\n\
        1. Weekly trend analysis\n\
        2. Key price breakouts\n\
        3. Volume analysis\n\
        4. Next week outlook\n\
        5. Investment strategy recommendations\n\n\
        Please respond in English, maintaining professionalism and objectivity.",
        analysis.start_date.format("%Y-%m-%d"),
        analysis.end_date.format("%Y-%m-%d"),
        analysis.start_price,
        analysis.end_price,
        analysis.weekly_change_pct,
        analysis.highest_price,
        analysis.highest_date.format("%Y-%m-%d"),
        analysis.lowest_price,
        analysis.lowest_date.format("%Y-%m-%d"),
        analysis.average_volume,
        analysis.total_volume
    );

    generate_gemini_response(&prompt).await
}

/// Generate monthly analysis report
pub async fn generate_monthly_analysis(analysis: &MonthlyAnalysis) -> Result<String> {
    let prompt = format!(
        "You are a professional stock analyst. Please analyze the following CSI 300 ETF monthly data:\n\n\
        Month: {}-{}\n\
        Start Price: {:.2} CNY\n\
        End Price: {:.2} CNY\n\
        Monthly Change: {:.2}%\n\
        Highest: {:.2} CNY ({})\n\
        Lowest: {:.2} CNY ({})\n\
        Average Volume: {:.0}\n\
        Total Volume: {}\n\n\
        Please analyze this month's market performance including:\n\
        1. Overall monthly trend\n\
        2. Important support and resistance levels\n\
        3. Monthly volume analysis\n\
        4. Next month market outlook\n\
        5. Long-term investment recommendations\n\n\
        Please respond in English, maintaining professionalism and objectivity.",
        analysis.year,
        analysis.month,
        analysis.start_price,
        analysis.end_price,
        analysis.monthly_change_pct,
        analysis.highest_price,
        analysis.highest_date.format("%Y-%m-%d"),
        analysis.lowest_price,
        analysis.lowest_date.format("%Y-%m-%d"),
        analysis.average_volume,
        analysis.total_volume
    );

    generate_gemini_response(&prompt).await
}

/// Call Gemini API to generate response
async fn generate_gemini_response(prompt: &str) -> Result<String> {
    let api_key = match std::env::var("GEMINI_API_KEY") {
        Ok(key) if !key.trim().is_empty() => key,
        _ => {
            warn!("GEMINI_API_KEY not set or empty, using fallback analysis");
            return Ok("🔑 API Key Required\n\nGemini API key is not configured. To enable AI-powered analysis:\n\n1. Get a Gemini API key from: https://makersuite.google.com/app/apikey\n2. Set GEMINI_API_KEY environment variable\n3. Restart the application\n\nFor now, you'll receive basic market analysis.".to_string());
        }
    };

    debug!(
        "Sending request to Gemini API, prompt length: {} characters",
        prompt.len()
    );

    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",
        api_key
    );

    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    };

    let response = client.post(&url).json(&request_body).send().await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<GeminiResponse>().await {
                    Ok(gemini_resp) => {
                        if let Some(text) = gemini_resp
                            .candidates
                            .first()
                            .and_then(|candidate| candidate.content.parts.first())
                            .map(|part| &part.text)
                        {
                            info!(
                                "Gemini API response successful, length: {} characters",
                                text.len()
                            );
                            Ok(text.clone())
                        } else {
                            warn!("Gemini API response format error");
                            Ok("Sorry, unable to generate analysis report. Please check API configuration.".to_string())
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse Gemini response: {:?}", e);
                        Ok("Sorry, error occurred while parsing AI response.".to_string())
                    }
                }
            } else {
                let status = resp.status();
                let error_text = resp.text().await.unwrap_or_default();
                warn!("Gemini API request failed: {} - {}", status, error_text);
                // Return a fallback message instead of error to prevent program crash
                Ok("⚠️ AI Analysis Unavailable\n\nThe Gemini API is currently not accessible. This could be due to:\n- API key not configured\n- API quota exceeded\n- Network connectivity issues\n\nPlease check your API configuration in the environment variables.".to_string())
            }
        }
        Err(e) => {
            warn!("Network request failed: {:?}", e);
            // Return a fallback message instead of error to prevent program crash
            Ok("⚠️ AI Analysis Unavailable\n\nNetwork connection to Gemini API failed. This could be due to:\n- Internet connectivity issues\n- Firewall blocking the request\n- DNS resolution problems\n\nThe system will continue with basic analysis.".to_string())
        }
    }
}

/// Generate custom analysis report
#[allow(dead_code)]
pub async fn generate_custom_analysis(prompt: &str) -> Result<String> {
    generate_gemini_response(prompt).await
}
