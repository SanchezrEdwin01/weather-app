use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherItem {
    pub id: i32,
    pub city: String,
    pub temperature: f64,
    pub description: String,
    pub humidity: f64,
    pub wind: Option<String>,
    pub visibility: Option<f64>,
    pub atmospheric_pressure: Option<f64>,
    pub sunrise: Option<NaiveTime>,
    pub sunset: Option<NaiveTime>,
    pub date_time: DateTime<Utc>,
}
