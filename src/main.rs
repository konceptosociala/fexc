#![deny(clippy::disallowed_methods)]

mod app;
mod config;
mod fonts;
mod i18n;
mod plugin;
mod widgets;

use app::Fexc;

const TITLE: &str = concat!("Fexc v", env!("CARGO_PKG_VERSION"));

fn main() {
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::Info)
        .filter(Some("sctk_adwaita"), log::LevelFilter::Off)
        .init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 720.0])
            .with_min_inner_size([640.0, 360.0])
            .with_decorations(false)
            .with_transparent(true),

        ..Default::default()
    };

    eframe::run_native(TITLE, options, Box::new(|cc| Ok(Box::new(Fexc::new(cc)))))
        .expect("Failed to run the application");
}