mod vehicles;
mod carcols;
use std::io::{stdin, stdout, Read, Write};
use std::{fs, io};
use std::path::Path;
use std::fs::{DirEntry, File};
use serde_derive::{Serialize};
use std::time::Instant;
use std::sync::{Mutex};

#[macro_use]
extern crate lazy_static;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    model_name: String,
    game_name: String
}

lazy_static! {
    pub static ref MODEL_DATA: Mutex<Vec<Model>> = Mutex::new(Vec::new());
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[allow(unused_must_use)]
fn main() -> io::Result<()> {
    let start = Instant::now();
    let entries = fs::read_dir("../../")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for entry in entries.iter() {
        visit_dirs(entry, &handle_file);
    }

    println!("Finished executing, {:.2?} time elapsed", start.elapsed());

    let val = serde_json::to_string::<Vec<Model>>(MODEL_DATA.lock().unwrap().as_ref()).unwrap();

    File::create("data.json");

    fs::write("data.json", val);

    pause();
    Ok(())
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
    let entry_name = dir.path().into_os_string().into_string().unwrap();
    if entry_name.contains("vehicles.meta") {
        vehicles::handle_vehicles(dir);
    } else if entry_name.contains("carcols.meta") {
        carcols::handle_carcols(dir);
    }
}