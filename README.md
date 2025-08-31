# CSI 300 ETF Investment Notification System

A Rust-based A-Share investment notification system focused on analyzing
the CSI 300 ETF index, providing daily, weekly, and monthly investment
analysis reports with email notifications.

## Features

- 📊 **Real-time Data Fetching**: Support for multiple data sources
  (TuShare, Alpha Vantage)
- 🤖 **AI Intelligent Analysis**: Integration with Google Gemini for
  professional investment analysis
- 📧 **Email Notifications**: Automatic delivery of analysis reports to
  specified email addresses
- ⏰ **Scheduled Execution**: Support for daily, weekly, and monthly
  analysis reports
- 📈 **Technical Indicators**: Provides key metrics like price changes
  and relative positions
- 🔧 **Flexible Configuration**: Environment variable-based
  configuration management

## Quick Start

### 1. Environment Requirements

- Rust 2024 edition or higher
- Network connection (for fetching stock data and sending emails)

### 2. Install Dependencies

```bash
cargo build --release
```

### 3. Configure Environment Variables

Copy the environment variables example file:

```bash
cp env-example.txt .env
```

Edit the `.env` file with your configuration information:

```bash
# Data Source API Keys
TUSHARE_TOKEN=your_tushare_token_here
ALPHA_VANTAGE_API_KEY=your_alpha_vantage_api_key_here

# Gemini AI Configuration
GEMINI_API_KEY=your_gemini_api_key_here

# Email Configuration
SMTP_SERVER=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your_email@gmail.com
SMTP_PASSWORD=your_app_password_here
FROM_EMAIL=your_email@gmail.com
TO_EMAILS=recipient1@example.com,recipient2@example.com
```

### 4. Run the Program

#### Daily Analysis

```bash
cargo run -- --mode daily --send-email
```

#### Weekly Analysis

```bash
cargo run -- --mode weekly --send-email
```

#### Monthly Analysis

```bash
cargo run -- --mode monthly --send-email
```

#### Test Run (without sending emails)

```bash
cargo run -- --mode daily
```

## API Configuration Guide

### TuShare API (Recommended)

1. Visit [TuShare Official Website](https://tushare.pro/)
2. Register an account and obtain API Token
3. Set the Token in environment variable `TUSHARE_TOKEN`

### Alpha Vantage API (Backup)

1. Visit [Alpha Vantage](https://www.alphavantage.co/)
2. Register an account and obtain API Key
3. Set the Key in environment variable `ALPHA_VANTAGE_API_KEY`

### Google Gemini API

1. Visit [Google AI Studio](https://makersuite.google.com/app/apikey)
2. Create API Key
3. Set the Key in environment variable `GEMINI_API_KEY`

### Email Configuration

#### Gmail Configuration

1. Enable two-factor authentication
2. Generate application-specific password
3. Use application-specific password as `SMTP_PASSWORD`

#### Other Email Providers

Adjust `SMTP_SERVER` and `SMTP_PORT` according to your email provider:

- QQ Mail: `smtp.qq.com:587`
- 163 Mail: `smtp.163.com:587`
- Outlook: `smtp-mail.outlook.com:587`

## Analysis Metrics Description

### Daily Analysis Metrics

- **Price Change**: Price change compared to the previous trading day
- **Relative to High**: Current price position as percentage between
  historical high/low prices
- **Relative to Low**: Current price distance from historical low as
  percentage

### Weekly Analysis Metrics

- **Weekly Change**: Overall price change within a week
- **Highest/Lowest Price**: Highest and lowest prices within the week
  and their dates
- **Volume Analysis**: Weekly average volume and total volume

### Monthly Analysis Metrics

- **Monthly Change**: Overall price change within a month
- **Important Levels**: Key support and resistance levels within the month
- **Market Outlook**: Market forecast for the next month

## GitHub Actions Configuration

Create `.github/workflows/investment-notice.yml`:

```yaml
name: Investment Notice

on:
  schedule:
    # 每个工作日晚上8点
    - cron: '0 20 * * 1-5'
    # 每周五晚上8点 (额外执行)
    - cron: '0 20 * * 5'
    # 每月最后一天晚上8点 (额外执行)
    - cron: '0 20 28-31 * *'
  workflow_dispatch:

jobs:
  analyze:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4

    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Cache dependencies
      uses: Swatinem/rust-cache@v2

    - name: Build
      run: cargo build --release

    - name: Run Analysis
      env:
        TUSHARE_TOKEN: ${{ secrets.TUSHARE_TOKEN }}
        ALPHA_VANTAGE_API_KEY: ${{ secrets.ALPHA_VANTAGE_API_KEY }}
        GEMINI_API_KEY: ${{ secrets.GEMINI_API_KEY }}
        SMTP_SERVER: ${{ secrets.SMTP_SERVER }}
        SMTP_PORT: ${{ secrets.SMTP_PORT }}
        SMTP_USERNAME: ${{ secrets.SMTP_USERNAME }}
        SMTP_PASSWORD: ${{ secrets.SMTP_PASSWORD }}
        FROM_EMAIL: ${{ secrets.FROM_EMAIL }}
        TO_EMAILS: ${{ secrets.TO_EMAILS }}
      run: |
        # Determine run mode based on trigger time
        if [ "$(date +%u)" = "5" ]; then
          # Friday: run weekly analysis
          cargo run --release -- --mode weekly --send-email
        elif [ "$(date +%d)" -ge 28 ] && [ "$(date +%u)" -le 5 ]; then
          # Month-end workday: run monthly analysis
          cargo run --release -- --mode monthly --send-email
        else
          # Regular workday: run daily analysis
          cargo run --release -- --mode daily --send-email
        fi
```

## Code Quality Checks

Run formatting and checks:

```bash
# Format code
cargo fmt

# Run Clippy checks
cargo clippy -- -D warnings

# Run all tests
cargo test
```

## Project Structure

```text
src/
├── main.rs              # Main program entry point
├── models.rs            # Data model definitions
├── data_fetcher.rs      # Data fetching module
├── analyzer.rs          # Data analysis module
├── gemini_client.rs     # Gemini AI integration
├── email_sender.rs      # Email sending module
└── scheduler.rs         # Scheduling logic module
```

## Development Notes

### Adding New Analysis Metrics

1. Define new data structures in `models.rs`
2. Implement analysis logic in `analyzer.rs`
3. Add display in report formatting functions in `main.rs`

### Supporting New Data Sources

1. Implement new fetch function in `data_fetcher.rs`
2. Add fallback logic in `fetch_hs300_data` function

### Custom AI Analysis

Modify prompts in `gemini_client.rs` to customize AI analysis content and style.

## License

MIT License

## Disclaimer

This project is for educational and research purposes only and does not
constitute investment advice. Investment involves risks, and market entry
should be approached with caution. Please make decisions based on your own
risk tolerance and investment experience.
