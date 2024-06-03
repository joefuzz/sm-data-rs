use std::default;
use serde::Deserialize;
use crate::{notes::{DevNote, Note}, requirements::Requirement, GameFlag};
use crate::requirements::*;

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
    links: Vec<Link>,
    strats: Vec<Strat>,
    obstacles: Obstacle,
    enemies: Vec<Enemy>,
    reusable_roomwide_notable: Vec<ReusableRoomwideStrat>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ReusableRoomwideStrat {
    name: StratName,
    note: Note,
    dev_note: DevNote,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Enemy {
    id: EnemyId,
    group_name: String,
    enemy_name: String,
    quantity: u8,
    home_nodes: Vec<NodeId>,
    between_nodes: Vec<NodeId>,
    spawn: Requirement,
    stop_spawn: Requirement,
    drop_requires: Requirement,
    farm_cycles: Vec<FarmCycle>,
    note: Note,
    dev_note: DevNote,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct FarmCycle {
    name: String,
    cycle_frames: u8,
    requires: Requirement,
    note: Note,
    dev_note: DevNote,
}

#[derive(Deserialize, Debug, Default)]
pub struct EnemyId(String);

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Link {
    from: NodeId,
    to: Option<Vec<LinkEnd>>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct LinkEnd {
    id: NodeId,
    note: Note,
    dev_note: DevNote,
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
    id: NodeId,
    name: String,
    node_type: NodeType,
    node_sub_type: NodeSubType,
    node_address: NodeMemoryAddress,
    door_orientation: DoorOrientation,
    door_environments: Vec<DoorEnvironment>,
    use_implicit_door_unlocks: bool,
    interaction_requires: Requirement,
    spawn_at: u8,
    locks: Vec<Lock>,
    twin_door_addresses: Option<Vec<TwinDoorAddress>>,
    utility: Option<Utility>,
    viewable_nodes: Vec<ViewableNode>,
    yields: Vec<Yield>,
    note: Note,
    dev_note: DevNote,
}

#[derive(Deserialize, Debug, Default)]
pub struct NodeId(u8);

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
    entrance_nodes: Vec<NodeId>,
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

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Lock {
    lock_type: LockType,
    lock: Requirement,
    name: String,
    unlock_strats: Vec<Strat>,
    note: Note,
    dev_note: DevNote,
    yields: Vec<Yield>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum LockType {
    #[default]
    BossFight,
    ColoredDoor,
    Cutscene,
    EscapeFunnel,
    GameFlag,
    KillEnemies,
    Permanent,
    TriggeredEvent,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Strat {
    link: Vec<NodeId>,
    name: StratName,
    notable: bool,
    reusable_roomwide_notable: StratName,
    entrance_condition: EntranceCondition,
    requires: Requirement,
    exit_condition: ExitCondition,
    g_mode_regain_mobility: Option<GModeRegainMobility>,
    bypasses_door_shell: bool,
    unlocks_doors: Vec<UnlocksDoor>,
    clears_obstacles: Vec<ObstacleId>,
    resets_obstacles: Vec<ObstacleId>,
    collects_items: Vec<NodeId>,
    sets_flags: Vec<GameFlag>,
    flash_suit_checked: bool,
    failures: Vec<Failure>,
    note: Note,
    dev_note: DevNote,
}

#[derive(Deserialize, Debug, Default)]
pub struct StratName(String);
#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Obstacle {
    id: ObstacleId,
    name: String,
    obstacle_type: ObstacleType,
    note: Note,
    dev_note: DevNote,   
}

#[derive(Deserialize, Debug, Default)]
pub struct ObstacleId(String);

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum ObstacleType {
    #[default]
    Inanimate,
    Enemies,
    Abstract,
}

#[derive(Deserialize, Debug, Default)]
pub struct EntranceCondition;
#[derive(Deserialize, Debug, Default)]
pub struct ExitCondition;
#[derive(Deserialize, Debug, Default)]
pub struct GModeRegainMobility;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnlocksDoor {
    node_id: NodeId,
    types: Vec<UnlockDoorType>,
    requires: Requirement,
    use_implicit_requires: bool,
    note: Note,
    dev_note: DevNote,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum UnlockDoorType {
    #[default]
    Missiles,
    Super,
    Powerbomb,
    Gray,
    Ammo,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Failure {
    name: String,
    leads_to_node: NodeId,
    cost: Vec<Requirement>,
    softlock: bool,
    note: Note,
    dev_note: DevNote,
}

#[derive(Deserialize, Debug, Default)]
pub struct Yield(String);

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TwinDoorAddress {
    room_address: String,
    door_address: String,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum Utility {
    #[default]
    Save,
    Missile,
    Super,
    Powerbomb,
    Energy,
    Reserve,
    Map,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ViewableNode {
    id: NodeId,
    strats: Vec<Strat>
}
