mod vehicles;
mod carcols;
mod ui;
use std::{fs, io, env};
use std::path::{Path, PathBuf};
use std::fs::{File};
use serde_derive::Serialize;
use std::sync::{Mutex};
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref MODEL_DATA: Mutex<Vec<Model>> = Mutex::new(Vec::new());
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    model_name: Option<String>,
    game_name: Option<String>
}

impl Model {
    pub fn new() -> Self {
        Self {
            model_name: None,
            game_name: None
        }
    }
    fn clone(&self) -> Model {
        Model {
            model_name: self.model_name.clone(),
            game_name: self.game_name.clone()
        }
    }
    fn clear(&mut self) {
        self.model_name = None;
        self.game_name = None;
    }
}

fn should_walk_dir(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| {
            if entry.path().is_dir() {
                !s.contains("node_modules") && !s.starts_with(".")
            } else {
                s.ends_with(".meta") || s.ends_with(".xml")
            }
        })
        .unwrap_or(false)
}

#[allow(unused_must_use)]
pub fn handle_files(path: PathBuf) {
    let start = Instant::now();

    WalkDir::new(path.as_path())
        .into_iter()
        .filter_entry(|e| should_walk_dir(e))
        .filter_map(|v| v.ok())
        .for_each(|dir| handle_file(dir));

    println!("Finished executing, {:.2?} time elapsed", start.elapsed());

    let val = serde_json::to_string::<Vec<Model>>(crate::MODEL_DATA.lock().unwrap().as_ref()).unwrap();

    File::create("data.json");

    fs::write("data.json", val).unwrap();
}

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
        println!("arg: {}", arg);
        if arg == "-path" {
            is_headless = true
        } else if is_headless && arg != "exporter.exe"{
            let path = Path::new(&arg);
            handle_files(PathBuf::from(path));
            return;
        }
    }

    let app = ui::CarExporterUi::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

fn handle_file(dir: DirEntry) {
    let entry_name = dir.file_name().to_str().unwrap();
    // let entry_name = dir.file_name().into_string().unwrap();
    let path = &dir.path().to_path_buf();
    // We don't need to send the entire direntry
    if entry_name.contains("vehicles.meta") {
        vehicles::handle_vehicles(path);
    } else if entry_name.contains("carcols.meta") {
        carcols::handle_carcols(path);
    }
}