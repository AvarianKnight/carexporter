use std::fs::{DirEntry, File};
use std::io::{BufReader};
use xml::EventReader;
use xml::reader::XmlEvent;
use crate::{Model, MODEL_DATA};

pub(crate) fn handle_carcols(dir: &DirEntry) {
    let file = File::open(dir.path()).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    let mut data: String = String::new();
    let mut model_name = String::new();
    let mut game_name = String::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                data = name.to_string();
            }
            Ok(XmlEvent::Characters(chars)) => {
                if data == "wheelName" {
                    data = String::new();
                    model_name = chars;
                } else if data == "modShopLabel" {
                    game_name = chars;
                }
            }
            Err(e) => {
                println!("Error: {} {}", dir.path().display(), e);
            }
            _ => {}
        }
        let str_new = String::new();
        if model_name != str_new && game_name != str_new {
            MODEL_DATA.lock().unwrap().push(Model{
                model_name: model_name.clone(),
                game_name: game_name.clone()
            });
            model_name.clear();
            game_name.clear();
        }
    }
}
