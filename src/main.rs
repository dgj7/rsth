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
                let ilbl_name = ui.label("Search for:");
                let itxt_search_str = ui.text_edit_singleline(&mut self.txt_search_string).labelled_by(ilbl_name.id);
                let ichk_search_fc = ui.checkbox(&mut self.chk_search_filecontents, "contents?");
                let ichk_search_fn = ui.checkbox(&mut self.chk_search_filenames, "names?");
                ui.separator();
                let ichk_regex = ui.checkbox(&mut self.chk_regex, "regex?");
                ui.separator();
                let ichk_case_sensitive = ui.checkbox(&mut self.chk_case_sensitive, "case-sensitive?");
                ui.separator();
                let ibtn_search = ui.add(egui::Button::new("Search!"));

                if ibtn_search.clicked() {
                    // todo
                }

                ichk_search_fc.on_hover_text("check if file contents should be searched");
                itxt_search_str.on_hover_text("the string to search for");
                ichk_search_fn.on_hover_text("check if file names should be searched");
                ichk_regex.on_hover_text("check if search string should be treated as a regular expression");
                ichk_case_sensitive.on_hover_text("check if search string character case should be matched");
                ibtn_search.on_hover_text("launch the search; all input fields will be cleared");
            });

            ui.horizontal(|ui| {
                let ilbl_name = ui.label("In Path:");
                //let itxt_path = ui.add_enabled(false, &mut self.txt_search_path);
                let itxt_path = ui.add_enabled(false, egui::TextEdit::singleline(&mut self.txt_search_path)).labelled_by(ilbl_name.id);
                //let itxt_path = ui.text_edit_singleline(&mut self.txt_search_path).labelled_by(ilbl_name.id);// todo: this should be disabled
                let ichk_subdirs = ui.checkbox(&mut self.chk_subdirs, "subdirs?");
                ui.separator();
                let ibtn_browse = ui.add(egui::Button::new("Borwse..."));

                if ibtn_browse.clicked() {
                    // todo
                }

                itxt_path.on_hover_text("path in which to search; update this path by clicking on the browse... button");
                ichk_subdirs.on_hover_text("check if subdirectories should also be searched");
                ibtn_browse.on_hover_text("select a path to search within");
            });

            ui.horizontal(|ui| {
                let ichk_filtered = ui.checkbox(&mut self.chk_filtered_search, "filtered?");
                ui.separator();
                let ichk_regex = ui.checkbox(&mut self.chk_regex_filter, "regex filter?");// todo: this should be disabled; enabled when chk_filtered_search is checked
                let itxt_filter = ui.text_edit_singleline(&mut self.txt_filter);// todo: this should be disabled; enabled when chk_filtered_search is checked

                ichk_filtered.on_hover_text("check if the search should be filtered");
                ichk_regex.on_hover_text("check if the filter should be interpreted as a regular expression");
                itxt_filter.on_hover_text("text to filter the search by");
            });
        });
    }
}
