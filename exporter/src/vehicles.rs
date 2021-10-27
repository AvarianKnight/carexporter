use std::fs::{DirEntry, File};
use std::io::{BufReader};
use serde_derive::Deserialize;
use xml::EventReader;
use xml::reader::XmlEvent;
use crate::{Model, MODEL_DATA};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
struct CVehicleModelInfo__InitDataList {
    InitDatas: InitDatas
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
struct InitDatas {
    Item: Vec<Item>
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
struct Item {
    modelName: String,
    gameName: String
}

pub(crate) fn handle_vehicles(dir: &DirEntry) {
    let file = File::open(dir.path()).unwrap();
    let file_buf = BufReader::new(file);

    let parser = EventReader::new(file_buf);

    let mut data: String = String::new();
    let mut model_name = String::new();
    let mut game_name = String::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                data = name.to_string();
            }
            Ok(XmlEvent::Characters(chars)) => {
                if data == "modelName" {
                    data = String::new();
                    model_name = chars;
                } else if data == "gameName" {
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