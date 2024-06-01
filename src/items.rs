use serde::Deserialize;
use serde_json::Deserializer;

#[derive(Deserialize, Debug)]
struct StartingResource {
    resource: Resource,
    #[serde(rename = "maxAmount")]
    max_amount: u16
}

#[derive(Deserialize, Debug)]
struct UpgradeItem {
    name: ItemName,
    data: String,
}

#[derive(Deserialize, Debug)]
struct ExpansionItem {
    name: ItemName,
    data: String,
    resource: Resource,
    #[serde(rename = "resourceAmount")]
    resource_amount: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum Resource {
    RegularEnergy,
    ReserveEnergy,
    Missile,
    Super,
    PowerBomb,
}

#[derive(Deserialize, Debug)]
enum ItemName {
    PowerBeam, 
    PowerSuit,
    Morph,
    Bombs,
    Charge,
    HiJump,
    Spazer,
    Varia,
    SpeedBooster,
    Ice,
    Grapple,
    Wave,
    XRayScope,
    Gravity,
    SpaceJump,
    SpringBall,
    Plasma,
    ScrewAttack,
    ETank,
    Super,
    Missile,
    ReserveTank,
    PowerBomb,
}

#[derive(Deserialize, Debug)]
#[allow(non_camel_case_types)]
enum GameFlag {
    f_AnimalsSaved,
    f_BeatSuperMetroid,
    f_DefeatedBombTorizo,
    f_DefeatedBotwoon,
    f_DefeatedCeresRidley,
    f_DefeatedCrocomire,
    f_DefeatedDraygon,
    f_DefeatedGoldenTorizo,
    f_DefeatedKraid,
    f_DefeatedMotherBrain,
    f_DefeatedPhantoon,
    f_DefeatedRidley,
    f_DefeatedSporeSpawn,
    f_KilledMetroidRoom1,
    f_KilledMetroidRoom2,
    f_KilledMetroidRoom3,
    f_KilledMetroidRoom4,
    f_KilledZebetites1,
    f_KilledZebetites2,
    f_KilledZebetites3,
    f_KilledZebetites4,
    f_MaridiaTubeBroken,
    f_ShaktoolDoneDigging,
    f_TourianOpen,
    f_UsedAcidChozoStatue,
    f_ZebesAwake,
    f_ZebesSetAblaze,
    f_MotherBrainGlassBroken,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Items {
    starting_room: String,
    starting_node: u16,
    starting_items: Vec<ItemName>,
    starting_flags: Vec<String>,
    starting_locks: Vec<String>,
    starting_resources: Vec<StartingResource>,
    implicit_items: Vec<ItemName>,
    upgrade_items: Vec<UpgradeItem>,
    expansion_items: Vec<ExpansionItem>,
    game_flags: Vec<GameFlag>,
}
