#![allow(unused)]

pub mod items;
pub mod rooms;

use std::{fs::File, path::Path};
use std::io::prelude::*;
use items::*;
use rooms::*;
use serde_json::Error;

pub fn load_items() -> Result<Items, Error> {
    let path = Path::new("data/items.json");

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open items.json: {}", why),
        Ok(file) => file,
    };

    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Err(why) => panic!("couldn't read items.json: {}", why),
        Ok(_) => (),
    }

    let items: Items = match serde_json::from_str(&text) {
        Err(why) => panic!("couldn't parse items.json: {}", why),
        Ok(items) => items,
    };

    return Ok(items);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_items() {
        let config = load_items();
        assert!(config.is_ok());
    }
}
