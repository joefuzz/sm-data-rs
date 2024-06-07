use std::default;
use serde::Deserialize;
use serde_json::Map;
use crate::{notes::{DevNote, Note}, requirements::*, GameFlag};

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Room {
    pub id:                         u8,
    pub name:                       String,
    pub area:                       Area,
    pub subarea:                    SubArea,
    pub subsubarea:                 SubSubArea,
    pub playable:                   bool,
    pub nodes:                      Vec<Node>,
    pub links:                      Vec<Link>,
    pub strats:                     Vec<Strat>,
    pub room_address:               Option<RoomMemoryAddress>,
    pub obstacles:                  Option<Vec<Obstacle>>,
    pub enemies:                    Option<Vec<Enemy>>,
    pub reusable_roomwide_notable:  Option<Vec<ReusableRoomwideStrat>>,
    pub note:                       Option<Note>,
    pub dev_note:                   Option<DevNote>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ReusableRoomwideStrat {
    name:       StratName,
    note:       Note,
    dev_note:   Option<DevNote>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Enemy {
    id: EnemyId,
    group_name:     String,
    enemy_name:     String,
    quantity:       u8,
    home_nodes:     Option<Vec<NodeId>>,
    between_nodes:  Option<Vec<NodeId>>,
    spawn:          Option<Vec<Requirement>>,
    stop_spawn:     Option<Vec<Requirement>>,
    drop_requires:  Option<Vec<Requirement>>,
    farm_cycles:    Option<Vec<FarmCycle>>,
    note:           Option<Note>,
    dev_note:       Option<DevNote>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct FarmCycle {
    name:           String,
    cycle_frames:   u8,
    requires:       Option<Vec<Requirement>>,
    note:           Option<Note>,
    dev_note:       Option<DevNote>,
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
    id:                         NodeId,
    name:                       String,
    node_type:                  NodeType,
    node_sub_type:              NodeSubType,
    node_address:               Option<NodeMemoryAddress>,
    door_orientation:           Option<DoorOrientation>,
    door_environments:          Option<Vec<DoorEnvironment>>,
    use_implicit_door_unlocks:  Option<bool>,
    interaction_requires:       Option<Requirement>,
    spawn_at:                   Option<u8>,
    locks:                      Option<Vec<Lock>>,
    twin_door_addresses:        Option<Vec<TwinDoorAddress>>,
    utility:                    Option<Utility>,
    viewable_nodes:             Option<Vec<ViewableNode>>,
    yields:                     Option<Vec<Yield>>,
    note:                       Option<Note>,
    dev_note:                   Option<DevNote>,
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
    physics:            Physics,
    entrance_nodes:     Option<Vec<NodeId>>,
    note:               Option<Note>,
    dev_note:           Option<DevNote>,
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
    name:           String,
    lock_type:      LockType,
    unlock_strats:  Vec<Strat>,
    lock:           Option<Vec<Requirement>>,
    note:           Option<Note>,
    dev_note:       Option<DevNote>,
    yields:         Option<Vec<Yield>>,
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
    name:                       StratName,
    requires:                   Vec<Requirement>,
    link:                       Option<Vec<NodeId>>,
    notable:                    Option<bool>,
    reusable_roomwide_notable:  Option<StratName>,
    entrance_condition:         Option<EntranceCondition>,
    exit_condition:             Option<ExitCondition>,
    g_mode_regain_mobility:     Option<GModeRegainMobility>,
    bypasses_door_shell:        Option<bool>,
    unlocks_doors:              Option<Vec<UnlocksDoor>>,
    clears_obstacles:           Option<Vec<ObstacleId>>,
    resets_obstacles:           Option<Vec<ObstacleId>>,
    collects_items:             Option<Vec<NodeId>>,
    sets_flags:                 Option<Vec<GameFlag>>,
    flash_suit_checked:         Option<bool>,
    failures:                   Option<Vec<Failure>>,
    note:                       Option<Note>,
    dev_note:                   Option<DevNote>,
}

#[derive(Deserialize, Debug, Default)]
pub struct StratName(String);
#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Obstacle {
    id:             ObstacleId,
    name:           String,
    obstacle_type:  ObstacleType,
    note:           Option<Note>,
    dev_note:       Option<DevNote>,   
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
#[serde(rename_all = "camelCase")]
pub struct EntranceCondition {
    come_in_normally:                   Option<ComeInNormally>,
    come_in_running:                    Option<ComeInRunning>,
    come_in_jumping:                    Option<ComeInJumping>,
    come_in_space_jumping:              Option<ComeInSpaceJumping>,
    come_in_shinecharging:              Option<ComeInShinecharging>,
    come_in_getting_blue_speed:         Option<ComeInGettingBlueSpeed>,
    come_in_shinecharged:               Option<ComeInShinecharged>,
    come_in_shinecharged_jumping:       Option<ComeInShinechargedJumping>,
    come_in_with_spark:                 Option<TransitionWithSpark>,
    come_in_stutter_shinecharging:      Option<ComeInStutterShinecharging>,
    come_in_with_bomb_boost:            Option<ComeInWithBombBoost>,
    come_in_with_door_stuck_setup:      Option<ComeInWithDoorStuckSetup>,
    come_in_speedballing:               Option<ComeInSpeedballing>,
    come_in_with_temporary_blue:        Option<TransitionWithTemporaryBlue>,
    come_in_blue_spinning:              Option<ComeInBlueSpinning>,
    come_in_with_mockball:              Option<ComeInWithMockball>,
    come_in_with_spring_ball_bounce:    Option<ComeInWithSpringBallBounce>,
    come_in_with_stored_fall_speed:     Option<TransitionWithStoredFallSpeed>,
    come_in_with_r_mode:                Option<ComeInWithRMode>,
    come_in_with_g_mode:                Option<ComeInWithGMode>,
    come_in_with_wall_jump_below:       Option<ComeInWithWallJumpBelow>,
    come_in_with_space_jump_below:      Option<ComeInWithSpaceJumpBelow>,
    come_in_with_platform_below:        Option<ComeInWithPlatformBelow>,
    come_in_with_grapple_teleport:      Option<TransitionWithGrappleTeleport>,
    comes_through_toilet:               Option<YesNoAny>,
    dev_note:                           Option<DevNote>,
}

#[derive(Deserialize, Debug, Default)]
pub struct ComeInNormally;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInRunning {
    speed_booster:      WithSpeedBooster,
    min_tiles:          u8,
    max_tiles:          Option<u8>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum WithSpeedBooster {
    True,
    False,
    #[default]
    Any,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInJumping {
    speed_booster:      WithSpeedBooster,
    min_tiles:          u8,
    max_tiles:          Option<u8>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInSpaceJumping {
    speed_booster:      WithSpeedBooster,
    min_tiles:          u8,
    max_tiles:          Option<u8>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInShinecharging {
    length:             u8,
    open_end:           u8,
    gentle_up_tiles:    Option<u8>,
    gentle_down_tiles:  Option<u8>,
    steep_up_tiles:     Option<u8>,
    steep_down_tiles:   Option<u8>,
    min_tiles:          Option<u8>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInGettingBlueSpeed {
    length:             u8,
    open_end:           u8,
    gentle_up_tiles:    Option<u8>,
    gentle_down_tiles:  Option<u8>,
    steep_up_tiles:     Option<u8>,
    steep_down_tiles:   Option<u8>,
    min_tiles:          Option<u8>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInShinecharged {
    frames_required:    u8,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInShinechargedJumping {
    frames_required:    u8,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransitionWithSpark {
    position:   Option<DoorSparkPosition>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum DoorSparkPosition {
    #[default]
    Top,
    Bottom,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInStutterShinecharging {
    min_tiles:  u8,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInWithBombBoost;
#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInWithDoorStuckSetup;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInSpeedballing {
    runway:     Runway,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Runway {
    length:             u8,
    open_end:           u8,
    gentle_up_tiles:    Option<u8>,
    gentle_down_tiles:  Option<u8>,
    steep_up_tiles:     Option<u8>,
    steep_down_tiles:   Option<u8>,
    min_tiles:          Option<u8>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransitionWithTemporaryBlue {
    direction:  FacingThroughTransition,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum FacingThroughTransition {
    Left,
    Right,
    #[default]
    Any,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInBlueSpinning {
    unusable_tiles:     u8,
    min_tiles:          Option<u8>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInWithMockball {
    adjacent_min_tiles:             Option<u8>,
    remote_and_landing_min_tiles:   Option<Vec<u8>>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInWithSpringBallBounce {
    movement_type:                  SpringBallMovement,
    adjacent_min_tiles:             Option<u8>,
    remote_and_landing_min_tiles:   Option<Vec<u8>>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum SpringBallMovement {
    Controlled,
    Uncontrolled,
    #[default]
    Any,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransitionWithStoredFallSpeed {
    fall_speed_in_tiles:    u8,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInWithRMode;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInWithGMode {
    mode:       GModeType,
    morphed:    bool,
    mobility:   Option<GModeMobility>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum GModeType {
    Direct, 
    Indirect, 
    #[default]
    Any,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum GModeMobility {
    Mobile,
    Immobile,
    #[default]
    Any,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInWithWallJumpBelow {
    min_height:     Option<u8>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInWithSpaceJumpBelow;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComeInWithPlatformBelow {
    min_height:             Option<u8>,
    max_height:             Option<u8>,
    max_left_position:      Option<u8>,
    min_right_position:     Option<u8>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransitionWithGrappleTeleport {
    block_positions:    Vec<(u8, u8)>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum YesNoAny {
    #[default]
    No,
    Yes,
    Any,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ExitCondition {
    LeaveNormally {},
    LeaveWithRunway(Runway),
    LeaveShinecharged(SparkFramesRemaining),
    LeaveWithTemporaryBlue(FacingThroughTransition),
    LeaveWithSpark(TransitionWithSpark),
    LeaveSpinning { remote_runway:  Runway, blue: Option<YesNoAny>, },
    LeaveWithMockball { remote_runway: Runway, landing_runway: Runway, blue: Option<YesNoAny>, },
    LeaveWithSpringBallBounce { remote_runway: Runway, landing_runway: Runway, movement_type: SpringBallMovement, blue: Option<YesNoAny>, },
    LeaveSpaceJumping { remote_runway: Runway, blue: YesNoAny, },
    LeaveWithStoredFallSpeed { fall_speed_in_tiles: u8, },
    LeaveWithGModeSetup { knockback: Option<bool>, },
    LeaveWithGMode { morphed: bool, },
    LeaveWithDoorFrameBelow { height: u8, },
    LeaveWithPlatformBelow { height: u8, left_position: u8, right_position: u8, },
    LeaveWithGrappleTeleport { 
        #[serde(alias = "blockPositions")]
        block_positions: Vec<(u8, u8)>, 
    },
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaveNormally;

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaveRunway {
    length:                 u8,
    open_end:               u8,
    gentle_up_tiles:        Option<u8>,
    gentle_down_tiles:      Option<u8>,
    steep_up_tiles:         Option<u8>,
    steep_down_tiles:       Option<u8>,
    starting_down_tiles:    Option<u8>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub enum SparkFramesRemaining {
    #[default]
    Auto,
    #[serde(rename = "framesRemaining")]
    Frames(u8),
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaveSpinning {
    remote_runway:  Runway,
    blue:           Option<YesNoAny>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaveWithMockball {
    remote_runway:      Runway,
    landing_runway:     Runway,
    blue:               Option<YesNoAny>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaveWithSpringBallBounce {
    remote_runway:      Runway,
    landing_runway:     Runway,
    movement_type:      SpringBallMovement,
    blue:               Option<YesNoAny>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaveSpaceJumping {
    remote_runway:      Runway,
    blue:               YesNoAny,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaveWithGModeSetup {
    knockback:  Option<bool>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaveWithGMode {
    morphed:    bool,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaveWithDoorFrameBelow {
    height:     u8,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LeaveWithPlatformBelow {
    height:             u8,
    left_position:      u8,
    right_position:     u8,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GModeRegainMobility {}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnlocksDoor {
    types:                  Vec<UnlockDoorType>,
    node_id:                Option<NodeId>,
    requires:               Option<Requirement>,
    use_implicit_requires:  Option<bool>,
    note:                   Option<Note>,
    dev_note:               Option<DevNote>,
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
    name:           String,
    leads_to_node:  Option<NodeId>,
    cost:           Option<Vec<Requirement>>,
    softlock:       Option<bool>,
    note:           Option<Note>,
    dev_note:       Option<DevNote>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Yield(String);

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TwinDoorAddress {
    room_address:   String,
    door_address:   String,
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
    id:         NodeId,
    strats:     Vec<Strat>
}
