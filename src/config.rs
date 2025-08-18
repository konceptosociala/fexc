pub struct Config {
    pub theme: egui::Theme,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            theme: egui::Theme::Dark,
        }
    }
}