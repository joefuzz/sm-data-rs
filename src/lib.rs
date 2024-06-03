#![allow(unused)]

pub mod items;
pub mod rooms;
pub mod notes;

use std::{fs::File, path::Path};
use std::io::prelude::*;
use items::*;
use rooms::*;
use serde_json::Error;

pub fn load_items() -> Result<Items, Error> {
    let path = Path::new("data/items.json");
    let mut text = String::new();

    load_json_text(path, &mut text);

    let items: Items = match serde_json::from_str(&text) {
        Err(why) => panic!("couldn't parse items.json: {}", why),
        Ok(items) => items,
    };

    return Ok(items);
}

pub fn load_room() -> Result<Room, Error> {
    let path = Path::new("data/region/brinstar/blue/Morph Ball Room.json");
    let mut text = String::new();

    load_json_text(path, &mut text);

    let room: Room = match serde_json::from_str(&text) {
        Err(why) => panic!("couldn't parse json: {}", why),
        Ok(items) => items,
    };

    return Ok(room);
}

fn load_json_text<'buf>(path: &Path, text: &mut String) {
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open items.json: {}", why),
        Ok(file) => file,
    };

    match file.read_to_string(text) {
        Err(why) => panic!("couldn't read items.json: {}", why),
        Ok(_) => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_deserializes_items() {
        let config = load_items();
        assert!(config.is_ok());
    }
    
    #[test]
    fn it_deserializes_room() {
        let rooms = load_room();
        assert!(rooms.is_ok());
    }
}
