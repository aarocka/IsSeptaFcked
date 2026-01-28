pub mod rr;
pub mod bus;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainData {
    pub number: String,
    pub from: String,
    pub to: String,
    pub late: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusRouteData {
    pub route: String,
    pub suspended: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusData {
    pub num: usize,
    pub time: String,
    pub time_t: i64,
    pub late: LateTrain,
    pub late_average: f64,
    pub status: StatusInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LateTrain {
    #[serde(rename = "10")]
    pub ten: Vec<TrainData>,
    #[serde(rename = "30")]
    pub thirty: Vec<TrainData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusInfo {
    pub status: String,
    pub css_class: String,
    pub message: String,
    pub summary: String,
    pub late: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusStatusData {
    pub num: usize,
    pub time: String,
    pub time_t: i64,
    pub status: BusStatusInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusStatusInfo {
    pub status: String,
    pub css_class: String,
    pub message: String,
    pub summary: String,
    pub suspended: Vec<String>,
}
