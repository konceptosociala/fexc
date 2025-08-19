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

        ui.add(egui::Label::new(
            egui::RichText::new(self.app.i18n("settings").to_uppercase()) 
                .size(28.0)
                .strong()
                .family(egui::FontFamily::Monospace)
        ));

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(format!("{}:", self.app.i18n("theme")));
                ui.label(format!("{}:", self.app.i18n("language")));
            });

            ui.vertical(|ui| {
                let mut responses = vec![];

                let dark_label = self.app.i18n("dark").to_owned();
                let light_label = self.app.i18n("light").to_owned();

                egui::ComboBox::from_id_salt("theme")
                    .selected_text(match self.app.config.theme {
                        egui::Theme::Light => &light_label,
                        egui::Theme::Dark => &dark_label,
                    })
                    .show_ui(ui, |ui| {
                        responses.push(
                            ui.selectable_value(&mut self.app.config.theme, egui::Theme::Dark, &dark_label)
                        );
                        responses.push(
                            ui.selectable_value(&mut self.app.config.theme, egui::Theme::Light, &light_label)
                        );
                    });

                egui::ComboBox::from_id_salt("lang")
                    .selected_text(match self.app.config.language {
                        Language::English => "English",
                        Language::Ukrainian => "Українська",
                    })
                    .show_ui(ui, |ui| {
                        responses.push(
                            ui.selectable_value(&mut self.app.config.language, Language::Ukrainian, "Українська")
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