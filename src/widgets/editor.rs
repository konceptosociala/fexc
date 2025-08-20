use std::f32;

use egui_extras::syntax_highlighting::{highlight, CodeTheme};

use crate::app::Fexc;

pub struct CodeEditor<'app> {
    app: &'app mut Fexc,
}

impl<'app> CodeEditor<'app> {
    pub const DARK_BG_COLOR: egui::Color32 = egui::Color32::from_rgb(24, 25, 38);
    pub const LIGHT_BG_COLOR: egui::Color32 = egui::Color32::from_rgb(220, 224, 232);

    pub fn new(app: &'app mut Fexc) -> Self {
        Self { app }
    }
}

impl egui::Widget for CodeEditor<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let rows = ui.max_rect().height() / self.app.config.editor_font_size as f32;
        let language = "hs";
        let code = &mut self.app.code;

        let theme = CodeTheme::from_memory(ui.ctx(), ui.style());

        let mut layouter = |ui: &egui::Ui, buf: &dyn egui::TextBuffer, _: f32| {
            let mut layout_job = highlight(
                ui.ctx(),
                ui.style(),
                &theme,
                buf.as_str(),
                language,
            );
            layout_job.wrap.max_width = f32::INFINITY;
            ui.fonts(|f| {
                f.layout_job(layout_job)
            })
        };

        egui::Frame::new()
            .fill(match self.app.config.theme {
                egui::Theme::Light => Self::LIGHT_BG_COLOR,
                egui::Theme::Dark => Self::DARK_BG_COLOR,
            })
            .stroke(egui::Stroke::NONE)
            .show(ui, |ui| 
        {
            egui::ScrollArea::both()
                .auto_shrink([true; 2])
                .show(ui, |ui| 
            {
                ui.add(
                    egui::TextEdit::multiline(code)
                        .margin(egui::vec2(10.0, 10.0))
                        .frame(false)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_rows(rows as usize)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter),
                );
            });
        });

        ui.label("")
    }
}