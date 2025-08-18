use std::sync::Arc;

use crate::{config::Config, window_frame};

pub struct Fexc {
    config: Config,
}

impl Fexc {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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

        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);

        let config_dir = dirs::config_dir()
            .map(|path| path.join("fexc"))
            .unwrap_or_else(|| std::path::PathBuf::from("./fexc_config"));
        println!("Config folder: {}", config_dir.display());

        Fexc {
            config: Config {
                theme: egui::Theme::Light,
            },
        }
    }
}

impl eframe::App for Fexc {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_theme(self.config.theme);

        window_frame::WindowFrame::new("Sasi").show(ctx, |ui| {   
            egui::SidePanel::left("files").show_inside(ui, |sidebar_ui| {
                sidebar_ui.label("Sidebar content here");
            });

            egui::TopBottomPanel::bottom("terminal").resizable(true).show_inside(ui, |bottom_ui| {
                bottom_ui.label("Terminal content here");
            });

            ui.heading("Hello world!");
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
}
