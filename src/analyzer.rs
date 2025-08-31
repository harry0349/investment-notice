use crate::models::{DailyAnalysis, MonthlyAnalysis, StockData, WeeklyAnalysis};
use anyhow::Result;
use chrono::Datelike;
use tracing::info;

/// Analyze daily data
pub async fn analyze_daily_data(data: &[StockData]) -> Result<DailyAnalysis> {
    if data.is_empty() {
        return Err(anyhow::anyhow!("No data available for analysis"));
    }

    let latest = data.last().unwrap();
    let previous = if data.len() > 1 {
        &data[data.len() - 2]
    } else {
        latest
    };

    // Calculate price change percentage
    let price_change_pct = ((latest.close - previous.close) / previous.close) * 100.0;

    // Find historical high and low prices
    let historical_high = data
        .iter()
        .map(|d| d.high)
        .fold(f64::NEG_INFINITY, f64::max);
    let historical_low = data.iter().map(|d| d.low).fold(f64::INFINITY, f64::min);

    // Calculate relative position
    let relative_to_high =
        ((latest.close - historical_low) / (historical_high - historical_low)) * 100.0;
    let relative_to_low =
        ((historical_high - latest.close) / (historical_high - historical_low)) * 100.0;

    let analysis = DailyAnalysis {
        date: latest.date,
        current_price: latest.close,
        previous_price: previous.close,
        price_change_pct,
        relative_to_high,
        relative_to_low,
        historical_high,
        historical_low,
        volume: latest.volume,
    };

    info!(
        "Daily analysis completed: Price {:.2}, Change {:.2}%, Relative to High {:.2}%, Relative to Low {:.2}%",
        analysis.current_price,
        analysis.price_change_pct,
        analysis.relative_to_high,
        analysis.relative_to_low
    );

    Ok(analysis)
}

/// Analyze weekly data
pub async fn analyze_weekly_data(data: &[StockData]) -> Result<WeeklyAnalysis> {
    if data.is_empty() {
        return Err(anyhow::anyhow!("No data available for analysis"));
    }

    let start_data = &data[0];
    let end_data = data.last().unwrap();

    let weekly_change_pct = ((end_data.close - start_data.close) / start_data.close) * 100.0;

    // Find highest and lowest prices within the week
    let mut highest_price = f64::NEG_INFINITY;
    let mut lowest_price = f64::INFINITY;
    let mut highest_date = start_data.date;
    let mut lowest_date = start_data.date;
    let mut total_volume = 0u64;

    for stock_data in data {
        if stock_data.high > highest_price {
            highest_price = stock_data.high;
            highest_date = stock_data.date;
        }
        if stock_data.low < lowest_price {
            lowest_price = stock_data.low;
            lowest_date = stock_data.date;
        }
        total_volume += stock_data.volume;
    }

    let average_volume = total_volume as f64 / data.len() as f64;

    let analysis = WeeklyAnalysis {
        start_date: start_data.date,
        end_date: end_data.date,
        start_price: start_data.close,
        end_price: end_data.close,
        weekly_change_pct,
        highest_price,
        highest_date,
        lowest_price,
        lowest_date,
        average_volume,
        total_volume,
    };

    info!(
        "Weekly analysis completed: Weekly change {:.2}%, Highest price {:.2}, Lowest price {:.2}",
        analysis.weekly_change_pct, analysis.highest_price, analysis.lowest_price
    );

    Ok(analysis)
}

/// Analyze monthly data
pub async fn analyze_monthly_data(data: &[StockData]) -> Result<MonthlyAnalysis> {
    if data.is_empty() {
        return Err(anyhow::anyhow!("No data available for analysis"));
    }

    let start_data = &data[0];
    let end_data = data.last().unwrap();

    let monthly_change_pct = ((end_data.close - start_data.close) / start_data.close) * 100.0;

    // Find highest and lowest prices within the month
    let mut highest_price = f64::NEG_INFINITY;
    let mut lowest_price = f64::INFINITY;
    let mut highest_date = start_data.date;
    let mut lowest_date = start_data.date;
    let mut total_volume = 0u64;

    for stock_data in data {
        if stock_data.high > highest_price {
            highest_price = stock_data.high;
            highest_date = stock_data.date;
        }
        if stock_data.low < lowest_price {
            lowest_price = stock_data.low;
            lowest_date = stock_data.date;
        }
        total_volume += stock_data.volume;
    }

    let average_volume = total_volume as f64 / data.len() as f64;

    let analysis = MonthlyAnalysis {
        year: end_data.date.year(),
        month: end_data.date.month(),
        start_date: start_data.date,
        end_date: end_data.date,
        start_price: start_data.close,
        end_price: end_data.close,
        monthly_change_pct,
        highest_price,
        highest_date,
        lowest_price,
        lowest_date,
        average_volume,
        total_volume,
    };

    info!(
        "Monthly analysis completed: Monthly change {:.2}%, Highest price {:.2}, Lowest price {:.2}",
        analysis.monthly_change_pct, analysis.highest_price, analysis.lowest_price
    );

    Ok(analysis)
}

/// Calculate moving average
#[allow(dead_code)]
pub fn calculate_moving_average(data: &[StockData], period: usize) -> Vec<f64> {
    let mut averages = Vec::new();

    for i in 0..data.len() {
        if i < period - 1 {
            averages.push(0.0);
            continue;
        }

        let sum: f64 = data[i - period + 1..=i].iter().map(|d| d.close).sum();
        let average = sum / period as f64;
        averages.push(average);
    }

    averages
}

/// Calculate RSI indicator
#[allow(dead_code)]
pub fn calculate_rsi(data: &[StockData], period: usize) -> Vec<f64> {
    let mut rsi_values = Vec::new();

    if data.len() < period + 1 {
        return rsi_values;
    }

    let mut gains = Vec::new();
    let mut losses = Vec::new();

    // Calculate price changes
    for i in 1..data.len() {
        let change = data[i].close - data[i - 1].close;
        if change > 0.0 {
            gains.push(change);
            losses.push(0.0);
        } else {
            gains.push(0.0);
            losses.push(change.abs());
        }
    }

    // Calculate RSI
    for i in period - 1..gains.len() {
        let avg_gain = gains[i - period + 1..=i].iter().sum::<f64>() / period as f64;
        let avg_loss = losses[i - period + 1..=i].iter().sum::<f64>() / period as f64;

        if avg_loss == 0.0 {
            rsi_values.push(100.0);
        } else {
            let rs = avg_gain / avg_loss;
            let rsi = 100.0 - (100.0 / (1.0 + rs));
            rsi_values.push(rsi);
        }
    }

    rsi_values
}

/// Calculate MACD indicator
#[allow(dead_code)]
pub fn calculate_macd(
    data: &[StockData],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let fast_ema = calculate_ema(
        &data.iter().map(|d| d.close).collect::<Vec<_>>(),
        fast_period,
    );
    let slow_ema = calculate_ema(
        &data.iter().map(|d| d.close).collect::<Vec<_>>(),
        slow_period,
    );

    let mut macd_line = Vec::new();
    for i in 0..fast_ema.len().min(slow_ema.len()) {
        macd_line.push(fast_ema[i] - slow_ema[i]);
    }

    let signal_line = calculate_ema(&macd_line, signal_period);

    let mut histogram = Vec::new();
    for i in 0..macd_line.len().min(signal_line.len()) {
        histogram.push(macd_line[i] - signal_line[i]);
    }

    (macd_line, signal_line, histogram)
}

/// Calculate EMA
#[allow(dead_code)]
fn calculate_ema(data: &[f64], period: usize) -> Vec<f64> {
    let mut ema = Vec::new();
    let multiplier = 2.0 / (period as f64 + 1.0);

    if data.is_empty() {
        return ema;
    }

    // 第一个EMA值等于第一个价格
    ema.push(data[0]);

    for i in 1..data.len() {
        let ema_value = (data[i] * multiplier) + (ema[i - 1] * (1.0 - multiplier));
        ema.push(ema_value);
    }

    ema
}
