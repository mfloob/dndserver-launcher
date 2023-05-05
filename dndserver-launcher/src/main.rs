#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod utils;
mod memory;

const ICON: &[u8; 16384] = include_bytes!("../icon.bmp");

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(500f32, 250f32)),
        icon_data: Some(eframe::IconData {
            rgba: ICON.to_vec(),
            width: 64,
            height: 64,
        }),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "dndserver client launcher v1.1.0",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    )
}