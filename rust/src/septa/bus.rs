use chrono::{DateTime, Local};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time;

use super::{BusRouteData, BusStatusData, BusStatusInfo};

static BUS_DATA: Lazy<Arc<RwLock<BusStatusData>>> = Lazy::new(|| {
    Arc::new(RwLock::new(BusStatusData {
        num: 0,
        time: "never".to_string(),
        time_t: -1,
        status: BusStatusInfo {
            status: "(unknown)".to_string(),
            css_class: "status-unknown".to_string(),
            message: "Waiting for initial data...".to_string(),
            summary: "Waiting for initial data...".to_string(),
            suspended: vec![],
        },
    }))
});

static RAW_DATA: Lazy<Arc<RwLock<Vec<SeptaBusResponse>>>> =
    Lazy::new(|| Arc::new(RwLock::new(vec![])));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeptaBusResponse {
    route: String,
    // Add other fields as needed
}

pub async fn boot() {
    // Start fetching data immediately
    tokio::spawn(async {
        fetch_loop().await;
    });

    // Start age checking
    tokio::spawn(async {
        age_check_loop().await;
    });
}

pub fn get_data() -> BusStatusData {
    BUS_DATA.read().unwrap().clone()
}

pub fn get_raw_data() -> Vec<SeptaBusResponse> {
    RAW_DATA.read().unwrap().clone()
}

async fn fetch_loop() {
    loop {
        match fetch_and_process().await {
            Ok(data) => {
                let mut bus_data = BUS_DATA.write().unwrap();
                *bus_data = data;
                tracing::info!("Bus data updated successfully");
            }
            Err(e) => {
                tracing::error!("Error fetching bus data: {}", e);
            }
        }

        // Wait 5 minutes before next fetch
        time::sleep(Duration::from_secs(300)).await;
    }
}

async fn fetch_and_process() -> Result<BusStatusData, Box<dyn std::error::Error>> {
    let url = "http://www3.septa.org/hackathon/Alerts/";
    
    tracing::info!("Fetching bus data from {}", url);
    let response = reqwest::get(url).await?;
    
    // Try to parse as array first
    let data: Vec<SeptaBusResponse> = match response.json().await {
        Ok(data) => data,
        Err(_) => {
            // If it fails, return empty data
            vec![]
        }
    };

    // Store raw data
    {
        let mut raw_data = RAW_DATA.write().unwrap();
        *raw_data = data.clone();
    }

    // Get current time
    let now: DateTime<Local> = Local::now();
    let time_str = now.format("%a %b %e, %Y %I:%M:%S %p").to_string();
    let time_t = now.timestamp();

    let num_routes = data.len();
    let suspended_routes = get_suspended_routes(&data);
    let status_str = determine_status(&suspended_routes);
    let css_class = get_status_class(&status_str);
    let (message, summary) = get_message(num_routes, &suspended_routes, &status_str);

    Ok(BusStatusData {
        num: num_routes,
        time: time_str,
        time_t,
        status: BusStatusInfo {
            status: status_str,
            css_class,
            message,
            summary,
            suspended: suspended_routes.iter().map(|r| r.route.clone()).collect(),
        },
    })
}

fn get_suspended_routes(_data: &[SeptaBusResponse]) -> Vec<BusRouteData> {
    // For now, return empty. The actual logic would check for suspended routes
    vec![]
}

fn determine_status(suspended: &[BusRouteData]) -> String {
    if suspended.len() >= 3 {
        "fucked".to_string()
    } else if !suspended.is_empty() {
        "a little fucked".to_string()
    } else {
        "not fucked".to_string()
    }
}

fn get_status_class(status: &str) -> String {
    match status {
        "not fucked" => "status-not-fcked".to_string(),
        "a little fucked" => "status-a-little-fcked".to_string(),
        "fucked" => "status-fcked".to_string(),
        _ => "status-unknown".to_string(),
    }
}

fn get_message(
    num_total: usize,
    suspended: &[BusRouteData],
    status: &str,
) -> (String, String) {
    let num_suspended = suspended.len();

    match status {
        "fucked" => (
            format!(
                "{} out of {} bus routes are suspended! You may want to look into alternate forms of transportation.",
                num_suspended, num_total
            ),
            format!("{} out of {} bus routes are suspended!", num_suspended, num_total),
        ),
        "a little fucked" => (
            format!(
                "{} out of {} bus routes are suspended.",
                num_suspended, num_total
            ),
            format!("{} out of {} bus routes are suspended.", num_suspended, num_total),
        ),
        "not fucked" => (
            "All bus routes are running normally!".to_string(),
            "All bus routes are running normally!".to_string(),
        ),
        _ => (
            "No bus data was returned to me by SEPTA's API.".to_string(),
            "No bus data currently available.".to_string(),
        ),
    }
}

async fn age_check_loop() {
    loop {
        time::sleep(Duration::from_secs(300)).await; // Check every 5 minutes

        let data = BUS_DATA.read().unwrap().clone();
        let now = Local::now().timestamp();
        let age = now - data.time_t;
        let max_age = 600; // 10 minutes

        if age > max_age {
            tracing::warn!(
                "Bus data is {} seconds old (max {}). Data may be stale.",
                age,
                max_age
            );
        } else {
            tracing::info!(
                "Bus data is {} seconds old, which is less than {} seconds.",
                age,
                max_age
            );
        }
    }
}
