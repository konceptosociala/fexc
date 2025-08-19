use crate::app::Fexc;

pub struct PluginsPage<'app> {
    app: &'app mut Fexc,
}

impl<'app> PluginsPage<'app> {
    pub fn new(app: &'app mut Fexc) -> Self {
        PluginsPage { app }
    }
}

impl egui::Widget for PluginsPage<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.style_mut().spacing.item_spacing = egui::vec2(10.0, 10.0);

        ui.add(egui::Label::new(
            egui::RichText::new(self.app.i18n("plugins").to_uppercase())
                .size(28.0)
                .strong()
                .family(egui::FontFamily::Monospace)
        ));            

        ui.label("")
    }
}