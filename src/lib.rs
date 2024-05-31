#![allow(unused)]

use std::{fs::File, path::Path};
use std::io::prelude::*;
use serde::Deserialize;
use serde_json::Result;

#[derive(Deserialize, Debug)]
struct StartingResource {
    resource: String,
    #[serde(rename = "maxAmount")]
    max_amount: u16
}

#[derive(Deserialize, Debug)]
struct UpgradeItem {
    name: String,
    data: String,
}

#[derive(Deserialize, Debug)]
struct ExpansionItem {
    name: String,
    data: String,
    resource: String,
    #[serde(rename = "resourceAmount")]
    resource_amount: u16,
}

#[derive(Deserialize, Debug)]
pub struct Items {
    #[serde(rename = "startingRoom")]
    starting_room: String,
    #[serde(rename = "startingNode")]
    starting_node: u16,
    #[serde(rename = "startingItems")]
    starting_items: Vec<String>,
    #[serde(rename = "startingFlags")]
    starting_flags: Vec<String>,
    #[serde(rename = "startingLocks")]
    starting_locks: Vec<String>,
    #[serde(rename = "startingResources")]
    starting_resources: Vec<StartingResource>,
    #[serde(rename = "implicitItems")]
    implicit_items: Vec<String>,
    #[serde(rename = "upgradeItems")]
    upgrade_items: Vec<UpgradeItem>,
    #[serde(rename = "expansionItems")]
    expansion_items: Vec<ExpansionItem>,
    #[serde(rename = "gameFlags")]
    game_flags: Vec<String>,
}

pub fn load_items() -> Result<Items> {
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
