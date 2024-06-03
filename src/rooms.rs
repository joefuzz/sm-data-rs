use std::default;
use serde::Deserialize;
use crate::notes::{Note, DevNote};

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Room {
    id: u8,
    name: String,
    area: Area,
    subarea: SubArea,
    subsubarea: SubSubArea,
    playable: bool,
    note: Note,
    dev_note: DevNote,
    room_address: RoomMemoryAddress,
    nodes: Vec<Node>,
}

#[derive(Deserialize, Debug, Default)]
pub enum Area {
    #[default]
    Ceres,
    Brinstar,
    Crateria,
    Norfair,
    LowerNorfair,
    Maridia,
    Tourain,
    WreckedShip,
}

#[derive(Deserialize, Debug, Default)]
pub enum SubArea {
    #[default]
    Blue,
    Green,
    Pink,
    Red,
    Kraid,
    West,
    Central,
    East,
    Outer,
    Inner,
    Crocomire,
}

#[derive(Deserialize, Debug, Default)]
pub enum SubSubArea {
    #[default]
    Green, 
    Pink, 
    Yellow
}

#[derive(Deserialize, Debug, Default)]
pub struct RoomMemoryAddress(String);

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Node {
    id: u8,
    name: String,
    node_type: NodeType,
    node_sub_type: NodeSubType,
    node_address: NodeMemoryAddress,
    door_orientation: DoorOrientation,
    door_environments: Vec<DoorEnvironment>,
    use_implicit_door_unlocks: bool,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
 enum NodeType {
    #[default]
    Door,
    Entrance,
    Exit,
    Event,
    Item,
    Junction,
    Utility,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum NodeSubType {
    #[default]
    Boss,
    Chozo,
    Flag,
    Hidden,
    Visible,
    Blue,
    ClosedWall,
    Doorway,
    Elevator,
    Eye,
    Gray,
    Green,
    Passage,
    Red,
    Sandpit,
    Vertical,
    Yellow,
    Junction,
    #[serde(rename = "g-mode")]
    GMode,
    Save,
    Missile,
    Energy,
    Ship,
    Map,
}

#[derive(Deserialize, Debug, Default)]
pub struct NodeMemoryAddress(String);

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum DoorOrientation { 
    #[default]
    Left, 
    Right, 
    Up, 
    Down 
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct DoorEnvironment {
    physics: Physics,
    entrance_nodes: Vec<u8>,
    note: Note,
    dev_note: DevNote,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum Physics {
    #[default]
    Air,
    Water,
    Lava,
    Acid,
    Normal,
}