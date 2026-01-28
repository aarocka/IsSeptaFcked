use chrono::{DateTime, Local};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time;

use super::{LateTrain, StatusData, StatusInfo, TrainData};

static RR_DATA: Lazy<Arc<RwLock<StatusData>>> = Lazy::new(|| {
    Arc::new(RwLock::new(StatusData {
        num: 0,
        time: "never".to_string(),
        time_t: -1,
        late: LateTrain {
            ten: vec![],
            thirty: vec![],
        },
        late_average: 0.0,
        status: StatusInfo {
            status: "(unknown)".to_string(),
            css_class: "status-unknown".to_string(),
            message: "Waiting for initial data...".to_string(),
            summary: "Waiting for initial data...".to_string(),
            late: vec![],
        },
    }))
});

static RAW_DATA: Lazy<Arc<RwLock<Vec<SeptaTrainResponse>>>> =
    Lazy::new(|| Arc::new(RwLock::new(vec![])));

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SeptaTrainResponse {
    trainno: String,
    #[serde(rename = "SOURCE")]
    source: String,
    dest: String,
    late: i32,
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

pub fn get_data() -> StatusData {
    RR_DATA.read().unwrap().clone()
}

pub fn get_raw_data() -> Vec<SeptaTrainResponse> {
    RAW_DATA.read().unwrap().clone()
}

async fn fetch_loop() {
    loop {
        match fetch_and_process().await {
            Ok(data) => {
                let mut rr_data = RR_DATA.write().unwrap();
                *rr_data = data;
                tracing::info!("RR data updated successfully");
            }
            Err(e) => {
                tracing::error!("Error fetching RR data: {}", e);
            }
        }

        // Wait 60 seconds before next fetch
        time::sleep(Duration::from_secs(60)).await;
    }
}

async fn fetch_and_process() -> Result<StatusData, Box<dyn std::error::Error>> {
    let url = "http://www3.septa.org/hackathon/TrainView/";
    
    tracing::info!("Fetching RR data from {}", url);
    let response = reqwest::get(url).await?;
    let data: Vec<SeptaTrainResponse> = response.json().await?;

    // Store raw data
    {
        let mut raw_data = RAW_DATA.write().unwrap();
        *raw_data = data.clone();
    }

    // Transform data
    let mut trains = HashMap::new();
    let mut count = 0;

    for train in data {
        // Skip trains with 999 minute delay (out of service)
        if train.late == 999 {
            continue;
        }

        trains.insert(
            train.trainno.clone(),
            TrainData {
                number: train.trainno,
                from: train.source,
                to: train.dest,
                late: train.late,
            },
        );
        count += 1;
    }

    // Get current time
    let now: DateTime<Local> = Local::now();
    let time_str = now.format("%a %b %e, %Y %I:%M:%S %p").to_string();
    let time_t = now.timestamp();

    // Calculate late trains
    let trains_vec: Vec<TrainData> = trains.values().cloned().collect();
    let late_10 = get_late(&trains_vec, 10, 29);
    let late_30 = get_late(&trains_vec, 30, 86400);

    let late_average = calculate_late_average(&late_10, &late_30);
    let status_str = determine_status(&late_10, &late_30);
    let css_class = get_status_class(&status_str);

    let mut late_train_names = Vec::new();
    late_train_names.extend(get_late_train_names(&late_10));
    late_train_names.extend(get_late_train_names(&late_30));

    let (message, summary) = get_message(count, &late_10, &late_30, late_average, &status_str);

    Ok(StatusData {
        num: count,
        time: time_str,
        time_t,
        late: LateTrain {
            ten: late_10,
            thirty: late_30,
        },
        late_average,
        status: StatusInfo {
            status: status_str,
            css_class,
            message,
            summary,
            late: late_train_names,
        },
    })
}

fn get_late(trains: &[TrainData], min: i32, max: i32) -> Vec<TrainData> {
    trains
        .iter()
        .filter(|t| t.late >= min && t.late <= max)
        .cloned()
        .collect()
}

fn calculate_late_average(late_10: &[TrainData], late_30: &[TrainData]) -> f64 {
    let mut total_late = 0;
    let mut count = 0;

    for train in late_10.iter().chain(late_30.iter()) {
        total_late += train.late;
        count += 1;
    }

    if count == 0 {
        return 0.0;
    }

    let avg = total_late as f64 / count as f64;
    (avg * 10.0).round() / 10.0
}

fn determine_status(late_10: &[TrainData], late_30: &[TrainData]) -> String {
    if late_30.len() >= 5 {
        "turbo fucked".to_string()
    } else if !late_30.is_empty() {
        "fucked".to_string()
    } else if !late_10.is_empty() {
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
        "turbo fucked" => "status-turbo-fcked".to_string(),
        _ => "status-unknown".to_string(),
    }
}

fn get_late_train_names(trains: &[TrainData]) -> Vec<String> {
    trains
        .iter()
        .map(|t| format!("#{} ({} min late)", t.number, t.late))
        .collect()
}

fn get_message(
    num_total: usize,
    late_10: &[TrainData],
    late_30: &[TrainData],
    late_average: f64,
    status: &str,
) -> (String, String) {
    let num_late = late_10.len() + late_30.len();

    match status {
        "turbo fucked" => (
            format!(
                "{} out of {} trains are late! (Avg: {} min late) Time to <a href=\"https://www.uber.com/invite/uberspotcat\">grab an Uber.</a>",
                num_late, num_total, late_average
            ),
            format!("{} out of {} trains are late!", num_late, num_total),
        ),
        "fucked" => (
            format!(
                "{} out of {} trains are late! (Avg: {} min late) You may want to look into alternate forms of transportation.",
                num_late, num_total, late_average
            ),
            format!("{} out of {} trains are late!", num_late, num_total),
        ),
        "a little fucked" => (
            format!(
                "{} out of {} trains are late. (Avg: {} min late) Check back here in a few minutes to see if things improve.",
                num_late, num_total, late_average
            ),
            format!("{} out of {} trains are late.", num_late, num_total),
        ),
        "not fucked" => (
            format!(
                "All {} trains that we know about are running on or close to on time!",
                num_total
            ),
            format!(
                "All {} trains that we know about are running on or close to on time!",
                num_total
            ),
        ),
        _ => (
            "No train data was returned to me by SEPTA's API.".to_string(),
            "No train data currently available.".to_string(),
        ),
    }
}

async fn age_check_loop() {
    loop {
        time::sleep(Duration::from_secs(300)).await; // Check every 5 minutes

        let data = RR_DATA.read().unwrap().clone();
        let now = Local::now().timestamp();
        let age = now - data.time_t;
        let max_age = 600; // 10 minutes

        if age > max_age {
            tracing::warn!(
                "RR data is {} seconds old (max {}). Data may be stale.",
                age,
                max_age
            );
        } else {
            tracing::info!(
                "RR data is {} seconds old, which is less than {} seconds.",
                age,
                max_age
            );
        }
    }
}
