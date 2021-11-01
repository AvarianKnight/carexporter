mod data_handler;
mod ui;
use crate::ui::{DataState, DataTransfer};
use jwalk::WalkDirGeneric;
use serde_derive::Serialize;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};
use std::{env, fs, thread};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref MODEL_DATA: Mutex<Vec<Model>> = Mutex::new(Vec::new());
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    model_name: Option<String>,
    game_name: Option<String>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            model_name: None,
            game_name: None,
        }
    }
    fn clone(&self) -> Model {
        Model {
            model_name: self.model_name.clone(),
            game_name: self.game_name.clone(),
        }
    }
    fn clear(&mut self) {
        self.model_name = None;
        self.game_name = None;
    }
}

#[allow(unused_must_use)]
pub fn handle_files(path: PathBuf, tx: Sender<DataTransfer>) {
    let start = Instant::now();
    let mut file_count = 0;
    let dir_count = Arc::new(Mutex::new(0));

    let thread_dir_count = Arc::clone(&dir_count);

    let walk_dir = WalkDirGeneric::<(usize, bool)>::new(path).process_read_dir(
        move |_depth, _path, _read_dir_state, children| {
            children.retain(|dir_entry_result| {
                dir_entry_result
                    .as_ref()
                    .map(|dir_entry| {
                        dir_entry
                            .file_name
                            .to_str()
                            .map(|s| {
                                if dir_entry.path().is_dir() {
                                    let mut num = thread_dir_count.lock().unwrap();
                                    *num += 1;
                                    !s.contains("node_modules") && !s.starts_with(".")
                                } else {
                                    s.ends_with(".meta") || s.ends_with(".xml")
                                }
                            })
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            });
        },
    );

    for entry in walk_dir {
        let entry = entry.unwrap();
        let entry_name = entry.file_name.to_str().unwrap();
        if entry_name.contains("vehicles") || entry_name.contains("carcols") {
            data_handler::handle_data(&entry.path());
            file_count += 1;
            if file_count % 25 == 0 {
                tx.send(DataTransfer {
                    state: DataState::Processing,
                    duration: None,
                    file_count: Some(file_count),
                    dir_count: Some(*dir_count.lock().unwrap()),
                });
            }
        }
    }

    tx.send(DataTransfer {
        state: DataState::Processing,
        duration: None,
        file_count: Some(file_count),
        dir_count: Some(*dir_count.lock().unwrap()),
    });

    let val =
        serde_json::to_string::<Vec<Model>>(crate::MODEL_DATA.lock().unwrap().as_ref()).unwrap();

    File::create("data.json");

    fs::write("data.json", val).unwrap();

    tx.send(DataTransfer {
        state: DataState::Finished,
        duration: Some(start.elapsed()),
        file_count: None,
        dir_count: None,
    });
}

#[allow(dead_code)]
fn jooat(string: String) -> u32 {
    let lower_str = string.to_lowercase();
    let char_iter = lower_str.chars();
    let mut hash: u32 = 0;

    for char in char_iter {
        hash = hash.overflowing_add(u32::from(char as u8)).0;
        hash = hash.overflowing_add(hash.overflowing_shl(10).0).0;
        hash ^= hash.overflowing_shr(6).0;
    }

    hash = hash.overflowing_add(hash.overflowing_shl(3).0).0;
    hash ^= hash.overflowing_shr(11).0;
    hash = hash.overflowing_add(hash.overflowing_shl(15).0).0;

    hash
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut is_headless: bool = false;
    for arg in args.into_iter() {
        if arg == "-path" {
            is_headless = true
        } else if is_headless && arg != "exporter.exe" {
            let path_orig = Path::new(&arg);

            let path = match path_orig.is_absolute() {
                true => path_orig.to_path_buf(),
                false => env::current_dir().unwrap().join(path_orig),
            };

            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                handle_files(path, tx);
            });

            let mut file_count = 0;
            let mut dir_count = 0;
            let mut duration: Duration = Default::default();

            loop {
                let data = match rx.try_recv() {
                    Ok(transfer_data) => transfer_data,
                    Err(_) => continue,
                };

                match data.state {
                    DataState::Processing => {
                        file_count = data.file_count.unwrap();
                        dir_count = data.dir_count.unwrap();
                    }
                    DataState::Finished => {
                        duration = data.duration.unwrap();
                        break;
                    }
                    _ => {}
                };
            }

            println!("Finished execution in {:.2?}, traveled {} directories and parsed {} xml/meta files", duration, dir_count, file_count);
            return;
        }
    }

    let app = ui::CarExporterUi::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
