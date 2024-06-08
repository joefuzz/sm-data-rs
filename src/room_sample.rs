use crate::*;

const room: Room =

Room {
    id: RoomId(
        38,
    ),
    name: RoomName(
        "Morph Ball Room",
    ),
    area: Brinstar,
    subarea: Blue,
    subsubarea: None,
    playable: true,
    nodes: [
        Node {
            id: NodeId(
                1,
            ),
            name: NodeName(
                "Left Door",
            ),
            node_type: Door,
            node_sub_type: Gray,
            node_address: Some(
                NodeMemoryAddress(
                    "0x0018e9e",
                ),
            ),
            door_orientation: Some(
                Left,
            ),
            door_environments: Some(
                [
                    DoorEnvironment {
                        physics: Air,
                        entrance_nodes: None,
                        note: None,
                        dev_note: None,
                    },
                ],
            ),
            use_implicit_door_unlocks: None,
            interaction_requires: None,
            spawn_at: None,
            locks: Some(
                [
                    Lock {
                        name: "Morph Ball Room Gray Lock (to Green Hill Zone)",
                        lock_type: KillEnemies,
                        unlock_strats: [
                            Strat {
                                name: StratName(
                                    "Base",
                                ),
                                requires: [
                                    Condition(
                                        Logic(
                                            ObstaclesCleared(
                                                [
                                                    ObstacleId(
                                                        "C",
                                                    ),
                                                ],
                                            ),
                                        ),
                                    ),
                                ],
                                link: None,
                                notable: Some(
                                    false,
                                ),
                                reusable_roomwide_notable: None,
                                entrance_condition: None,
                                exit_condition: None,
                                g_mode_regain_mobility: None,
                                bypasses_door_shell: None,
                                unlocks_doors: None,
                                clears_obstacles: None,
                                resets_obstacles: None,
                                collects_items: None,
                                sets_flags: None,
                                flash_suit_checked: None,
                                failures: None,
                                note: None,
                                dev_note: Some(
                                    SingleLine(
                                        "Obstacle can be destroyed either going 1 -> 6 or 6 -> 1.",
                                    ),
                                ),
                            },
                        ],
                        lock: Some(
                            [
                                Condition(
                                    Flag(
                                        "f_ZebesAwake",
                                    ),
                                ),
                            ],
                        ),
                        note: None,
                        dev_note: Some(
                            SingleLine(
                                "Technically this yields f_ZebesAwake, although it requires the same flag to spawn the enemies, so it's not possible here.",
                            ),
                        ),
                        yields: None,
                    },
                ],
            ),
            twin_door_addresses: None,
            utility: None,
            viewable_nodes: None,
            yields: None,
            note: None,
            dev_note: None,
        },
        Node {
            id: NodeId(
                2,
            ),
            name: NodeName(
                "Elevator",
            ),
            node_type: Door,
            node_sub_type: Elevator,
            node_address: Some(
                NodeMemoryAddress(
                    "0x0018eb6",
                ),
            ),
            door_orientation: Some(
                Up,
            ),
            door_environments: None,
            use_implicit_door_unlocks: None,
            interaction_requires: None,
            spawn_at: None,
            locks: None,
            twin_door_addresses: None,
            utility: None,
            viewable_nodes: None,
            yields: None,
            note: None,
            dev_note: None,
        },
        Node {
            id: NodeId(
                3,
            ),
            name: NodeName(
                "Right Door",
            ),
            node_type: Door,
            node_sub_type: Blue,
            node_address: Some(
                NodeMemoryAddress(
                    "0x0018eaa",
                ),
            ),
            door_orientation: Some(
                Right,
            ),
            door_environments: Some(
                [
                    DoorEnvironment {
                        physics: Air,
                        entrance_nodes: None,
                        note: None,
                        dev_note: None,
                    },
                ],
            ),
            use_implicit_door_unlocks: None,
            interaction_requires: None,
            spawn_at: None,
            locks: None,
            twin_door_addresses: None,
            utility: None,
            viewable_nodes: None,
            yields: None,
            note: None,
            dev_note: None,
        },
        Node {
            id: NodeId(
                4,
            ),
            name: NodeName(
                "Right Item",
            ),
            node_type: Item,
            node_sub_type: Visible,
            node_address: Some(
                NodeMemoryAddress(
                    "0x786DE",
                ),
            ),
            door_orientation: None,
            door_environments: None,
            use_implicit_door_unlocks: None,
            interaction_requires: None,
            spawn_at: None,
            locks: None,
            twin_door_addresses: None,
            utility: None,
            viewable_nodes: None,
            yields: None,
            note: None,
            dev_note: Some(
                MultiLine(
                    [
                        "This item is technically no longer spawned after Zebes is awake, which would require canRiskPermanentLossOfAccess.",
                        "The logic here is not modeling that, as that bug is likely not going to be kept by anyone that utilizes this data.",
                    ],
                ),
            ),
        },
        Node {
            id: NodeId(
                5,
            ),
            name: NodeName(
                "Left Item",
            ),
            node_type: Item,
            node_sub_type: Visible,
            node_address: Some(
                NodeMemoryAddress(
                    "0x7874C",
                ),
            ),
            door_orientation: None,
            door_environments: None,
            use_implicit_door_unlocks: None,
            interaction_requires: None,
            spawn_at: None,
            locks: Some(
                [
                    Lock {
                        name: "Blue Brinstar Power Bombs Spawn Lock",
                        lock_type: GameFlag,
                        unlock_strats: [
                            Strat {
                                name: StratName(
                                    "Base",
                                ),
                                requires: [
                                    LogicalOr(
                                        [
                                            Condition(
                                                Tech(
                                                    "h_ZebesIsAwake",
                                                ),
                                            ),
                                            Condition(
                                                Tech(
                                                    "h_AllItemsSpawned",
                                                ),
                                            ),
                                        ],
                                    ),
                                ],
                                link: None,
                                notable: Some(
                                    false,
                                ),
                                reusable_roomwide_notable: None,
                                entrance_condition: None,
                                exit_condition: None,
                                g_mode_regain_mobility: None,
                                bypasses_door_shell: None,
                                unlocks_doors: None,
                                clears_obstacles: None,
                                resets_obstacles: None,
                                collects_items: None,
                                sets_flags: None,
                                flash_suit_checked: None,
                                failures: None,
                                note: None,
                                dev_note: None,
                            },
                        ],
                        lock: None,
                        note: Some(
                            SingleLine(
                                "Item doesn't appear before Zebes is awakened.",
                            ),
                        ),
                        dev_note: None,
                        yields: None,
                    },
                ],
            ),
            twin_door_addresses: None,
            utility: None,
            viewable_nodes: None,
            yields: None,
            note: None,
            dev_note: None,
        },
        Node {
            id: NodeId(
                6,
            ),
            name: NodeName(
                "Junction (Left of Morph Tunnel)",
            ),
            node_type: Junction,
            node_sub_type: Junction,
            node_address: None,
            door_orientation: None,
            door_environments: None,
            use_implicit_door_unlocks: None,
            interaction_requires: None,
            spawn_at: None,
            locks: None,
            twin_door_addresses: None,
            utility: None,
            viewable_nodes: None,
            yields: None,
            note: None,
            dev_note: None,
        },
    ],
    links: [
        Link {
            from: NodeId(
                1,
            ),
            to: Some(
                [
                    LinkEnd {
                        id: NodeId(
                            1,
                        ),
                        note: None,
                        dev_note: None,
                    },
                    LinkEnd {
                        id: NodeId(
                            5,
                        ),
                        note: None,
                        dev_note: None,
                    },
                    LinkEnd {
                        id: NodeId(
                            6,
                        ),
                        note: Some(
                            SingleLine(
                                "Requires either tanking one hit, or killing the Sidehoppers",
                            ),
                        ),
                        dev_note: None,
                    },
                ],
            ),
        },
        Link {
            from: NodeId(
                2,
            ),
            to: Some(
                [
                    LinkEnd {
                        id: NodeId(
                            2,
                        ),
                        note: None,
                        dev_note: None,
                    },
                    LinkEnd {
                        id: NodeId(
                            3,
                        ),
                        note: None,
                        dev_note: None,
                    },
                    LinkEnd {
                        id: NodeId(
                            4,
                        ),
                        note: None,
                        dev_note: None,
                    },
                ],
            ),
        },
        Link {
            from: NodeId(
                3,
            ),
            to: Some(
                [
                    LinkEnd {
                        id: NodeId(
                            1,
                        ),
                        note: None,
                        dev_note: None,
                    },
                    LinkEnd {
                        id: NodeId(
                            2,
                        ),
                        note: None,
                        dev_note: None,
                    },
                    LinkEnd {
                        id: NodeId(
                            3,
                        ),
                        note: None,
                        dev_note: None,
                    },
                    LinkEnd {
                        id: NodeId(
                            5,
                        ),
                        note: None,
                        dev_note: None,
                    },
                ],
            ),
        },
        Link {
            from: NodeId(
                4,
            ),
            to: Some(
                [
                    LinkEnd {
                        id: NodeId(
                            2,
                        ),
                        note: None,
                        dev_note: None,
                    },
                    LinkEnd {
                        id: NodeId(
                            4,
                        ),
                        note: None,
                        dev_note: None,
                    },
                    LinkEnd {
                        id: NodeId(
                            5,
                        ),
                        note: None,
                        dev_note: None,
                    },
                ],
            ),
        },
        Link {
            from: NodeId(
                5,
            ),
            to: Some(
                [
                    LinkEnd {
                        id: NodeId(
                            4,
                        ),
                        note: None,
                        dev_note: None,
                    },
                    LinkEnd {
                        id: NodeId(
                            5,
                        ),
                        note: None,
                        dev_note: None,
                    },
                    LinkEnd {
                        id: NodeId(
                            6,
                        ),
                        note: None,
                        dev_note: None,
                    },
                ],
            ),
        },
        Link {
            from: NodeId(
                6,
            ),
            to: Some(
                [
                    LinkEnd {
                        id: NodeId(
                            1,
                        ),
                        note: None,
                        dev_note: None,
                    },
                    LinkEnd {
                        id: NodeId(
                            5,
                        ),
                        note: None,
                        dev_note: None,
                    },
                ],
            ),
        },
    ],
    strats: [
        Strat {
            name: StratName(
                "Leave with Runway",
            ),
            requires: [
                Condition(
                    Logic(
                        ObstaclesCleared(
                            [
                                ObstacleId(
                                    "C",
                                ),
                            ],
                        ),
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        1,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: Some(
                LeaveWithRunway(
                    Runway {
                        length: 4,
                        open_end: 1,
                        gentle_up_tiles: None,
                        gentle_down_tiles: None,
                        steep_up_tiles: None,
                        steep_down_tiles: None,
                        min_tiles: None,
                    },
                ),
            ),
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Leave Charged",
            ),
            requires: [
                Condition(
                    Logic(
                        ObstaclesCleared(
                            [
                                ObstacleId(
                                    "C",
                                ),
                            ],
                        ),
                    ),
                ),
                Condition(
                    Tech(
                        "canShinechargeMovement",
                    ),
                ),
                Condition(
                    Logic(
                        CanShineCharge(
                            SpeedConditions {
                                used_tiles: 25,
                                open_end: 0,
                                gentle_up_tiles: None,
                                gentle_down_tiles: None,
                                steep_up_tiles: None,
                                steep_down_tiles: None,
                                starting_down_tiles: None,
                            },
                        ),
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        1,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: Some(
                LeaveShinecharged(
                    FramesRemaining(
                        135,
                    ),
                ),
            ),
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: Some(
                true,
            ),
            failures: None,
            note: None,
            dev_note: Some(
                SingleLine(
                    "Destroy obstacle C by travelling between nodes 1 and 6.",
                ),
            ),
        },
        Strat {
            name: StratName(
                "G-Mode Setup - Get Hit By Sidehopper",
            ),
            requires: [
                Condition(
                    Tech(
                        "h_ZebesIsAwake",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        1,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: Some(
                LeaveWithGModeSetup {
                    knockback: None,
                },
            ),
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: Some(
                true,
            ),
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "G-Mode Regain Mobility",
            ),
            requires: [
                Condition(
                    Logic(
                        EnemyDamage {
                            enemy: "Sidehopper",
                            enemy_type: "contact",
                            hits: 1,
                        },
                    ),
                ),
                Condition(
                    Tech(
                        "h_ZebesIsAwake",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        1,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: Some(
                GModeRegainMobility,
            ),
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: Some(
                true,
            ),
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Crystal Flash",
            ),
            requires: [
                Condition(
                    Logic(
                        ObstaclesCleared(
                            [
                                ObstacleId(
                                    "C",
                                ),
                            ],
                        ),
                    ),
                ),
                Condition(
                    Tech(
                        "h_canCrystalFlash",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        1,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: Some(
                true,
            ),
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Room Entry Speedball",
            ),
            requires: [
                Condition(
                    Tech(
                        "canSpeedball",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        5,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: Some(
                ComeInShinecharging(
                    Runway {
                        length: 2,
                        open_end: 0,
                        gentle_up_tiles: None,
                        gentle_down_tiles: None,
                        steep_up_tiles: None,
                        steep_down_tiles: None,
                        min_tiles: None,
                    },
                ),
            ),
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "A",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: Some(
                MultiLine(
                    [
                        "This is a direct link because clearing the hoppers is unnecessary.",
                        "One tile is subtracted from the runway length since Samus must be blue before contacting the first Hopper.",
                        "There is 1 unusable tile in this runway.",
                    ],
                ),
            ),
        },
        Strat {
            name: StratName(
                "G-Mode Morph Overload PLMs Zebes Asleep",
            ),
            requires: [
                Condition(
                    Tech(
                        "h_ZebesNotAwake",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        5,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: Some(
                ComeInWithGMode {
                    mode: Any,
                    morphed: true,
                    mobility: None,
                },
            ),
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "D",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: Some(
                true,
            ),
            failures: None,
            note: Some(
                SingleLine(
                    "Overload the PLMs by rolling through the camera scroll blocks which are 4 tiles to the right of the stair by the door.",
                ),
            ),
            dev_note: None,
        },
        Strat {
            name: StratName(
                "G-Mode Overload PLMs Indirect",
            ),
            requires: [
                LogicalOr(
                    [
                        Condition(
                            Logic(
                                EnemyDamage {
                                    enemy: "Sidehopper",
                                    enemy_type: "contact",
                                    hits: 2,
                                },
                            ),
                        ),
                        Condition(
                            Logic(
                                Ammo(
                                    AmmoAmount {
                                        ammo_type: PowerBomb,
                                        count: 1,
                                    },
                                ),
                            ),
                        ),
                        LogicalAnd(
                            [
                                Condition(
                                    Equipment(
                                        Morph,
                                    ),
                                ),
                                LogicalOr(
                                    [
                                        LogicalAnd(
                                            [
                                                Condition(
                                                    Equipment(
                                                        Plasma,
                                                    ),
                                                ),
                                                Condition(
                                                    Equipment(
                                                        Charge,
                                                    ),
                                                ),
                                            ],
                                        ),
                                        Condition(
                                            Equipment(
                                                ScrewAttack,
                                            ),
                                        ),
                                    ],
                                ),
                            ],
                        ),
                    ],
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        5,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: Some(
                ComeInWithGMode {
                    mode: Indirect,
                    morphed: true,
                    mobility: None,
                },
            ),
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "D",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: Some(
                true,
            ),
            failures: None,
            note: Some(
                MultiLine(
                    [
                        "Either kill the Sidehoppers immediately on entry by placing a Power Bomb while rolling off the stair or tank their hits.",
                        "If Samus has Morph Ball, it is also possible to kill the Sidehoppers upon room entry with Screw Attack or a powerful beam.",
                        "Overload the PLMs by rolling through the camera scroll blocks which are 4 tiles to the right of the stair by the door.",
                        "Roll under the last Sidehopper and through the bomb blocks.",
                    ],
                ),
            ),
            dev_note: None,
        },
        Strat {
            name: StratName(
                "G-Mode Morph Overload PLMs Direct",
            ),
            requires: [
                LogicalOr(
                    [
                        Condition(
                            Logic(
                                EnemyDamage {
                                    enemy: "Sidehopper",
                                    enemy_type: "contact",
                                    hits: 3,
                                },
                            ),
                        ),
                        LogicalAnd(
                            [
                                Condition(
                                    Logic(
                                        EnemyDamage {
                                            enemy: "Sidehopper",
                                            enemy_type: "contact",
                                            hits: 1,
                                        },
                                    ),
                                ),
                                Condition(
                                    Logic(
                                        Ammo(
                                            AmmoAmount {
                                                ammo_type: PowerBomb,
                                                count: 1,
                                            },
                                        ),
                                    ),
                                ),
                            ],
                        ),
                        LogicalAnd(
                            [
                                Condition(
                                    Equipment(
                                        Morph,
                                    ),
                                ),
                                Condition(
                                    Equipment(
                                        ScrewAttack,
                                    ),
                                ),
                                Condition(
                                    Tech(
                                        "canTrickyJump",
                                    ),
                                ),
                            ],
                        ),
                    ],
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        5,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: Some(
                ComeInWithGMode {
                    mode: Direct,
                    morphed: true,
                    mobility: None,
                },
            ),
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "D",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: Some(
                true,
            ),
            failures: None,
            note: Some(
                MultiLine(
                    [
                        "Tank the Sidehopper hits or kill them after a single hit by quickly placing a Power Bomb.",
                        "It is also possible to kill the Sidehoppers with a very fast Screw Attack, if Samus has Morph Ball.",
                        "Overload the PLMs by rolling through the camera scroll blocks which are 4 tiles to the right of the stair by the door.",
                        "Roll under the last Sidehopper and through the bomb blocks.",
                    ],
                ),
            ),
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Run Through",
            ),
            requires: [
                LogicalOr(
                    [
                        Condition(
                            Logic(
                                EnemyDamage {
                                    enemy: "Sidehopper",
                                    enemy_type: "contact",
                                    hits: 1,
                                },
                            ),
                        ),
                        Condition(
                            Logic(
                                ObstaclesCleared(
                                    [
                                        ObstacleId(
                                            "C",
                                        ),
                                    ],
                                ),
                            ),
                        ),
                    ],
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        6,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Power Beam Sidehopper Kill",
            ),
            requires: [
                Condition(
                    Logic(
                        EnemyDamage {
                            enemy: "Sidehopper",
                            enemy_type: "contact",
                            hits: 6,
                        },
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        6,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "C",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Quick Sidehopper Kill",
            ),
            requires: [
                Condition(
                    Logic(
                        EnemyDamage {
                            enemy: "Sidehopper",
                            enemy_type: "contact",
                            hits: 1,
                        },
                    ),
                ),
                LogicalOr(
                    [
                        Condition(
                            Logic(
                                KillEnemies(
                                    EnemiesToKill {
                                        enemies: [
                                            [
                                                "Sidehopper",
                                                "Sidehopper",
                                            ],
                                            [
                                                "Sidehopper",
                                            ],
                                        ],
                                        explicit_weapons: Some(
                                            [
                                                "Missile",
                                                "Super",
                                                "PowerBomb",
                                                "ScrewAttack",
                                                "Plasma",
                                            ],
                                        ),
                                        excluded_weapons: None,
                                        farmable_ammo: None,
                                    },
                                ),
                            ),
                        ),
                    ],
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        6,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "C",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: Some(
                MultiLine(
                    [
                        "For the Power Bomb and Screw Attack kills, this strat assumes you don't know you're entering the room beforehand.",
                        "If you know, it's possible to Power Bomb kill the first two Sidehoppers damage-free by morphing before entering.",
                        "For Screw, just entering with a spin jump would work.",
                    ],
                ),
            ),
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Spark into Room",
            ),
            requires: [
                Condition(
                    Logic(
                        Shinespark {
                            frames: 40,
                            excess_frames: None,
                        },
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        6,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: Some(
                ComeInWithSpark {
                    position: None,
                },
            ),
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "C",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Kill on Entry",
            ),
            requires: [
                Condition(
                    Tech(
                        "canPrepareForNextRoom",
                    ),
                ),
                LogicalOr(
                    [
                        LogicalAnd(
                            [
                                Condition(
                                    Equipment(
                                        Plasma,
                                    ),
                                ),
                                Condition(
                                    Equipment(
                                        Charge,
                                    ),
                                ),
                            ],
                        ),
                        Condition(
                            Equipment(
                                ScrewAttack,
                            ),
                        ),
                        LogicalAnd(
                            [
                                Condition(
                                    Tech(
                                        "h_canUsePowerBombs",
                                    ),
                                ),
                                Condition(
                                    Logic(
                                        KillEnemies(
                                            EnemiesToKill {
                                                enemies: [
                                                    [
                                                        "Sidehopper",
                                                    ],
                                                ],
                                                explicit_weapons: Some(
                                                    [
                                                        "Missile",
                                                        "Super",
                                                        "PowerBomb",
                                                    ],
                                                ),
                                                excluded_weapons: None,
                                                farmable_ammo: None,
                                            },
                                        ),
                                    ),
                                ),
                            ],
                        ),
                    ],
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        6,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "C",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: Some(
                MultiLine(
                    [
                        "Enter the room ready to kill the hoppers before they can hit Samus.",
                        "with a charge plasma shot ready, or in a screw attack spin jump, or a morph, ready to lay a power bomb.",
                    ],
                ),
            ),
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Power Bomb Sidehopper Kill and Bomb Blocks",
            ),
            requires: [
                Condition(
                    Logic(
                        EnemyDamage {
                            enemy: "Sidehopper",
                            enemy_type: "contact",
                            hits: 1,
                        },
                    ),
                ),
                LogicalOr(
                    [
                        Condition(
                            Tech(
                                "canMockball",
                            ),
                        ),
                        Condition(
                            Logic(
                                EnemyDamage {
                                    enemy: "Sidehopper",
                                    enemy_type: "contact",
                                    hits: 1,
                                },
                            ),
                        ),
                    ],
                ),
                Condition(
                    Logic(
                        KillEnemies(
                            EnemiesToKill {
                                enemies: [
                                    [
                                        "Sidehopper",
                                        "Sidehopper",
                                    ],
                                    [
                                        "Sidehopper",
                                    ],
                                ],
                                explicit_weapons: Some(
                                    [
                                        "PowerBomb",
                                    ],
                                ),
                                excluded_weapons: None,
                                farmable_ammo: None,
                            },
                        ),
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        6,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "A",
                    ),
                    ObstacleId(
                        "C",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: Some(
                MultiLine(
                    [
                        "This strat assumes you don't know you're entering the room beforehand. Otherwise, it's possible to kill the first two Sidehoppers without taking damage.",
                        "It's possible to take out obstacle A alongside the third Sidehopper, but there's a risk of taking an additional hit (negated by mockball).",
                    ],
                ),
            ),
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Medium Sidehopper Kill",
            ),
            requires: [
                Condition(
                    Logic(
                        EnemyDamage {
                            enemy: "Sidehopper",
                            enemy_type: "contact",
                            hits: 3,
                        },
                    ),
                ),
                LogicalOr(
                    [
                        Condition(
                            Equipment(
                                Spazer,
                            ),
                        ),
                        Condition(
                            Equipment(
                                Wave,
                            ),
                        ),
                    ],
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        6,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "C",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Free Passage",
            ),
            requires: [
                Condition(
                    Tech(
                        "h_ZebesNotAwake",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        1,
                    ),
                    NodeId(
                        6,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "C",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: Some(
                SingleLine(
                    "The obstacle isn't really cleared, but should enable all strats that require it to be.",
                ),
            ),
        },
        Strat {
            name: StratName(
                "Leave Charged",
            ),
            requires: [
                Condition(
                    Logic(
                        CanShineCharge(
                            SpeedConditions {
                                used_tiles: 30,
                                open_end: 0,
                                gentle_up_tiles: None,
                                gentle_down_tiles: None,
                                steep_up_tiles: None,
                                steep_down_tiles: None,
                                starting_down_tiles: None,
                            },
                        ),
                    ),
                ),
                Condition(
                    Tech(
                        "canShinechargeMovement",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        2,
                    ),
                    NodeId(
                        2,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: Some(
                LeaveShinecharged(
                    FramesRemaining(
                        130,
                    ),
                ),
            ),
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: Some(
                true,
            ),
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Base",
            ),
            requires: [],
            link: Some(
                (
                    NodeId(
                        2,
                    ),
                    NodeId(
                        3,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Base",
            ),
            requires: [],
            link: Some(
                (
                    NodeId(
                        2,
                    ),
                    NodeId(
                        4,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Grapple Teleport",
            ),
            requires: [],
            link: Some(
                (
                    NodeId(
                        3,
                    ),
                    NodeId(
                        1,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: Some(
                ComeInWithGrappleTeleport {
                    block_positions: [
                        (
                            2,
                            34,
                        ),
                    ],
                },
            ),
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: Some(
                true,
            ),
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Carry Grapple Teleport",
            ),
            requires: [],
            link: Some(
                (
                    NodeId(
                        3,
                    ),
                    NodeId(
                        1,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: Some(
                ComeInWithGrappleTeleport {
                    block_positions: [
                        (
                            2,
                            34,
                        ),
                    ],
                },
            ),
            exit_condition: Some(
                LeaveWithGrappleTeleport {
                    block_positions: [
                        (
                            2,
                            34,
                        ),
                    ],
                },
            ),
            g_mode_regain_mobility: None,
            bypasses_door_shell: Some(
                true,
            ),
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Base",
            ),
            requires: [],
            link: Some(
                (
                    NodeId(
                        3,
                    ),
                    NodeId(
                        2,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Carry G-Mode Up the Elevator",
            ),
            requires: [],
            link: Some(
                (
                    NodeId(
                        3,
                    ),
                    NodeId(
                        2,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: Some(
                ComeInWithGMode {
                    mode: Any,
                    morphed: false,
                    mobility: None,
                },
            ),
            exit_condition: Some(
                LeaveWithGMode {
                    morphed: false,
                },
            ),
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: Some(
                true,
            ),
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Leave with Runway",
            ),
            requires: [],
            link: Some(
                (
                    NodeId(
                        3,
                    ),
                    NodeId(
                        3,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: Some(
                LeaveWithRunway(
                    Runway {
                        length: 5,
                        open_end: 1,
                        gentle_up_tiles: None,
                        gentle_down_tiles: None,
                        steep_up_tiles: None,
                        steep_down_tiles: None,
                        min_tiles: None,
                    },
                ),
            ),
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Leave Charged",
            ),
            requires: [
                Condition(
                    Logic(
                        CanShineCharge(
                            SpeedConditions {
                                used_tiles: 30,
                                open_end: 0,
                                gentle_up_tiles: None,
                                gentle_down_tiles: None,
                                steep_up_tiles: None,
                                steep_down_tiles: None,
                                starting_down_tiles: None,
                            },
                        ),
                    ),
                ),
                Condition(
                    Tech(
                        "canShinechargeMovement",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        3,
                    ),
                    NodeId(
                        3,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: Some(
                LeaveShinecharged(
                    FramesRemaining(
                        130,
                    ),
                ),
            ),
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: Some(
                true,
            ),
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "G-Mode Morph Power Bomb the Blocks",
            ),
            requires: [
                Condition(
                    Tech(
                        "h_canArtificialMorphPowerBomb",
                    ),
                ),
                Condition(
                    Tech(
                        "h_canArtificialMorphMovement",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        3,
                    ),
                    NodeId(
                        5,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: Some(
                ComeInWithGMode {
                    mode: Any,
                    morphed: true,
                    mobility: None,
                },
            ),
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "B",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: Some(
                true,
            ),
            failures: None,
            note: Some(
                SingleLine(
                    "Place the Power Bomb, then exit g-mode before the bomb goes off.",
                ),
            ),
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Base",
            ),
            requires: [
                LogicalOr(
                    [
                        Condition(
                            Tech(
                                "canWalljump",
                            ),
                        ),
                        Condition(
                            Equipment(
                                Morph,
                            ),
                        ),
                        Condition(
                            Equipment(
                                SpaceJump,
                            ),
                        ),
                        Condition(
                            Equipment(
                                HiJump,
                            ),
                        ),
                    ],
                ),
            ],
            link: Some(
                (
                    NodeId(
                        4,
                    ),
                    NodeId(
                        2,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Careful Jump",
            ),
            requires: [
                Condition(
                    Tech(
                        "canCarefulJump",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        4,
                    ),
                    NodeId(
                        2,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Crouch Jump Down Grab",
            ),
            requires: [
                Condition(
                    Tech(
                        "h_canCrouchJumpDownGrab",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        4,
                    ),
                    NodeId(
                        2,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Crystal Flash",
            ),
            requires: [
                Condition(
                    Tech(
                        "h_canCrystalFlash",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        4,
                    ),
                    NodeId(
                        4,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "B",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: Some(
                true,
            ),
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Base",
            ),
            requires: [
                LogicalOr(
                    [
                        Condition(
                            Tech(
                                "h_canUsePowerBombs",
                            ),
                        ),
                        Condition(
                            Logic(
                                ObstaclesCleared(
                                    [
                                        ObstacleId(
                                            "B",
                                        ),
                                    ],
                                ),
                            ),
                        ),
                    ],
                ),
            ],
            link: Some(
                (
                    NodeId(
                        4,
                    ),
                    NodeId(
                        5,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "B",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Base",
            ),
            requires: [
                LogicalOr(
                    [
                        Condition(
                            Tech(
                                "h_canUsePowerBombs",
                            ),
                        ),
                        Condition(
                            Logic(
                                ObstaclesCleared(
                                    [
                                        ObstacleId(
                                            "B",
                                        ),
                                    ],
                                ),
                            ),
                        ),
                    ],
                ),
            ],
            link: Some(
                (
                    NodeId(
                        5,
                    ),
                    NodeId(
                        4,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "B",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Power Bomb Blocks While in Artificial Morph from the Left",
            ),
            requires: [
                Condition(
                    Logic(
                        ObstaclesCleared(
                            [
                                ObstacleId(
                                    "D",
                                ),
                            ],
                        ),
                    ),
                ),
                Condition(
                    Tech(
                        "h_canArtificialMorphPowerBomb",
                    ),
                ),
                LogicalOr(
                    [
                        Condition(
                            Tech(
                                "h_canArtificialMorphSpringBall",
                            ),
                        ),
                        Condition(
                            Tech(
                                "h_canArtificialMorphPowerBomb",
                            ),
                        ),
                        Condition(
                            Tech(
                                "h_canArtificialMorphBombs",
                            ),
                        ),
                    ],
                ),
            ],
            link: Some(
                (
                    NodeId(
                        5,
                    ),
                    NodeId(
                        5,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "B",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: Some(
                SingleLine(
                    "Place the Power Bomb, then exit g-mode before the bomb goes off.",
                ),
            ),
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Blocks Already Broken",
            ),
            requires: [
                Condition(
                    Equipment(
                        Morph,
                    ),
                ),
                Condition(
                    Logic(
                        ObstaclesCleared(
                            [
                                ObstacleId(
                                    "A",
                                ),
                            ],
                        ),
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        5,
                    ),
                    NodeId(
                        6,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Bomb the Blocks",
            ),
            requires: [
                Condition(
                    Tech(
                        "h_canBombThings",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        5,
                    ),
                    NodeId(
                        6,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "A",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Morph Ball Room Speedball (Right to Left)",
            ),
            requires: [
                Condition(
                    Logic(
                        ObstaclesCleared(
                            [
                                ObstacleId(
                                    "B",
                                ),
                            ],
                        ),
                    ),
                ),
                Condition(
                    Logic(
                        CanShineCharge(
                            SpeedConditions {
                                used_tiles: 21,
                                open_end: 1,
                                gentle_up_tiles: None,
                                gentle_down_tiles: None,
                                steep_up_tiles: None,
                                steep_down_tiles: None,
                                starting_down_tiles: None,
                            },
                        ),
                    ),
                ),
                Condition(
                    Tech(
                        "canSpeedball",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        5,
                    ),
                    NodeId(
                        6,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "A",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: Some(
                MultiLine(
                    [
                        "It's a short charge into a speedball to break the Bomb Blocks.",
                        "The Power Bomb Blocks need to be destroyed to have enough running room.",
                    ],
                ),
            ),
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Laugh at Dead Sidehoppers",
            ),
            requires: [
                Condition(
                    Logic(
                        ObstaclesCleared(
                            [
                                ObstacleId(
                                    "C",
                                ),
                            ],
                        ),
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        6,
                    ),
                    NodeId(
                        1,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Tank the damage",
            ),
            requires: [
                Condition(
                    Logic(
                        EnemyDamage {
                            enemy: "Sidehopper",
                            enemy_type: "contact",
                            hits: 1,
                        },
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        6,
                    ),
                    NodeId(
                        1,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Safe Sidehopper Power Beam Kill",
            ),
            requires: [
                Condition(
                    Equipment(
                        Morph,
                    ),
                ),
                Condition(
                    Logic(
                        ObstaclesCleared(
                            [
                                ObstacleId(
                                    "A",
                                ),
                            ],
                        ),
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        6,
                    ),
                    NodeId(
                        1,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "C",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: Some(
                SingleLine(
                    "If the bomb blocks are broken, the SideHoppers can be killed safely from behind with Power Beam.",
                ),
            ),
            dev_note: Some(
                SingleLine(
                    "Obstacle can be broken by going to 5 and back.",
                ),
            ),
        },
        Strat {
            name: StratName(
                "Free Passage",
            ),
            requires: [
                Condition(
                    Tech(
                        "h_ZebesNotAwake",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        6,
                    ),
                    NodeId(
                        1,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "C",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: Some(
                SingleLine(
                    "The obstacle isn't really cleared, but should enable all strats that require it to be.",
                ),
            ),
        },
        Strat {
            name: StratName(
                "Blocks Already Broken",
            ),
            requires: [
                Condition(
                    Equipment(
                        Morph,
                    ),
                ),
                Condition(
                    Logic(
                        ObstaclesCleared(
                            [
                                ObstacleId(
                                    "A",
                                ),
                            ],
                        ),
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        6,
                    ),
                    NodeId(
                        5,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: None,
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Bomb the Blocks",
            ),
            requires: [
                Condition(
                    Tech(
                        "h_canBombThings",
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        6,
                    ),
                    NodeId(
                        5,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "A",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: None,
            dev_note: None,
        },
        Strat {
            name: StratName(
                "Morph Ball Room Speedball (Left to Right)",
            ),
            requires: [
                Condition(
                    Tech(
                        "canSpeedball",
                    ),
                ),
                Condition(
                    Logic(
                        ObstaclesCleared(
                            [
                                ObstacleId(
                                    "C",
                                ),
                            ],
                        ),
                    ),
                ),
                Condition(
                    Logic(
                        CanShineCharge(
                            SpeedConditions {
                                used_tiles: 18,
                                open_end: 0,
                                gentle_up_tiles: None,
                                gentle_down_tiles: None,
                                steep_up_tiles: None,
                                steep_down_tiles: None,
                                starting_down_tiles: None,
                            },
                        ),
                    ),
                ),
            ],
            link: Some(
                (
                    NodeId(
                        6,
                    ),
                    NodeId(
                        5,
                    ),
                ),
            ),
            notable: None,
            reusable_roomwide_notable: None,
            entrance_condition: None,
            exit_condition: None,
            g_mode_regain_mobility: None,
            bypasses_door_shell: None,
            unlocks_doors: None,
            clears_obstacles: Some(
                [
                    ObstacleId(
                        "A",
                    ),
                ],
            ),
            resets_obstacles: None,
            collects_items: None,
            sets_flags: None,
            flash_suit_checked: None,
            failures: None,
            note: Some(
                MultiLine(
                    [
                        "It's a short charge into a speedball to break the bomb blocks.",
                        "The Sidehoppers need to be destroyed beforehand to clear the running space.",
                    ],
                ),
            ),
            dev_note: None,
        },
    ],
    room_address: Some(
        RoomMemoryAddress(
            "0x79E9F",
        ),
    ),
    obstacles: Some(
        [
            Obstacle {
                id: ObstacleId(
                    "A",
                ),
                name: "Bomb Blocks",
                obstacle_type: Inanimate,
                note: None,
                dev_note: None,
            },
            Obstacle {
                id: ObstacleId(
                    "B",
                ),
                name: "Power Bomb Blocks",
                obstacle_type: Inanimate,
                note: None,
                dev_note: None,
            },
            Obstacle {
                id: ObstacleId(
                    "C",
                ),
                name: "Sidehopper trio",
                obstacle_type: Enemies,
                note: None,
                dev_note: None,
            },
            Obstacle {
                id: ObstacleId(
                    "D",
                ),
                name: "At the Power Bomb Item While in Artificial Morph",
                obstacle_type: Abstract,
                note: None,
                dev_note: None,
            },
        ],
    ),
    enemies: Some(
        [
            Enemy {
                id: EnemyId(
                    "e1",
                ),
                group_name: EnemyGroup(
                    "Morph Ball Room Sidehoppers",
                ),
                enemy_name: EnemyName(
                    "Sidehopper",
                ),
                quantity: 3,
                home_nodes: Some(
                    [
                        NodeId(
                            1,
                        ),
                        NodeId(
                            6,
                        ),
                    ],
                ),
                between_nodes: None,
                spawn: Some(
                    [
                        Condition(
                            Tech(
                                "f_ZebesAwake",
                            ),
                        ),
                    ],
                ),
                stop_spawn: None,
                drop_requires: None,
                farm_cycles: None,
                note: None,
                dev_note: None,
            },
        ],
    ),
    reusable_roomwide_notable: None,
    note: None,
    dev_note: None,
};