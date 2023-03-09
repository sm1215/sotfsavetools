use egui::Button;

use crate::save::{Save, GameSettingsInner};

use super::SaveTool;

#[derive(Debug, Clone, Default)]
pub struct ToolSeason {
    current_game_mode: String,
    current_game_type: String,
    current_season_name: String,
    is_custom_game: bool,
}

impl SaveTool for ToolSeason {
    fn new(save: &Save) -> Self {
        let mut tool = Self::default();
        tool.fetch_current_game_mode(save);
        tool.fetch_current_game_type(save);
        tool.fetch_is_custom_game();
        tool.fetch_current_season_name(save);
        tool
    }

    fn render(&mut self, save: &mut Save, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui
                .add_enabled(self.is_custom_game, Button::new("Fix Season"))
                .clicked()
            {
                self.fix_season(save);
            }

            if !self.is_custom_game {
                ui.label("Not a custom game.");
            } else {
                ui.label(&self.current_season_name);
            }
        });

    }
}

impl ToolSeason {
    // game_mode can be set to "Normal" or "Custom" or probably also "Hard" but this fix is only for normal mode right now
    pub fn fetch_current_game_mode(&mut self, save: &Save) {
        let settings: Vec<GameSettingsInner> = save.game_setup.data.game_setup.settings.clone();

        self.current_game_mode = settings
            .into_iter()
            .filter(|entry| entry.name == "Mode")
            .collect::<Vec<GameSettingsInner>>()[0]
            .string_value
            .clone();
    }

    // game_type can be set to "Normal" or "Custom"
    pub fn fetch_current_game_type(&mut self, save: &Save) {
        self.current_game_type = save.game_state.data.game_state.game_type.clone();
    }

    // a Custom game is represented in two places
    // game_setup where it actively influences the user's game
    // game_state where it describes the user's save in the save / load interface in-game
    pub fn fetch_is_custom_game(&mut self) {
        self.is_custom_game = self.current_game_mode == "Custom" || self.current_game_type == "Custom";
    }

    pub fn fetch_current_season_name(&mut self, save: &Save) {
        self.current_season_name = match save.weather_system.data.weather_system.current_season {
            0 => "Spring".to_string(),
            1 => "Summer".to_string(),
            2 => "Fall".to_string(),
            3 => "Winter".to_string(),
            _ => panic!("Unknown current season name."),
        }
    }

    // evaluate what the host's current season should be after changing offset
    pub fn determine_new_season(&mut self, save: &Save) -> i32 {
        let offset = save.weather_system.data.weather_system.starting_day_offset;
        let game_days = save.game_state.data.game_state.game_days;

        let settings: Vec<GameSettingsInner> = save.game_setup.data.game_setup.settings.clone();
        
        let starting_season = settings
            .into_iter()
            .filter(|entry| entry.name == "GameSetting.Environment.StartingSeason")
            .collect::<Vec<GameSettingsInner>>()[0]
            .string_value
            .clone();
 
        let starting_season_int = match starting_season.as_str() {
            "Spring" => 0,
            "Summer" => 1,
            "Fall" => 2,
            "Winter" => 3,
            _ => panic!("Unknown starting season"),
        };

        let number_of_seasons = 4;

        // figure out how far we've progressed through the current season out of all years passed
        let seasonal_progress = game_days as f32 / (number_of_seasons as f32 * offset);

        // disregard how many years have passed, only interested in progress through the current year
        let mantissa = seasonal_progress - seasonal_progress.floor();

        // map progress to the integers used for each season
        // i.e. spring = 0, summer = 1, etc...
        let mantissa_times_seasons = mantissa * number_of_seasons as f32;

        // adjust for the starting season
        let season = mantissa_times_seasons.floor() as i32 + starting_season_int;
        let actual_season = season % number_of_seasons;

        actual_season
    }

    pub fn fetch_game_uid(&mut self, save: &Save) -> GameSettingsInner {
        let settings: Vec<GameSettingsInner> = save.game_setup.data.game_setup.settings.clone();

        settings
            .into_iter()
            .filter(|entry| entry.name == "UID")
            .collect::<Vec<GameSettingsInner>>()[0]
            .clone()
    }

    pub fn fix_season(&mut self, save: &mut Save) {
        // set season settings back to normal defaults
        save.weather_system.data.weather_system.starting_day_offset = 5.0;
        save.weather_system.data.weather_system.current_season = self.determine_new_season(save);

        // set game settings to default normal config
        let normal_mode = GameSettingsInner::get_normal_mode();
        
        // preserve the player's current game uid object
        let game_uid = self.fetch_game_uid(&save);
        
        save.game_setup.data.game_setup.settings = vec![normal_mode, game_uid];

        // update the game state's game type.
        // this is more of a cosmetic change, it appears in the save / load menu in-game
        // and is based on the game_setup's mode.
        save.game_state.data.game_state.game_type = "Normal".to_string();
    }
}
