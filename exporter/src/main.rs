mod vehicles;
mod carcols;
mod ui;
use std::{fs, io, env};
use std::path::{Path, PathBuf};
use std::fs::{DirEntry, File};
use serde_derive::Serialize;
use std::sync::{Mutex};
use std::time::Instant;

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

#[allow(unused_must_use)]
pub fn handle_files(path: PathBuf) {
    let start = Instant::now();
    // TODO: Proper error handling
    let entries = fs::read_dir(path.as_path()).unwrap()
        .map(|res| res.map(|e| {
            e.path()
        }))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();

    for entry in entries.iter() {
        crate::visit_dirs(entry, &crate::handle_file);
    }

    println!("Finished executing, {:.2?} time elapsed", start.elapsed());

    let val = serde_json::to_string::<Vec<Model>>(crate::MODEL_DATA.lock().unwrap().as_ref()).unwrap();

    File::create("data.json");

    fs::write("data.json", val).unwrap();
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

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn handle_file(dir: &DirEntry) {
    let entry_name = dir.file_name().into_string().unwrap();
    let path = &dir.path();
    // We don't need to send the entire direntry
    if entry_name.contains("vehicles.meta") {
        vehicles::handle_vehicles(path);
    } else if entry_name.contains("carcols.meta") {
        carcols::handle_carcols(path);
    }
}