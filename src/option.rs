use eframe::egui::{self, Widget};
use font_kit::source::SystemSource;

#[derive(Clone)]
pub struct Option {
    pub name: String,
    pub selected: String,
    pub exact_font: String,
}

impl Option {
    fn is_name_in_fonts(&self, ui: &egui::Ui, name: &str) -> bool {
        // Test load status of font. Use fallback if not found
        let mut is_in = false;
        ui.ctx().fonts(|fonts| {
            for f in fonts.families() {
                if f == egui::FontFamily::Name(name.into()) {
                    is_in = true;
                    break;
                }
            }
        });
        return is_in;
    }

    fn write(&self, ui: &mut egui::Ui, text: &mut String) {
        if self.selected != "Default" {
            if self.is_name_in_fonts(ui, &self.selected) {
                // Write text with font
                let multi = egui::TextEdit::multiline(text);
                multi
                    .font(egui::FontId {
                        size: 12.0,
                        // self.fonts.font_data.first_key_value().unwrap().1.tweak.scale,
                        family: egui::FontFamily::Name(self.selected.clone().into()),
                    })
                    .ui(ui);
                return;
            }
        }

        ui.text_edit_multiline(text);
    }

    pub fn draw(
        &mut self,
        ui: &mut egui::Ui,
        text: &mut String,
        fonts: &mut egui::FontDefinitions,
    ) {
        ui.vertical_centered(|ui| {
            // Need this, otherwise ComboBox is not centered
            ui.columns(3, |cols| {
                // NOTE: When name is too big, ComboBox move to the left
                // This have to be fixed
                self.draw_combobox(&mut cols[1]);
            });

            ui.add_space(5.0);

            self.write(ui, text);

            if (self.selected != "Default") && !self.is_name_in_fonts(ui, &self.selected) {
                self.add_selected_font(ui, fonts);
            }
        });
    }

    fn draw_combobox(&mut self, ui: &mut egui::Ui) {
        // NOTE: Search for drop down menu we can search
        egui::ComboBox::from_id_salt(&self.name)
            .selected_text(self.selected.clone())
            .show_ui(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    let source = SystemSource::new();
                    for font in source.all_families().unwrap() {
                        ui.selectable_value(&mut self.selected, font.clone(), font.clone());
                    }
                });
            });
    }

    fn add_selected_font(&mut self, ui: &egui::Ui, fonts: &mut egui::FontDefinitions) {
        let system_fonts = SystemSource::new()
            .select_family_by_name(&self.selected)
            .unwrap();

        let mut new_font = self.selected.clone();
        for f in system_fonts.fonts() {
            let font = f.load().unwrap();
            let poss_font = font.postscript_name().unwrap_or("Default".to_string());
            if poss_font.ends_with("-Regular") {
                new_font = poss_font.clone();
            }

            let data = font.copy_font_data().unwrap().to_vec();
            fonts
                .font_data
                .insert(poss_font.clone(), egui::FontData::from_owned(data));
        }

        fonts
            .families
            .entry(egui::FontFamily::Name(self.selected.clone().into()))
            .or_default()
            .push(new_font.clone());

        ui.ctx().set_fonts(fonts.clone());
        self.exact_font = new_font.clone();
    }
}
