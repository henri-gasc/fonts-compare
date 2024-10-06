use eframe::egui;

mod option;
use option::Option;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "Compare fonts",
        options,
        Box::new(|_| Ok(Box::new(MyApp::new()))),
    )
}

struct MyApp {
    text: String,
    left: Option,
    right: Option,
    zoom: f32,
    fonts: egui::FontDefinitions,
}

impl MyApp {
    fn new() -> Self {
        // let fc = Fontconfig::new().unwrap();
        Self {
            text: "The quick brown fox jumps over the lazy dog".to_owned(),
            left: Option {
                name: "Left box".to_owned(),
                selected: "Default".to_owned(),
                exact_font: "".to_string(),
            },
            right: Option {
                name: "Right box".to_owned(),
                selected: "Default".to_owned(),
                exact_font: "".to_string(),
            },
            zoom: 2.0,
            fonts: egui::FontDefinitions::default(),
        }
    }

    fn draw_zoom(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button(egui::RichText::new("+").monospace()).clicked() {
                self.zoom += 0.1;
            }
            if ui.button(egui::RichText::new("-").monospace()).clicked() {
                self.zoom -= 0.1;
            }
            self.zoom = self.zoom.clamp(0.1, 5.0);
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(self.zoom);
            self.draw_zoom(ui);

            ui.vertical_centered(|ui| {
                ui.label("Compare fonts").highlight();
                ui.add_space(20.0);
                if self.left.selected == self.right.selected {
                    ui.label(
                        egui::RichText::new("They are the same font").color(egui::Color32::RED),
                    );
                } else {
                    ui.label("");
                }
                ui.horizontal(|ui| {
                    ui.columns(2, |cols| {
                        self.left
                            .draw(&mut cols[0], &mut self.text, &mut self.fonts);
                        self.right
                            .draw(&mut cols[1], &mut self.text, &mut self.fonts);
                    });
                });
            });
        });
    }
}
