mod analyzer;
mod data_fetcher;
mod email_sender;
mod gemini_client;
mod models;
mod scheduler;

use anyhow::Result;
use clap::Parser;
use tracing::info;

#[derive(Parser)]
#[command(name = "investment-notice")]
#[command(about = "A-Share Investment Notification System - CSI 300 ETF Analysis")]
struct Args {
    /// Run mode: daily, weekly, monthly
    #[arg(short, long, default_value = "daily")]
    mode: String,

    /// Whether to send email notifications
    #[arg(short, long, default_value = "false")]
    send_email: bool,

    /// Debug mode
    #[arg(short, long, default_value = "false")]
    debug: bool,
}

/// Main entry point for the A-Share Investment Notification System
///
/// This application analyzes CSI 300 ETF data and provides investment notifications
/// through various analysis modes (daily, weekly, monthly) with optional email delivery.
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    let args = Args::parse();

    info!(
        "Starting A-Share Investment Notification System, mode: {}",
        args.mode
    );

    match args.mode.as_str() {
        "daily" => run_daily_analysis(args.send_email).await?,
        "weekly" => run_weekly_analysis(args.send_email).await?,
        "monthly" => run_monthly_analysis(args.send_email).await?,
        _ => {
            eprintln!(
                "Invalid mode: {}. Supported modes: daily, weekly, monthly",
                args.mode
            );
            std::process::exit(1);
        }
    }

    info!("Analysis completed");
    Ok(())
}

/// Execute daily investment analysis
///
/// Fetches current CSI 300 ETF data, performs technical analysis,
/// generates AI-powered insights, and optionally sends email notifications.
async fn run_daily_analysis(send_email: bool) -> Result<()> {
    info!("Starting daily analysis");

    // Fetch CSI 300 ETF data
    let data = data_fetcher::fetch_hs300_data().await?;
    info!("Retrieved {} data points", data.len());

    // Analyze data
    let analysis = analyzer::analyze_daily_data(&data).await?;
    info!(
        "Analysis completed, price change: {:.2}%",
        analysis.price_change_pct
    );

    // Generate intelligent analysis using Gemini
    let gemini_analysis = gemini_client::generate_daily_analysis(&analysis).await?;
    info!("Gemini analysis completed");

    // Generate summary report
    let report = format_daily_report(&analysis, &gemini_analysis);
    println!("{}", report);

    // Send email notification
    if send_email {
        email_sender::send_email("Daily Investment Analysis Report", &report).await?;
        info!("Email sent successfully");
    }

    Ok(())
}

/// Execute weekly investment analysis
///
/// Performs both daily analysis and additional weekly-specific analysis
/// including trend analysis and volume studies.
async fn run_weekly_analysis(send_email: bool) -> Result<()> {
    info!("Starting weekly analysis");

    // Also execute daily analysis
    run_daily_analysis(false).await?;

    // Fetch weekly data for weekly analysis
    let weekly_data = data_fetcher::fetch_weekly_hs300_data().await?;
    let weekly_analysis = analyzer::analyze_weekly_data(&weekly_data).await?;
    let gemini_analysis = gemini_client::generate_weekly_analysis(&weekly_analysis).await?;
    let report = format_weekly_report(&weekly_analysis, &gemini_analysis);

    println!("{}", report);

    if send_email {
        email_sender::send_email("Weekly Investment Analysis Report", &report).await?;
    }

    Ok(())
}

/// Execute monthly investment analysis
///
/// Performs both daily analysis and comprehensive monthly analysis
/// including long-term trend assessment and market outlook.
async fn run_monthly_analysis(send_email: bool) -> Result<()> {
    info!("Starting monthly analysis");

    // Also execute daily analysis
    run_daily_analysis(false).await?;

    // Fetch monthly data for monthly analysis
    let monthly_data = data_fetcher::fetch_monthly_hs300_data().await?;
    let monthly_analysis = analyzer::analyze_monthly_data(&monthly_data).await?;
    let gemini_analysis = gemini_client::generate_monthly_analysis(&monthly_analysis).await?;
    let report = format_monthly_report(&monthly_analysis, &gemini_analysis);

    println!("{}", report);

    if send_email {
        email_sender::send_email("Monthly Investment Analysis Report", &report).await?;
    }

    Ok(())
}

fn format_daily_report(analysis: &models::DailyAnalysis, gemini_analysis: &str) -> String {
    format!(
        "📊 CSI 300 ETF Daily Analysis Report\n\n\
        📅 Date: {}\n\n\
        💰 Current Price: {:.2} CNY\n\
        📈 Price Change: {:.2}%\n\
        📊 Relative to High: {:.2}%\n\
        📉 Relative to Low: {:.2}%\n\n\
        🤖 AI Analysis:\n{}\n",
        analysis.date.format("%Y-%m-%d"),
        analysis.current_price,
        analysis.price_change_pct,
        analysis.relative_to_high,
        analysis.relative_to_low,
        gemini_analysis
    )
}

fn format_weekly_report(analysis: &models::WeeklyAnalysis, gemini_analysis: &str) -> String {
    format!(
        "📈 CSI 300 ETF Weekly Analysis Report\n\n\
        📅 Period: {} to {}\n\n\
        💰 Start Price: {:.2} CNY\n\
        💰 End Price: {:.2} CNY\n\
        📈 Weekly Change: {:.2}%\n\
        📊 Highest: {:.2} CNY ({})\n\
        📉 Lowest: {:.2} CNY ({})\n\n\
        🤖 AI Analysis:\n{}\n",
        analysis.start_date.format("%Y-%m-%d"),
        analysis.end_date.format("%Y-%m-%d"),
        analysis.start_price,
        analysis.end_price,
        analysis.weekly_change_pct,
        analysis.highest_price,
        analysis.highest_date.format("%Y-%m-%d"),
        analysis.lowest_price,
        analysis.lowest_date.format("%Y-%m-%d"),
        gemini_analysis
    )
}

fn format_monthly_report(analysis: &models::MonthlyAnalysis, gemini_analysis: &str) -> String {
    format!(
        "📊 CSI 300 ETF Monthly Analysis Report\n\n\
        📅 Month: {}-{}\n\n\
        💰 Start Price: {:.2} CNY\n\
        💰 End Price: {:.2} CNY\n\
        📈 Monthly Change: {:.2}%\n\
        📊 Highest: {:.2} CNY ({})\n\
        📉 Lowest: {:.2} CNY ({})\n\n\
        🤖 AI Analysis:\n{}\n",
        analysis.year,
        analysis.month,
        analysis.start_price,
        analysis.end_price,
        analysis.monthly_change_pct,
        analysis.highest_price,
        analysis.highest_date.format("%Y-%m-%d"),
        analysis.lowest_price,
        analysis.lowest_date.format("%Y-%m-%d"),
        gemini_analysis
    )
}
