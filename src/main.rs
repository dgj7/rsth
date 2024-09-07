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
    txt_search_string: String,
    chk_search_filecontents: bool,
    chk_search_filenames: bool,
    chk_regex: bool,
    chk_case_sensitive: bool,

    txt_search_path: String,
    chk_subdirs: bool,

    chk_filtered_search: bool,
    chk_regex_filter: bool,
    txt_filter: String,

}

impl Default for TextHunter {
    fn default() -> Self {
        Self {
            txt_search_string: "".to_owned(),
            chk_search_filecontents: true,
            chk_search_filenames: true,
            chk_regex: false,
            chk_case_sensitive: false,

            txt_search_path: "".to_owned(),
            chk_subdirs: false,

            chk_filtered_search: false,
            chk_regex_filter: false,
            txt_filter: "".to_owned(),
        }
    }
}

impl eframe::App for TextHunter {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let name = ui.label("Search for:");
                ui.text_edit_singleline(&mut self.txt_search_string).labelled_by(name.id);
                ui.checkbox(&mut self.chk_search_filecontents, "contents?");
                ui.checkbox(&mut self.chk_search_filenames, "names?");
                ui.separator();
                ui.checkbox(&mut self.chk_regex, "regex?");
                ui.separator();
                ui.checkbox(&mut self.chk_case_sensitive, "case-sensitive?");
                ui.separator();

                if ui.add(egui::Button::new("Search!")).clicked() {
                    // todo
                }
            });

            ui.horizontal(|ui| {
                let name = ui.label("In Path:");
                ui.text_edit_singleline(&mut self.txt_search_path).labelled_by(name.id);// todo: this should be disabled
                ui.checkbox(&mut self.chk_subdirs, "subdirs?");
                ui.separator();

                if ui.add(egui::Button::new("Browse...")).clicked() {
                    // todo
                }
            });

            ui.horizontal(|ui| {
                ui.disable();
                ui.checkbox(&mut self.chk_filtered_search, "filtered?");
                ui.separator();
                ui.checkbox(&mut self.chk_regex_filter, "regex filter?");// todo: this should be disabled; enabled when chk_filtered_search is checked
                ui.text_edit_singleline(&mut self.txt_filter);// todo: this should be disabled; enabled when chk_filtered_search is checked
            });
        });
    }
}
