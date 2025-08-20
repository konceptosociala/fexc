use std::{path::PathBuf, sync::Arc};
use egui_phosphor::bold as ph;
use catppuccin_egui::Theme as CatppuccinTheme;
use crate::{
    config::Config, 
    fonts, 
    i18n::I18n, 
    plugin::Plugin,
    widgets::{
        editor::CodeEditor, pages::{
            plugins::PluginsPage, project::ProjectPage, search::SearchPage, settings::SettingsPage
        }, toolbar::{ToolbarButton, ToolbarHeading}, window_frame::WindowFrame
    },
};

const HASKELL_DEMO: &str = 
r#"-- Custom map function
myMap :: (a -> b) -> [a] -> [b]
myMap _ [] = []
myMap f (x:xs) = f x : myMap f xs

-- Using myMap with an anonymous function
mappedList = myMap (\x -> x * 2) [1, 2, 3] -- mappedList will be [2, 4, 6]

-- Using foldl1 (fold left, using the first element as the initial accumulator)
sumList = foldl1 (+) [1, 2, 3, 4, 5] -- sumList will be 15
"#;

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
    pub config: Config,
    pub i18n: I18n,
    pub current_page: Page,
    pub open_files: Vec<PathBuf>,
    pub current_project: Option<PathBuf>,
    pub _plugins: Vec<Box<dyn Plugin>>,

    pub code: String,
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
            code: HASKELL_DEMO.to_owned(),
            ..Default::default()
        }
    }

    pub fn i18n(&self, key: &str) -> &str {
        &self.i18n[(&self.config.language, key)]
    }

    pub fn window_name(&self) -> String {
        self.current_project.as_ref()
            .and_then(|p| p.to_str().map(|s| s.to_owned()))
            .unwrap_or_else(|| self.i18n("empty_project").to_owned())
    }

    pub fn _get_catppuccin_theme(&self) -> &'static CatppuccinTheme {
        match self.config.theme {
            egui::Theme::Light => &catppuccin_egui::LATTE,
            egui::Theme::Dark => &catppuccin_egui::MACCHIATO,
        }
    }

    pub fn set_editor_font_size(&mut self, ctx: &egui::Context) {
        ctx.options_mut(|opts| {
            opts.light_style = Arc::new({
                let mut style = (*opts.light_style).clone();
                let monospace = style.text_styles.get_mut(&egui::TextStyle::Monospace).unwrap();
                monospace.size = self.config.editor_font_size as f32;
                style
            });

            opts.dark_style = Arc::new({
                let mut style = (*opts.dark_style).clone();
                let monospace = style.text_styles.get_mut(&egui::TextStyle::Monospace).unwrap();
                monospace.size = self.config.editor_font_size as f32;
                style
            });
        });
    }
}

impl eframe::App for Fexc {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_theme(self.config.theme);
        self.set_editor_font_size(ctx);

        WindowFrame::new(&self.window_name()).show(ctx, |ui| {   
            egui::MenuBar::new()
                .ui(ui, |ui| 
            {
                ui.menu_button(self.i18n("file"), |ui| {
                    if ui.button(self.i18n("quit")).clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button(self.i18n("file"), |ui| {
                    if ui.button(self.i18n("quit")).clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });

            egui::SidePanel::left("sidebar")
                .min_width(256.0)
                .resizable(true)
                .show_inside(ui, |sidebar| 
            {
                let project_label = self.i18n("project").to_owned();
                let search_label = self.i18n("search").to_owned();
                let plugins_label = self.i18n("plugins").to_owned();
                let settings_label = self.i18n("settings").to_owned();

                sidebar.horizontal(|toolbar| {
                    toolbar.add(ToolbarHeading::new("λ"));                    

                    toolbar.separator();

                    toolbar.add(ToolbarButton::new(ph::FOLDER_OPEN, &project_label, &mut self.current_page, Page::Project));
                    toolbar.add(ToolbarButton::new(ph::MAGNIFYING_GLASS, &search_label, &mut self.current_page, Page::Search));
                    toolbar.add(ToolbarButton::new(ph::PUZZLE_PIECE, &plugins_label, &mut self.current_page, Page::Plugins));
                    toolbar.add(ToolbarButton::new(ph::GEAR, &settings_label, &mut self.current_page, Page::Settings));
                });

                sidebar.separator();

                match self.current_page {
                    Page::Project => {
                        sidebar.add(ProjectPage::new(self));
                    }
                    Page::Search => {
                        sidebar.add(SearchPage::new(self));
                    }
                    Page::Plugins => {
                        sidebar.add(PluginsPage::new(self));
                    }
                    Page::Settings => {
                        sidebar.add(SettingsPage::new(self));
                    }
                };
            });

            egui::TopBottomPanel::bottom("terminal")
                .min_height(128.0)
                .resizable(true)
                .show_inside(ui, |bottom_ui| 
            {
                bottom_ui.label("<terminal>");
            });

            ui.add(CodeEditor::new(self));
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
                new_v.size *= 1.3;

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
                new_v.size *= 1.3;

                (k.clone(), new_v)
            }).collect();
            catppuccin_egui::set_style_theme(&mut style, catppuccin_egui::MACCHIATO);
            style
        });
    });
}
