use crate::{app::Fexc, i18n::Language};

pub struct SettingsPage<'app> {
    app: &'app mut Fexc,
}

impl<'app> SettingsPage<'app> {
    pub fn new(app: &'app mut Fexc) -> Self {
        SettingsPage { app }
    }
}

impl egui::Widget for SettingsPage<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.style_mut().spacing.item_spacing = egui::vec2(10.0, 10.0);

        ui.heading("Settings Page");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label("Theme:");
                ui.label("Language:");
            });

            ui.vertical(|ui| {
                let mut responses = vec![];

                egui::ComboBox::from_id_salt("theme")
                    .selected_text(match self.app.config.theme {
                        egui::Theme::Light => "Light",
                        egui::Theme::Dark => "Dark",
                    })
                    .show_ui(ui, |ui| {
                        responses.push(
                            ui.selectable_value(&mut self.app.config.theme, egui::Theme::Dark, "Dark")
                        );
                        responses.push(
                            ui.selectable_value(&mut self.app.config.theme, egui::Theme::Light, "Light")
                        );
                    });

                egui::ComboBox::from_id_salt("lang")
                    .selected_text(match self.app.config.language {
                        Language::English => "English",
                        Language::Ukrainian => "Ukrainian",
                    })
                    .show_ui(ui, |ui| {
                        responses.push(
                            ui.selectable_value(&mut self.app.config.language, Language::Ukrainian, "Ukrainian")
                        );
                        responses.push(
                            ui.selectable_value(&mut self.app.config.language, Language::English, "English")
                        );
                    });

                if responses.iter().any(|r| r.changed()) {
                    self.app.config.save().unwrap_or_else(|e| {
                        log::error!("Failed to save config: {e}");
                    });
                }
            });
        });

        ui.label("")
    }
}