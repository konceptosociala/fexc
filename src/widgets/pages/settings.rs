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

        let mut responses = vec![];
        let dark_label = self.app.i18n("dark").to_owned();
        let light_label = self.app.i18n("light").to_owned();

        egui::Grid::new("settings_grid").spacing(egui::vec2(10.0, 10.0)).show(ui, |ui| {
            // Theme row
            ui.label(format!("{}:", self.app.i18n("theme")));
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
            ui.end_row();

            // Language row
            ui.label(format!("{}:", self.app.i18n("language")));
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
            ui.end_row();

            // Font size row
            ui.label(format!("{}:", self.app.i18n("editor_font_size")));
            responses.push(
                ui.add(
                    egui::DragValue::new(&mut self.app.config.editor_font_size)
                        .speed(0.15)
                        .range(8.0..=32.0)
                )
            );
            ui.end_row();
        });

        if responses.iter().any(|r| r.changed()) {
            self.app.config.save().unwrap_or_else(|e| {
                log::error!("Failed to save config: {e}");
            });
        }

        ui.label("")
    }
}