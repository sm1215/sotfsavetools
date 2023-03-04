use eframe::epaint::ahash::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{json::JsonString, serde_as};

#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct WeatherSystem {
    #[serde_as(as = "JsonString")]
    pub weather_system: WeatherSystemInner,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct WeatherSystemInner {
    pub version: String,

    #[serde(rename = "_cloudState")]
    pub cloud_state: i32,

    #[serde(rename = "_currentRainType")]
    pub current_rain_type: i32,
    #[serde(rename = "_currentSeason")]
    pub current_season: i32,
    
    #[serde(rename = "_startingDayOffset")]
    pub starting_day_offset: f32,
    #[serde(rename = "_wetnessVelocity")]
    pub wetness_velocity: f32,
    #[serde(rename = "_wetnessCurrent")]
    pub wetness_current: f32,
    #[serde(rename = "_wetnessTarget")]
    pub wetness_target: f32,

    #[serde(rename = "_isRaining")]
    pub is_raining: bool,
    #[serde(rename = "_rainBlocked")]
    pub rain_blocked: bool,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}
