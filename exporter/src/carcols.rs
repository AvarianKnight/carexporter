use std::fs::{File};
use std::io::{BufReader};
use xml::EventReader;
use xml::reader::XmlEvent;
use crate::{Model, MODEL_DATA};
use std::path::{PathBuf};

pub(crate) fn handle_carcols(path: &PathBuf) {
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let parser = EventReader::new(file);

    let mut data = String::new();
    let mut model = Model::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => { data = name.to_string() }
            Ok(XmlEvent::Characters(chars)) => {
                match data.as_str() {
                    "wheelName" => { model.model_name = Some(chars) }
                    "modShopLabel" => { model.game_name = Some(chars) }
                    _ => {}
                }
                data.clear();
            }
            Err(e) => {  println!("Error: {} {}", path.display(), e) }
            _ => {}
        }
        if model.model_name.is_some() && model.game_name.is_some() {
            MODEL_DATA.lock().unwrap().push(model.clone());
            model.clear();
        }
    }
}
