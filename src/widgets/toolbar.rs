use crate::app::Page;

pub struct ToolbarButton<'a> {
    icon: egui::RichText,
    tooltip: &'a str,
    page: &'a mut Page,
    action_page: Page,
}

impl<'a> ToolbarButton<'a> {
    pub const BUTTON_SIZE: f32 = 48.0;
    pub const ICON_SIZE: f32 = 24.0;

    pub fn new(
        icon: impl Into<egui::RichText>, 
        tooltip: &'a str,
        page: &'a mut Page,
        action_page: Page,
    ) -> Self {
        ToolbarButton {
            icon: icon.into(),
            tooltip,
            page,
            action_page,
        }
    }
}

impl egui::Widget for ToolbarButton<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let response = ui.add_sized(
            egui::vec2(ToolbarButton::BUTTON_SIZE, ToolbarButton::BUTTON_SIZE),
            egui::Button::new(
                self.icon
                    .size(ToolbarButton::ICON_SIZE)
                    .strong()                    
            )
        )
        .on_hover_text(self.tooltip);

        if response.clicked() {
            *self.page = self.action_page;
        }

        response
    }
}

pub struct ToolbarHeading {
    text: egui::RichText,
}

impl ToolbarHeading {
    pub const HEADING_SIZE: f32 = 48.0;

    pub fn new(text: &str) -> Self {
        ToolbarHeading { text: egui::RichText::new(text) }
    }
}

impl egui::Widget for ToolbarHeading {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add(
            egui::Label::new(
                self.text.clone()
                    .size(ToolbarHeading::HEADING_SIZE)
                    .family(egui::FontFamily::Name("heading".into()))
                    .strong()
            )
        )
    }
}