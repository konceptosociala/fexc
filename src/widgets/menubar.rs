pub struct MenuBar;

impl MenuBar {
    pub fn new() -> Self {
        MenuBar
    }

    pub fn show(
        self, 
        ui: &mut egui::Ui, 
        add_contents: impl FnOnce(&mut egui::Ui),
    ) -> egui::InnerResponse<()> {
        egui::Frame::new()
            .fill(egui::Color32::TRANSPARENT)
            .inner_margin(egui::Margin {
                left: 8,
                right: 8,
                top: 8,
                bottom: 4,
            })
            .show(ui, |ui| 
        {
            egui::MenuBar::new()
                .ui(ui, add_contents);
        })
    }
}