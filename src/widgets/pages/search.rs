use crate::app::Fexc;

pub struct SearchPage<'app> {
    app: &'app mut Fexc,
}

impl<'app> SearchPage<'app> {
    pub fn new(app: &'app mut Fexc) -> Self {
        SearchPage { app }
    }
}

impl egui::Widget for SearchPage<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.style_mut().spacing.item_spacing = egui::vec2(10.0, 10.0);

        ui.add(egui::Label::new(
            egui::RichText::new(self.app.i18n("search").to_uppercase())
                .size(28.0)
                .strong()
                .family(egui::FontFamily::Monospace)
        ));            

        ui.label("")
    }
}