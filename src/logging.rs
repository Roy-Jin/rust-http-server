use std::time::Instant;

use axum::http::StatusCode;
use colored::*;

pub fn log_request(method: &str, uri: &str, status: StatusCode, start: Instant) {
    let latency = start.elapsed();
    let latency_ms = latency.as_millis();
    
    let method_colored = match method {
        "GET" => method.green(),
        "POST" => method.yellow(),
        "PUT" => method.blue(),
        "DELETE" => method.red(),
        "PATCH" => method.magenta(),
        _ => method.white(),
    };
    
    let status_colored = match status.as_u16() {
        200..=299 => status.to_string().green(),
        300..=399 => status.to_string().yellow(),
        400..=499 => status.to_string().red(),
        500..=599 => status.to_string().red().bold(),
        _ => status.to_string().white(),
    };
    
    let timestamp = chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string().dimmed();
    
    println!(
        "{} {} {} {} {}ms",
        timestamp,
        method_colored.bold(),
        uri,
        status_colored,
        latency_ms.to_string().cyan()
    );
}
