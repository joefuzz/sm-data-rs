use serde::Deserialize;
use crate::items::*;

#[derive(Deserialize, Debug, Default)]
pub enum Requirement {
    #[default]
    None,
    Equipment(ItemName),
    Tech(String),
    Flag(GameFlag),
    Logical(Box<Logic>),
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Logic {
    and: Requirement,
    or: Requirement,
    not: Requirement,
    ammo: AmmoAmount,
    ammo_drain: AmmoAmount,
    refill: Resource,
    partial_refill: PartialRefill,
    enemy_kill: EnemyKill,
    acid_frames: u8,
    gravityless_acid_frames: u8,
    draygon_electricity_frames: u8,
    enemy_damage: EnemyDamage,
    heat_frames: u8,
    heat_frames_with_energy_drops: HeatFramesWithEnemyDrops,
    gravityless_heat_frames: u8,
    hibashi_hits: u8,
    lava_frames: u8,
    gravityless_lava_frames: u8,
    samus_eater_frames: u8,
    metroid_frames: u8,
    energy_at_most: u8,
    auto_reserve_trigger: AutoReserveTrigger,
    spike_hits: u8,
    thorn_hits: u8,
    door_unlocked_at_node: u8,
    obstacles_cleared: Vec<String>,
    obstacles_not_cleared: Vec<String>,
    resource_capacity: Vec<ResourceAmount>,
    resource_available: Vec<ResourceAmount>,
    resource_missing_at_most: Vec<ResourceAmount>,
    can_shinecharge: u8,
    get_blue_speed: u8,
    shinespark: u8,
    reset_room: ResetRoom,
    item_not_collected_at_node: u8,
    gain_flash_suit: GainFlashSuit,
    use_flash_suit: UseFlashSuit,
    no_flash_suit: NoFlashSuit,
    tech: String,
}

#[derive(Deserialize, Debug, Default)]
pub enum AmmoResource {
    #[default]
    Missile,
    Super,
    PowerBomb,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct AmmoAmount {
    #[serde(rename = "type")]
    ammo_type: AmmoResource,
    count: u8,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ResourceAmount {
    #[serde(rename = "type")]
    resource_type: Resource,
    count: u8,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct PartialRefill {
    #[serde(rename = "type")]
    resource_type: Resource,
    limit: u8,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct EnemyKill {
    enemies: Vec<String>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct EnemyDamage {
    enemy: String,
    #[serde(rename = "type")]
    enemy_type: String,
    hits: u8,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct HeatFramesWithEnemyDrops {
    frames: u8,
    drops: Vec<EnemyDrops>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct EnemyDrops {
    enemy: String,
    count: u8,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct AutoReserveTrigger {
    min_reserve_energy: u8,
    max_reserve_energy: u8,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ResetRoom {
    nodes: Vec<u8>,
    nodes_to_avoid: Vec<u8>,
    must_stay_put: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct GainFlashSuit;
#[derive(Deserialize, Debug, Default)]
pub struct UseFlashSuit;
#[derive(Deserialize, Debug, Default)]
pub struct NoFlashSuit;
