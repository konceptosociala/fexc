use crate::app::Fexc;

pub struct ProjectPage<'app> {
    app: &'app mut Fexc,
}

impl<'app> ProjectPage<'app> {
    pub fn new(app: &'app mut Fexc) -> Self {
        ProjectPage { app }
    }
}

impl egui::Widget for ProjectPage<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.style_mut().spacing.item_spacing = egui::vec2(10.0, 10.0);

        if let Some(project) = &self.app.current_project {
            ui.add(egui::Label::new(
                egui::RichText::new(format!("{}", project.file_stem().unwrap().to_string_lossy()).to_uppercase())
                    .size(28.0)
                    .strong()
                    .family(egui::FontFamily::Monospace)
            ));
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 2.0 - 20.0); // Adjust for vertical centering
                ui.label(self.app.i18n("no_project_open"));
            });
        }

        ui.label("")
    }
}