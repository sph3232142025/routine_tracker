use chrono::{Local, Duration};
use crate::storage;

pub fn generate(period: &str) {
    let now = Local::now().timestamp();

    let from = match period {
        "daily" => now - 24 * 3600,
        "weekly" => now - 7 * 24 * 3600,
        "monthly" => now - 30 * 24 * 3600,
        _ => {
            println!("Unknown period");
            return;
        }
    };

    let rows = storage::fetch_sessions_between(from, now);

    println!("=== Report: {period} ===");
    for (app, activity, start, end) in rows {
        let mins = (end - start) / 60;
        println!("{app} | {activity} | {mins} minutes");
    }
}
