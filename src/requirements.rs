use serde::Deserialize;
use crate::{items::*, Node, NodeId, ObstacleId};

#[derive(Deserialize, Debug)]
pub enum Requirement {
    #[serde(rename = "or")]
    LogicalOr(Vec<Requirement>),
    #[serde(rename = "not")]
    LogicalNot(Vec<Requirement>),
    #[serde(rename = "and")]
    LogicalAnd(Vec<Requirement>),
    #[serde(untagged)]
    Simple(Check),
}

#[derive(Deserialize, Debug)]
pub enum Check {
    #[serde(untagged)]
    Strategy(Logic),
    #[serde(untagged)]
    Equipment(ItemName),
    #[serde(untagged)]
    Tech(String),
    #[serde(untagged)]
    Flag(GameFlag),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Logic {
    Ammo(AmmoAmount),
    AmmoDrain(AmmoAmount),
    Refill(Vec<Resource>),
    PartialRefill { resource: Option<Resource>, limit: Option<u8> },
    #[serde(rename = "enemyKill")]
    KillEnemies(EnemiesToKill),
    AcidFrames(u8),
    AcidFramesNoGravity(u8),
    DraygonElectricityFrames(u8),
    EnemyDamage { 
        enemy: String, 
        #[serde(rename = "type")]
        enemy_type: String, 
        hits: u8 
    },
    HeatFrames(u8),
    HeatFramesWithEnergyDrops { frames: Option<u8>, drops: Option<Vec<EnemyDrops>> },
    HeatFramesWithoutGravity(u8),
    HibashiHits(u8),
    LavaFrames(u8),
    LavaFramesWithoutGravity(u8),
    SamusEaterFrames(u8),
    MetroidFrames(u8),
    EnergyAtMost(u16),
    AutoReserveTrigger { min_reserve_energy: Option<u8>, max_reserve_energy: Option<u8> },
    SpikeHits(u8),
    ThornHits(u8),
    DoorUnlockedAtNode(NodeId),
    ObstaclesCleared(Vec<ObstacleId>),
    ObstaclesNotCleared(Vec<ObstacleId>),
    ResourceCapacity(Vec<ResourceAmount>),
    ResourceAvailable(Vec<ResourceAmount>),
    ResourceMissingAtMost(Vec<ResourceAmount>),
    CanShineCharge(SpeedConditions),
    Shinespark { frames: u8, excess_frames: Option<u8> },
    ResetRoom { nodes: Vec<NodeId>, nodes_to_avoid: Option<Vec<NodeId>>, must_stay_put: Option<bool> },
    ItemNotCollectedAtNode(NodeId),
    GainFlashSuit {},
    UseFlashSuit {},
    NoFlashSuit {},
    Tech(String),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpeedConditions {
    used_tiles:             u8,
    open_end:               u8,
    gentle_up_tiles:        Option<u8>,
    gentle_down_tiles:      Option<u8>,
    steep_up_tiles:         Option<u8>,
    steep_down_tiles:       Option<u8>,
    starting_down_tiles:    Option<u8>,
}

#[derive(Deserialize, Debug)]
pub enum AmmoResource {
    Missile,
    Super,
    PowerBomb,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AmmoAmount {
    #[serde(rename = "type")]
    ammo_type: AmmoResource,
    count: u8,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceAmount {
    #[serde(rename = "type")]
    resource_type: Resource,
    count: u8,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnemiesToKill {
    enemies:            Vec<Vec<String>>,
    explicit_weapons:   Option<Vec<String>>,
    excluded_weapons:   Option<Vec<String>>,
    farmable_ammo:      Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnemyDrops {
    enemy: String,
    count: u8,
}
