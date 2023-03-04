use eframe::epaint::ahash::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{json::JsonString, serde_as};

#[serde_as]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct GameSetup {
    #[serde_as(as = "JsonString")]
    pub game_setup: GameSetupInner,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct GameSetupInner {
    pub version: String,

    #[serde(rename = "_settings")]
    pub settings: Vec<GameSettingsInner>,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct GameSettingsInner {
    pub name: String,
    pub setting_type: i32,
    pub version: i32,
    pub bool_value: bool,
    pub int_value: i32,
    pub float_value: f32,
    pub string_value: String,
    pub protected: bool,
    pub float_array_value: Vec<f32>,
    pub is_set: bool,
}

impl GameSettingsInner {
    pub fn get_normal_mode() -> Self {
        Self {
            name: "Mode".to_string(),
            setting_type: 3,
            version: 0,
            bool_value: false,
            int_value: 0,
            float_value: 0.0,
            string_value: "Normal".to_string(),
            protected: false,
            float_array_value: vec![],
            is_set: false
        }
    }
}
