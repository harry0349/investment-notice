use chrono::{DateTime, Datelike, Timelike, Utc, Weekday};
use std::time::Duration;
use tokio::time;
use tracing::{info, warn};

/// Check if the date is a workday
#[allow(dead_code)]
pub fn is_workday(date: DateTime<Utc>) -> bool {
    let weekday = date.weekday();
    !matches!(weekday, Weekday::Sat | Weekday::Sun)
}

/// Check if the date is the last workday of the month
#[allow(dead_code)]
pub fn is_last_workday_of_month(date: DateTime<Utc>) -> bool {
    let current_month = date.month();
    let next_month = if current_month == 12 {
        1
    } else {
        current_month + 1
    };
    let next_year = if current_month == 12 {
        date.year() + 1
    } else {
        date.year()
    };

    let next_month_first = Utc::now()
        .with_year(next_year)
        .unwrap()
        .with_month(next_month)
        .unwrap()
        .with_day(1)
        .unwrap()
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap();

    let days_until_next_month = (next_month_first - date).num_days();

    // If there are 1-3 days until next month and it's a workday, consider it the last workday of the month
    (1..=3).contains(&days_until_next_month) && is_workday(date)
}

/// Check if the date is Friday
#[allow(dead_code)]
pub fn is_friday(date: DateTime<Utc>) -> bool {
    date.weekday() == Weekday::Fri
}

/// Calculate next execution time
#[allow(dead_code)]
pub fn get_next_execution_time(mode: &str, current_time: DateTime<Utc>) -> DateTime<Utc> {
    let target_hour = 20; // 8 PM
    let target_minute = 0;

    match mode {
        "daily" => {
            // Daily execution: next workday at 8 PM
            let mut next_time = current_time
                .with_hour(target_hour)
                .unwrap()
                .with_minute(target_minute)
                .unwrap()
                .with_second(0)
                .unwrap();

            if next_time <= current_time {
                // If it's not yet 8 PM today, schedule for tomorrow
                next_time += chrono::Duration::days(1);
            }

            // If it's weekend, schedule for next workday
            while !is_workday(next_time) {
                next_time += chrono::Duration::days(1);
            }

            next_time
        }
        "weekly" => {
            // Weekly execution: every Friday
            let mut next_time = current_time
                .with_hour(target_hour)
                .unwrap()
                .with_minute(target_minute)
                .unwrap()
                .with_second(0)
                .unwrap();

            // Find next Friday
            while next_time.weekday() != Weekday::Fri || next_time <= current_time {
                next_time += chrono::Duration::days(1);
            }

            next_time
        }
        "monthly" => {
            // Monthly execution: last workday of the month
            let next_time = current_time
                .with_hour(target_hour)
                .unwrap()
                .with_minute(target_minute)
                .unwrap()
                .with_second(0)
                .unwrap();

            // Find first workday of next month
            let mut check_date = next_time + chrono::Duration::days(1);
            while !is_last_workday_of_month(check_date) {
                check_date += chrono::Duration::days(1);
            }

            check_date
                .with_hour(target_hour)
                .unwrap()
                .with_minute(target_minute)
                .unwrap()
                .with_second(0)
                .unwrap()
        }
        _ => {
            warn!("Unknown mode: {}, using daily mode", mode);
            get_next_execution_time("daily", current_time)
        }
    }
}

/// Start scheduler
#[allow(dead_code)]
pub async fn start_scheduler<F, Fut>(mode: &str, mut handler: F) -> !
where
    F: FnMut() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = ()> + Send,
{
    info!("Starting scheduler, mode: {}", mode);

    loop {
        let now = Utc::now();
        let next_execution = get_next_execution_time(mode, now);

        let wait_duration = (next_execution - now)
            .to_std()
            .unwrap_or(Duration::from_secs(60));

        info!(
            "Next execution time: {}, waiting {} seconds",
            next_execution.format("%Y-%m-%d %H:%M:%S UTC"),
            wait_duration.as_secs()
        );

        time::sleep(wait_duration).await;

        info!("Executing scheduled task");
        handler().await;

        // Brief delay to avoid overly frequent execution
        time::sleep(Duration::from_secs(5)).await;
    }
}

/// Get current time information
#[allow(dead_code)]
pub fn get_time_info(date: DateTime<Utc>) -> (bool, bool, bool) {
    let is_workday = is_workday(date);
    let is_friday = is_friday(date);
    let is_last_workday = is_last_workday_of_month(date);

    (is_workday, is_friday, is_last_workday)
}

/// Format time information
#[allow(dead_code)]
pub fn format_time_info(date: DateTime<Utc>) -> String {
    let (is_workday, is_friday, is_last_workday) = get_time_info(date);

    let weekday_str = match date.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };

    format!(
        "Date: {}, Weekday: {}, Workday: {}, Friday: {}, Last workday of month: {}",
        date.format("%Y-%m-%d %H:%M:%S"),
        weekday_str,
        if is_workday { "Yes" } else { "No" },
        if is_friday { "Yes" } else { "No" },
        if is_last_workday { "Yes" } else { "No" }
    )
}
