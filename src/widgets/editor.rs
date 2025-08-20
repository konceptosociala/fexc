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
            .inner_margin(egui::Margin {
                left: 0,
                right: 0,
                top: 0,
                bottom: 4,
            })
            .stroke(egui::Stroke::NONE)
            .show(ui, |ui| 
        {
            egui::ScrollArea::vertical()
                .auto_shrink([true; 2])
                .show(ui, |ui| 
            {
                ui.horizontal_top(|ui| {
                    numlines_show(ui, code, rows as usize, self.app.config.editor_font_size as f32);

                    egui::Frame::new()
                        .fill(match self.app.config.theme {
                            egui::Theme::Light => Self::LIGHT_BG_COLOR,
                            egui::Theme::Dark => Self::DARK_BG_COLOR,
                        })
                        .stroke(egui::Stroke::NONE)
                        .show(ui, |ui| 
                    {
                        egui::ScrollArea::horizontal()
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
                });
            });
        });

        ui.label("")
    }
}

fn numlines_show(ui: &mut egui::Ui, text: &str, rows: usize, font_size: f32) {
    let total = if text.ends_with('\n') || text.is_empty() {
        text.lines().count() + 1
    } else {
        text.lines().count()
    };

    let max_indent = total
        .to_string()
        .len();
    
    let mut counter = (1..=total)
        .map(|i| {
            let num = i;
            let label = num.to_string();
            format!(
                "{}{label}",
                " ".repeat(max_indent.saturating_sub(label.len()))
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let width = max_indent as f32 * font_size * 0.5;

    ui.add(
        egui::TextEdit::multiline(&mut counter)
            .margin(egui::Margin {
                left: 16,
                right: 0,
                top: 10,
                bottom: 10,
            })
            .id_source("numlines")
            .font(egui::TextStyle::Monospace)
            .interactive(false)
            .frame(false)
            .desired_rows(rows)
            .desired_width(width)
    );
}