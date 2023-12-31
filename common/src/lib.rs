use chrono::DateTime;
use serde::{Serialize,Deserialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct MetricV1 {
    pub client_id: u32,
    pub auth_token: String,
    pub datetime: DateTime<chrono::Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsResponse {
    pub status: String,
    pub count: u32
}
