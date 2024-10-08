// // // // // // // // // // // // // // // // // // // // // // // //
//
// fonts_compare, Compare fonts installed on your system
// Copyright (C) 2024 Henri GASC
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
// // // // // // // // // // // // // // // // // // // // // // // //

use eframe::egui::{self, Widget};
use font_kit::{family_handle::FamilyHandle, source::SystemSource};

#[derive(Clone)]
pub struct Option {
    pub name: String,
    pub selected: String,
    pub exact_font: String,
    pub regular: String,
}

impl Default for Option {
    fn default() -> Self {
        return Self {
            name: "box".to_string(),
            selected: "Default".to_string(),
            exact_font: "".to_string(),
            regular: "".to_string(),
        };
    }
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
        if (self.selected != "Default") && self.is_name_in_fonts(ui, &self.selected) {
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

            if self.selected != "Default" {
                if !self.is_name_in_fonts(ui, &self.selected) {
                    self.add_selected_font(ui, fonts);
                }
                self.draw_variants(ui, fonts);
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

    fn get_family(&self) -> FamilyHandle {
        return SystemSource::new()
            .select_family_by_name(&self.selected)
            .unwrap();
    }

    fn add_selected_font(&mut self, ui: &egui::Ui, fonts: &mut egui::FontDefinitions) {
        let binding = self.get_family();
        let family = binding.fonts();

        // Get first value (to be sure the font is available)
        let mut new_font = family
            .first()
            .unwrap()
            .load()
            .unwrap()
            .postscript_name()
            .unwrap();

        for handle in family {
            let font = handle.load().unwrap();
            let poss_font = font.postscript_name().unwrap_or("Default".to_string());
            // By default, search for the regular variant
            if poss_font.ends_with("-Regular") {
                new_font = poss_font.clone();
            } else if poss_font.find('-').is_none() {
                new_font = poss_font.clone();
            }

            // Store the font
            let data = font.copy_font_data().unwrap().to_vec();
            fonts
                .font_data
                .insert(poss_font.clone(), egui::FontData::from_owned(data));
        }
        self.link_font(fonts, new_font.clone());

        ui.ctx().set_fonts(fonts.clone());
        self.exact_font = new_font.clone();
        self.regular = new_font.clone();
    }

    fn link_font(&mut self, fonts: &mut egui::FontDefinitions, new_font: String) {
        // Link the font to the family
        let font = fonts
            .families
            .entry(egui::FontFamily::Name(self.selected.clone().into()))
            .or_default();
        // Remove the other fonts, keep only the last
        font.clear();
        font.push(new_font);
    }

    fn variance(&self, font_name: &str) -> String {
        if let Some(n) = font_name.find('-') {
            return font_name[n + 1..].to_string();
        }
        return "".to_string();
    }

    fn draw_button(
        &mut self,
        ui: &mut egui::Ui,
        text: egui::RichText,
        name: String,
        fonts: &mut egui::FontDefinitions,
        changed: &mut bool,
    ) {
        if ui.button(text).clicked() {
            self.exact_font = name.clone();
            self.link_font(fonts, name);
            *changed = true;
        }
    }

    fn draw_variants(&mut self, ui: &mut egui::Ui, fonts: &mut egui::FontDefinitions) {
        let color_selected = egui::Color32::GOLD;
        let num_columns = 3;
        let binding = self.get_family();
        let family = binding.fonts();
        let mut changed = false;
        let mut is_regular = true;
        let mut has_regular = false;

        ui.columns(num_columns, |cols| {
            let mut i = 0;
            for handle in family {
                let font = handle.load().unwrap();
                let name = font.postscript_name().unwrap_or(self.exact_font.clone());
                let var = self.variance(&name);
                if (var == "") || (name == self.selected) || name.ends_with("-Regular") {
                    has_regular = true;
                    continue;
                }
                // Change color if selected
                let mut text = egui::RichText::new(var);
                if name == self.exact_font {
                    is_regular = false;
                    text = text.color(color_selected);
                }

                self.draw_button(&mut cols[i % num_columns], text, name, fonts, &mut changed);
                i += 1;
            }

            // Add regular option if we add other options
            if (i != 0) && has_regular {
                // Change color if selected
                let mut text = egui::RichText::new("Regular");
                if is_regular {
                    text = text.color(color_selected);
                }

                self.draw_button(
                    &mut cols[i % num_columns],
                    text,
                    self.regular.clone(),
                    fonts,
                    &mut changed,
                );
            }
        });

        // Apply change in fonts, only if they changed (button was clicked)
        if changed {
            ui.ctx().set_fonts(fonts.clone());
        }
    }
}
