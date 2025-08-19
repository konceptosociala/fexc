use std::sync::Arc;

use crate::{
    config::Config, i18n::I18n, plugin::Plugin, window_frame
};

pub struct Fexc {
    config: Config,
    i18n: I18n,
    _plugins: Vec<Box<dyn Plugin>>,
}

impl Fexc {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        set_themes(cc);
        add_phosphor_icons(cc);
        
        let config = Config::load().unwrap_or_else(|e| {
            log::error!("{e}");

            Config::default()
        });

        Fexc {
            config,
            i18n: I18n::new(),
            _plugins: Vec::new(),
        }
    }

    pub fn i18n(&self, key: &str) -> &str {
        &self.i18n[(&self.config.language, key)]
    }
}

impl eframe::App for Fexc {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_theme(self.config.theme);

        window_frame::WindowFrame::new("~/").show(ctx, |ui| {   
            egui::SidePanel::left("files").show_inside(ui, |sidebar_ui| {
                sidebar_ui.label("Sidebar content here");
            });

            egui::TopBottomPanel::bottom("terminal").resizable(true).show_inside(ui, |bottom_ui| {
                bottom_ui.label("Terminal content here");
            });

            ui.heading(self.i18n("debug"));
            ui.label("This is a label.");
            ui.separator();
            if ui.button("Click me!").clicked() {
                ui.label("Button clicked!");
            }
            ui.horizontal(|ui| {
                ui.label("Horizontal layout:");
                ui.text_edit_singleline(&mut String::from("Editable text"));
            });
            ui.collapsing("Collapsible section", |ui| {
                ui.label("Inside the collapsible section.");
            });
            ui.checkbox(&mut true, "Sample checkbox");
            ui.radio_value(&mut 1, 1, "Radio 1");
            ui.radio_value(&mut 1, 2, "Radio 2");
            ui.add(egui::Slider::new(&mut 42.0, 0.0..=100.0).text("Slider"));
            ui.text_edit_multiline(&mut String::from("Multiline\nText"));
            ui.hyperlink("https://github.com/");
        });
    }

    fn save(&mut self, _: &mut dyn eframe::Storage) {
        self.config.save().unwrap_or_else(|e| {
            log::error!("{e}");
        });
    }
}

fn add_phosphor_icons(cc: &eframe::CreationContext<'_>) {
    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    cc.egui_ctx.set_fonts(fonts);
}

fn set_themes(cc: &eframe::CreationContext<'_>) {
    cc.egui_ctx.options_mut(|opts| {
        opts.light_style = Arc::new({
            let mut style = (*opts.light_style).clone();
            style.interaction.selectable_labels = false;
            style.text_styles = style.text_styles.iter().map(|(k, v)| {
                let mut new_v = v.clone();
                new_v.size *= 1.5;

                (k.clone(), new_v)
            }).collect();
            catppuccin_egui::set_style_theme(&mut style, catppuccin_egui::LATTE);
            style
        });

        opts.dark_style = Arc::new({
            let mut style = (*opts.dark_style).clone();
            style.interaction.selectable_labels = false;
            style.text_styles = style.text_styles.iter().map(|(k, v)| {
                let mut new_v = v.clone();
                new_v.size *= 1.5;

                (k.clone(), new_v)
            }).collect();
            catppuccin_egui::set_style_theme(&mut style, catppuccin_egui::MACCHIATO);
            style
        });
    });
}
