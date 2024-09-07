#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
            ..Default::default()
    };

    eframe::run_native("TextHunter", options, Box::new(|_cc| {
        Ok(Box::<TextHunter>::default())
    }),
        )
}

struct TextHunter {
    //
}

impl Default for TextHunter {
    fn default() -> Self {
        Self {
            // set TextHunter fields here
        }
    }
}

impl eframe::App for TextHunter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TextHunter");
        });
    }
}
