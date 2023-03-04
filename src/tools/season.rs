use egui::Button;

use crate::save::{Save};

use super::SaveTool;

#[derive(Debug, Clone, Default)]
pub struct ToolSeason {
    current_season: i32,
    current_season_name: String,
}

impl SaveTool for ToolSeason {
    fn new(save: &Save) -> Self {
        let mut tool = Self::default();
        tool.fetch_current_season_name(save);
        tool
    }

    fn render(&mut self, save: &mut Save, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui
                // TODO: determine if user is on custom settings and add bool check here
                .add_enabled(true, Button::new("Fix Season"))
                .clicked()
            {
                self.fix_season(save);
            }
        });

        ui.label(&*self.current_season_name);
    }
}

impl ToolSeason {
    // TODO: need to implement this
    // pub fn is_custom_game()

    pub fn fetch_current_season_name(&mut self, save: &Save) {
        self.current_season_name = match save.weather_system.data.weather_system.current_season {
            0 => "Spring".to_string(),
            1 => "Summer".to_string(),
            2 => "Fall".to_string(),
            3 => "Winter".to_string(),
            _ => "Unknown Season".to_string(),
        }
    }

    pub fn fix_season(&mut self, save: &mut Save) {
        // set season settings back to normal defaults
        save.weather_system.data.weather_system.current_season = 0;
        save.weather_system.data.weather_system.starting_day_offset = 5.0;
    }
}
