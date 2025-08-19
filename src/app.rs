use std::sync::Arc;
use egui_phosphor::bold as ph;
use catppuccin_egui::Theme as CatppuccinTheme;
use crate::{
    config::Config, 
    fonts, 
    i18n::I18n, 
    plugin::Plugin,
    widgets::{
        toolbar::{ToolbarButton, ToolbarHeading}, window_frame::WindowFrame
    },
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Page {
    #[default]
    Project,
    Search,
    Plugins,
    Settings,
}

#[derive(Default)]
pub struct Fexc {
    config: Config,
    i18n: I18n,
    current_page: Page,
    _plugins: Vec<Box<dyn Plugin>>,
}

impl Fexc {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        set_themes(cc);
        set_fonts(cc);
        
        let config = Config::load().unwrap_or_else(|e| {
            log::error!("{e}");
            Config::default()
        });

        Fexc {
            config,
            ..Default::default()
        }
    }

    pub fn i18n(&self, key: &str) -> &str {
        &self.i18n[(&self.config.language, key)]
    }

    pub fn window_name(&self) -> String {
        self.config.current_project.as_ref()
            .and_then(|p| p.to_str().map(|s| s.to_owned()))
            .unwrap_or_else(|| "~/".to_owned())
    }

    pub fn _get_catppuccin_theme(&self) -> &'static CatppuccinTheme {
        match self.config.theme {
            egui::Theme::Light => &catppuccin_egui::LATTE,
            egui::Theme::Dark => &catppuccin_egui::MACCHIATO,
        }
    }
}

impl eframe::App for Fexc {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_theme(self.config.theme);

        WindowFrame::new(&self.window_name()).show(ctx, |ui| {   
            egui::SidePanel::left("sidebar")
                .min_width(256.0)
                .resizable(true)
                .show_inside(ui, |sidebar| 
            {
                sidebar.horizontal(|toolbar| {
                    toolbar.add(ToolbarHeading::new("λ"));                    

                    toolbar.separator();

                    toolbar.add(ToolbarButton::new(ph::FOLDER_OPEN, "Project", &mut self.current_page, Page::Project));
                    toolbar.add(ToolbarButton::new(ph::MAGNIFYING_GLASS, "Search", &mut self.current_page, Page::Search));
                    toolbar.add(ToolbarButton::new(ph::PUZZLE_PIECE, "Plugins", &mut self.current_page, Page::Plugins));
                    toolbar.add(ToolbarButton::new(ph::GEAR, "Settings", &mut self.current_page, Page::Settings));
                });

                sidebar.separator();
            });

            egui::TopBottomPanel::bottom("terminal")
                .min_height(128.0)
                .resizable(true)
                .show_inside(ui, |bottom_ui| 
            {
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

fn set_fonts(cc: &eframe::CreationContext<'_>) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "DSE".to_owned(),
        Arc::new(egui::FontData::from_static(fonts::DSE_TYPEWRITER)),
    );
    fonts.font_data.insert(
        "FSEX".to_owned(),
        Arc::new(egui::FontData::from_static(fonts::FIXEDSYS_EXCELSIOR)),
    );

    fonts.families.get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "FSEX".to_owned());

    fonts.families
        .entry(egui::FontFamily::Name("heading".into()))
        .or_insert_with(Vec::new)
        .insert(0, "DSE".to_owned());

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
