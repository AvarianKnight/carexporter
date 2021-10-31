use std::fs::{File};
use std::io::{BufReader, Read};
use quick_xml::events::Event;
use quick_xml::Reader;
use crate::{Model, MODEL_DATA};
use std::path::{PathBuf};

pub(crate) fn handle_data(path: &PathBuf) {
    let file = File::open(path).unwrap();
    let mut file_string = String::new();
    BufReader::new(file).read_to_string(&mut file_string).unwrap();

    let mut data = String::new();
    let mut model = Model::new();
    let mut reader = Reader::from_str(&*file_string);
    reader.trim_text(true);

    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"wheelName" => data = String::from("wheelName"),
                    b"modShopLabel" => data = String::from("modShopLabel"),
                    b"modelName" => data = String::from("modelName"),
                    b"gameName" => data = String::from("gameName"),
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                let chars = e.unescape_and_decode(&reader).unwrap();
                match data.as_str() {
                    "wheelName" => { model.model_name = Some(chars) },
                    "modShopLabel" => { model.game_name = Some(chars) },
                    "modelName" => { model.model_name = Some(chars) },
                    "gameName" => { model.game_name = Some(chars) },
                    _ => {}
                }
                data.clear();
            },
            Ok(Event::Eof) => break,
            Err(e) => println!("Error in file {} {}: {:?}", path.display() ,reader.buffer_position(), e),
            _ => (),
        }

        if model.model_name.is_some() && model.game_name.is_some() {
            MODEL_DATA.lock().unwrap().push(model.clone());
            model.clear();
        }

        buf.clear();
    }
}
