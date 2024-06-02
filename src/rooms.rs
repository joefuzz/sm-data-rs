use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
}

#[derive(Deserialize, Debug)]
pub enum Area {
    Ceres,
    Brinstar,
    Crateria,
    Norfair,
    LowerNorfair,
    Maridia,
    Tourain,
    WreckedShip,
}

#[derive(Deserialize, Debug)]
pub struct SubArea(Area);

#[derive(Deserialize, Debug)]
pub struct SubSubArea(SubArea);

#[derive(Deserialize, Debug)]
pub enum Note {
    SingleLine(String),
    MultiLine(Vec<String>),
}

#[derive(Deserialize, Debug)]
pub enum DevNote {
    SingleLine(String),
    MultiLine(Vec<String>),
}

#[derive(Deserialize, Debug)]
pub struct RoomMemoryAddress(String);