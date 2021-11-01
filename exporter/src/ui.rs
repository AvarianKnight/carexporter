use eframe::{egui, epi};
use std::{env, thread};
use native_dialog::{FileDialog};
use eframe::egui::{Vec2, Color32};
use std::time::Duration;
use std::sync::{mpsc};
use crate::{handle_files};
use std::sync::mpsc::{Receiver};

pub enum DataState {
    NoData,
    Processing,
    Finished
}

pub struct DataTransfer {
    pub(crate) state: DataState,
    pub(crate) duration: Option<Duration>,
    pub(crate) file_count: Option<i32>,
    pub(crate) dir_count: Option<i32>
}

pub struct CarExporterUi {
    first_frame: bool,
    was_canceled: bool,
    processing: bool,
    finished: bool,
    file_count: i32,
    directory_count: i32,
    duration: Duration,
    receiver: Option<Receiver<DataTransfer>>
}

impl Default for CarExporterUi {
    fn default() -> Self {
        Self {
            first_frame: true,
            was_canceled: false,
            processing: false,
            finished: false,
            file_count: 0,
            directory_count: 0,
            duration: Default::default(),
            receiver: None
        }
    }
}

impl epi::App for CarExporterUi {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        if self.first_frame {
            frame.set_window_size(Vec2::new(800.0, 250.0));
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

            if ui.button("Open File").clicked() && !self.processing {
                self.processing = true;
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
                        self.processing = false;
                        return
                    }
                };

                let (tx, rx) = mpsc::channel();

                self.receiver = Some(rx);

                thread::spawn(move || {
                    handle_files(path, tx);
                });
            }

            if self.receiver.is_some() && self.processing {
                let data = match self.receiver.as_ref().unwrap().try_recv() {
                    Ok(transfer_data) => transfer_data,
                    Err(_) => DataTransfer {
                        state: DataState::NoData,
                        duration: None,
                        file_count: None,
                        dir_count: None
                    }
                };

                match data.state {
                    DataState::Processing => {
                        self.file_count = data.file_count.unwrap();
                        self.directory_count = data.dir_count.unwrap();
                    }
                    DataState::Finished => {
                        self.processing = false;
                        self.finished = true;
                        self.duration = data.duration.unwrap();
                    }
                    _ => {}
                };
            }

            if self.processing {
                ui.add_space(space);
                ui.add(egui::Label::new(format!("Exporting data, {} directories traversed with {} files exported so far.", self.directory_count, self.file_count)).text_color(Color32::GREEN));
            }

            if self.finished {
                ui.add_space(space);
                ui.add(egui::Label::new(format!("Successfully exported {} files in {:.2?}", self.file_count, self.duration)).text_color(Color32::GREEN));
            }

            if self.was_canceled {
                ui.add_space(space);
                ui.add(egui::Label::new("User canceled").text_color(Color32::RED));
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