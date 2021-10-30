use eframe::{egui, epi};
use std::{env};
use native_dialog::{FileDialog};
use eframe::egui::Vec2;

pub struct CarExporterUi {
    first_frame: bool,
    finished: bool,
    processing: bool,
    was_canceled: bool,
}

impl Default for CarExporterUi {
    fn default() -> Self {
        Self {
            first_frame: true,
            finished: false,
            processing: false,
            was_canceled: false
        }
    }
}

impl epi::App for CarExporterUi {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        if self.first_frame {
            frame.set_window_size(Vec2::new(800.0, 225.0));
            self.first_frame = false;
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            let space = 5.0;
            ui.heading("Car Metadata Exporter");
            ui.add_space(space);
            ui.heading("This is used for exporting a cars metadata like model name and game name so they won't be marked as NULL in-game.");
            ui.add_space(space);
            ui.heading("Click the file below and click the resource that has your vehicles, or select resources to go through each of them");
            ui.add_space(space);
            ui.heading("After clicking the directory the UI will freeze for a while, this is normal and expected, how long it freezes depends on the amount of vehicles you have");
            ui.add_space(space);

            if self.processing {
                ui.add(egui::Label::new("Processing..."));

            }

            // TODO: Non blocking
            if ui.button("Open File").clicked() && !self.processing {
                self.was_canceled = false;
                self.finished = false;
                let path = FileDialog::new()
                    .set_location(env::current_dir().unwrap().as_path())
                    .show_open_single_dir()
                    .unwrap();

                let path = match path {
                    Some(actual_path) => actual_path,
                    None => {
                        self.was_canceled = true;
                        return
                    }
                };

                crate::handle_files(path);

                self.finished = true;

            }

            if self.finished {
                ui.add(egui::Label::new("Successfully exported"));
            }

            if self.was_canceled {
                ui.add(egui::Label::new("User canceled"));
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Source code ");
                    ui.hyperlink_to("carexporter", "https://github.com/AvarianKnight/carexporter");
                    ui.label(" | ");
                    ui.hyperlink("https://github.com/AvarianKnight/carexporter");
                    egui::warn_if_debug_build(ui);
                });
            });
        });
    }

    /// Called once before the first frame.
    /// Hint: No its not
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn name(&self) -> &str {
        "Car Metadata Exporter"
    }
}