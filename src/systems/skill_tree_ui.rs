use bevy::prelude::*;
use crate::components::{
    ship::Player,
    upgrades::{PlayerUpgrades, UpgradeType},
    ship_classes::{ShipClass, ClassProgression},
    resources::Inventory,
};
use crate::systems::stat_visualization;

/// Root marker for skill tree UI
#[derive(Component)]
pub struct SkillTreeRoot;

/// Tab marker for class selection
#[derive(Component)]
pub struct ClassTab {
    pub class: ShipClass,
}

/// Currently active class tab
#[derive(Resource)]
pub struct ActiveClassTab(pub ShipClass);

impl Default for ActiveClassTab {
    fn default() -> Self {
        Self(ShipClass::Fighter)
    }
}

/// Skill node button marker
#[derive(Component)]
pub struct SkillNode {
    pub upgrade_type: UpgradeType,
}

/// Stat panel marker
#[derive(Component)]
pub struct StatPanel;

/// Radar chart marker
#[derive(Component)]
pub struct RadarChart;

/// Node tooltip marker
#[derive(Component)]
pub struct NodeTooltip {
    pub upgrade_type: UpgradeType,
}

/// Currently hovered node for preview
#[derive(Resource, Default)]
pub struct HoveredNode(pub Option<UpgradeType>);

/// Setup advanced skill tree UI
pub fn setup_skill_tree_ui(
    mut commands: Commands,
    inventory: Res<Inventory>,
    class_progression: Res<ClassProgression>,
    active_tab: Res<ActiveClassTab>,
) {
    println!("[Skill Tree UI] Setting up advanced skill tree interface");
    
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::srgb(0.05, 0.05, 0.1).into(),
            ..default()
        },
        SkillTreeRoot,
    )).with_children(|parent| {
        // Header with title and skill points
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
            ..default()
        }).with_children(|header| {
            // Title
            header.spawn(TextBundle::from_section(
                "SKILL TREE",
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            
            // Skill points display
            header.spawn(TextBundle::from_section(
                format!("Skill Points: {} / {}", 
                    class_progression.available_points(),
                    class_progression.total_skill_points
                ),
                TextStyle {
                    font_size: 28.0,
                    color: Color::srgb(1.0, 0.8, 0.2),
                    ..default()
                },
            ));
        });
        
        // Class tabs
        spawn_class_tabs(parent, &class_progression, &active_tab);
        
        // Main content area
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                ..default()
            },
            ..default()
        }).with_children(|content| {
            // Left panel: Stats
            spawn_stat_panel(content);
            
            // Right panel: Skill tree
            spawn_skill_tree_panel(content, &active_tab, &class_progression);
        });
        
        // Footer with instructions
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(10.0)),
                ..default()
            },
            ..default()
        }).with_children(|footer| {
            footer.spawn(TextBundle::from_section(
                "Click nodes to unlock | Hover for details | ESC to close",
                TextStyle {
                    font_size: 16.0,
                    color: Color::srgb(0.6, 0.6, 0.6),
                    ..default()
                },
            ));
        });
    });
}

/// Spawn class tabs
fn spawn_class_tabs(
    parent: &mut ChildBuilder,
    class_progression: &ClassProgression,
    active_tab: &ActiveClassTab,
) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Px(60.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(10.0),
            margin: UiRect::bottom(Val::Px(15.0)),
            ..default()
        },
        ..default()
    }).with_children(|tabs| {
        for class in [
            ShipClass::Fighter,
            ShipClass::Tank,
            ShipClass::Gunner,
            ShipClass::Stealth,
            ShipClass::Sniper,
            ShipClass::MissileTanker,
        ] {
            let is_active = class == active_tab.0;
            let points = class_progression.get_points(class);
            
            tabs.spawn((
                ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(15.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: if is_active {
                        Color::srgb(0.3, 0.3, 0.5)
                    } else {
                        Color::srgb(0.15, 0.15, 0.2)
                    }.into(),
                    border_color: if is_active {
                        Color::srgb(0.6, 0.8, 1.0)
                    } else {
                        Color::srgb(0.3, 0.3, 0.3)
                    }.into(),
                    ..default()
                },
                ClassTab { class },
            )).with_children(|button| {
                button.spawn(TextBundle::from_section(
                    format!("{} ({})", class.name(), points),
                    TextStyle {
                        font_size: 18.0,
                        color: if is_active {
                            Color::WHITE
                        } else {
                            Color::srgb(0.7, 0.7, 0.7)
                        },
                        ..default()
                    },
                ));
            });
        }
    });
}

/// Spawn stat panel with radar chart
fn spawn_stat_panel(parent: &mut ChildBuilder) {
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(350.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::all(Val::Px(2.0)),
                row_gap: Val::Px(10.0),
                ..default()
            },
            background_color: Color::srgb(0.1, 0.1, 0.15).into(),
            border_color: Color::srgb(0.3, 0.3, 0.4).into(),
            ..default()
        },
        StatPanel,
    )).with_children(|panel| {
        // Stats title
        panel.spawn(TextBundle::from_section(
            "SHIP STATISTICS",
            TextStyle {
                font_size: 24.0,
                color: Color::srgb(0.8, 0.9, 1.0),
                ..default()
            },
        ));
        
        // Stat list (will be populated by update system)
        panel.spawn((
            TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font_size: 14.0,
                    color: Color::srgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ),
            StatListText,
        ));
        
        // Radar chart
        panel.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(250.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.05, 0.05, 0.08).into(),
                ..default()
            },
            RadarChart,
        )).with_children(|chart_container| {
            // Spawn actual radar chart
            stat_visualization::spawn_radar_chart(chart_container, stat_visualization::StatDistribution::default());
        });
    });
}

/// Marker for stat list text
#[derive(Component)]
pub struct StatListText;

/// Spawn skill tree panel with node graph
fn spawn_skill_tree_panel(
    parent: &mut ChildBuilder,
    active_tab: &ActiveClassTab,
    class_progression: &ClassProgression,
) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(15.0)),
            border: UiRect::all(Val::Px(2.0)),
            overflow: Overflow::clip(),
            ..default()
        },
        background_color: Color::srgb(0.08, 0.08, 0.12).into(),
        border_color: Color::srgb(0.3, 0.3, 0.4).into(),
        ..default()
    }).with_children(|tree_panel| {
        // Class name
        tree_panel.spawn(TextBundle::from_section(
            format!("{} TREE", active_tab.0.name().to_uppercase()),
            TextStyle {
                font_size: 28.0,
                color: active_tab.0.primary_color(),
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        }));
        
        // Scrollable node container
        tree_panel.spawn((
            NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip_y(),
                    max_height: Val::Percent(100.0),
                    position_type: PositionType::Relative,
                    ..default()
                },
                ..default()
            },
            SkillTreeContent,
        )).with_children(|content| {
            // Inner content wrapper that will be scrolled
            content.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        margin: UiRect::top(Val::Px(0.0)),
                        ..default()
                    },
                    ..default()
                },
                SkillTreeInnerContent,
            )).with_children(|inner| {
                spawn_skill_nodes(inner, active_tab.0, class_progression);
            });
        });
    });
}

/// Marker for scrollable skill tree content
#[derive(Component)]
pub struct SkillTreeContent;

/// Marker for inner content that scrolls
#[derive(Component)]
pub struct SkillTreeInnerContent;

/// Spawn skill nodes for a class
fn spawn_skill_nodes(
    parent: &mut ChildBuilder,
    class: ShipClass,
    _class_progression: &ClassProgression,
) {
    // Get all upgrades for this class
    let upgrades: Vec<UpgradeType> = get_upgrades_for_class(class);
    
    // Group by tier
    let mut tiers: Vec<Vec<UpgradeType>> = vec![Vec::new(); 7]; // Tiers 0-6
    for upgrade in upgrades {
        let tier = upgrade.tier() as usize;
        if tier < tiers.len() {
            tiers[tier].push(upgrade);
        }
    }
    
    // Display tier by tier
    for (tier_num, tier_upgrades) in tiers.iter().enumerate().skip(1) {
        if tier_upgrades.is_empty() {
            continue;
        }
        
        // Tier header
        parent.spawn(TextBundle::from_section(
            format!("Tier {} - {} Nodes", tier_num, tier_upgrades.len()),
            TextStyle {
                font_size: 20.0,
                color: Color::srgb(0.7, 0.8, 0.9),
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(15.0), Val::Px(10.0)),
            ..default()
        }));
        
        // Nodes in this tier
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                column_gap: Val::Px(10.0),
                row_gap: Val::Px(10.0),
                margin: UiRect::bottom(Val::Px(25.0)),
                ..default()
            },
            ..default()
        }).with_children(|tier_container| {
            for upgrade in tier_upgrades {
                spawn_skill_node_button(tier_container, *upgrade);
            }
        });
    }
}

/// Spawn individual skill node button
fn spawn_skill_node_button(parent: &mut ChildBuilder, upgrade_type: UpgradeType) {
    let node_size = 120.0;
    
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(node_size),
                height: Val::Px(node_size),
                padding: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgb(0.15, 0.15, 0.2).into(),
            border_color: Color::srgb(0.3, 0.3, 0.4).into(),
            ..default()
        },
        SkillNode { upgrade_type },
    )).with_children(|button| {
        // Node name
        button.spawn(TextBundle::from_section(
            upgrade_type.name(),
            TextStyle {
                font_size: 12.0,
                color: Color::WHITE,
                ..default()
            },
        ).with_text_justify(JustifyText::Center));
        
        // Tier indicator
        button.spawn(TextBundle::from_section(
            format!("T{}", upgrade_type.tier()),
            TextStyle {
                font_size: 10.0,
                color: Color::srgb(0.6, 0.6, 0.6),
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::top(Val::Px(5.0)),
            ..default()
        }));
    });
}

/// Get all upgrades for a specific class
fn get_upgrades_for_class(class: ShipClass) -> Vec<UpgradeType> {
    use UpgradeType::*;
    
    match class {
        ShipClass::Fighter => vec![
            FighterEngineBoost1, FighterEngineBoost2, FighterStrafeSpeed1,
            FighterStrafeSpeed2, FighterRollRate1, FighterAfterburner,
            FighterDriftManeuvers, FighterInertiaDampeners1, FighterInertiaDampeners2,
            FighterThrustVectoring, FighterBoostDuration, FighterQuickTurn,
            FighterVectoredThrusters, FighterAdvancedFlightComputer, FighterBoostRecharge,
            FighterEvasiveManeuvers, FighterSpeedDemon, FighterAgilityMaster,
            FighterMomentumControl, FighterPrecisionFlying, FighterEmergencySpeed,
            FighterCombatAgility, FighterEvasionMatrix, FighterDodgeRoll,
            FighterAfterburnerOverdrive, FighterInstantAcceleration, FighterZeroGManeuvers,
            FighterBarrelRoll, FighterMasterPilot, FighterQuantumEngines,
            FighterPerfectManeuverability, FighterSpeedOfLight, FighterUntouchable,
            FighterAceManeuvers, FighterSupersonicBoost, FighterApexFighter,
        ],
        ShipClass::Tank => vec![
            TankHullPlating1, TankHullPlating2, TankHullPlating3,
            TankShieldCapacity1, TankShieldCapacity2, TankArmorThick1,
            TankReinforcedFrame, TankShieldHardening, TankDamageReduction1,
            TankDamageReduction2, TankShieldBooster, TankHullRepair,
            TankStructuralIntegrity, TankEnergyShields, TankReactiveArmor,
            TankShieldRegeneration, TankAblativeCoating, TankHardpoints,
            TankCompositeArmor, TankShieldOverdrive, TankDamageAbsorption,
            TankLastStand, TankFortressModePassive, TankEmergencyShields,
            TankHeavyArmor, TankShieldReflection, TankImpenetrableHull,
            TankAdaptiveArmor, TankShieldCapacity3, TankBulwark,
            TankBastionProtocols, TankShieldOvercharge, TankUltimateArmor,
            TankIndestructible, TankShieldBarrier, TankPerfectDefense,
            TankIronWill, TankJuggernaut,
        ],
        ShipClass::Gunner => vec![
            GunnerWeaponDamage1, GunnerWeaponDamage2, GunnerWeaponDamage3,
            GunnerFireRate1, GunnerFireRate2, GunnerAmmoCapacity1,
            GunnerWeaponHeat1, GunnerMultiTargeting, GunnerWeaponCooling1,
            GunnerWeaponCooling2, GunnerAmmoCapacity2, GunnerReloadSpeed,
            GunnerAccuracy, GunnerPenetration1, GunnerSplashDamage,
            GunnerPlasmaWeapons, GunnerRailgunUnlock, GunnerCriticalHits1,
            GunnerCriticalHits2, GunnerAmmoEfficiency, GunnerWeaponStabilization,
            GunnerArmorPiercing, GunnerExplosiveRounds, GunnerWeaponOvercharge,
            GunnerDualWielding, GunnerPenetratingRounds, GunnerRapidFire,
            GunnerCriticalDamage, GunnerWeaponSynergy, GunnerBurstFire,
            GunnerSuppressiveFire, GunnerMasterGunner, GunnerChainLightning,
            GunnerDevastationPassive, GunnerPerfectAccuracy, GunnerObliterate,
            GunnerWeaponMastery, GunnerInfiniteAmmo, GunnerArsenalMaster,
        ],
        ShipClass::Stealth => vec![
            StealthSignatureReduction1, StealthSignatureReduction2, StealthSilentRunning,
            StealthLowProfile, StealthSensorDampening, StealthCloakField1,
            StealthCloakField2, StealthRadarJamming, StealthHeatMasking,
            StealthVisualCamo, StealthECM, StealthGhostProtocols1,
            StealthActiveCamouflage, StealthHeatSink, StealthGhostProtocols2,
            StealthSilentWeapons, StealthCloakDuration, StealthPerfectStealth1,
            StealthSensorGhost, StealthPhantomCloak, StealthPerfectStealth2,
            StealthAmbushTactics, StealthShadowStrike, StealthBackstab,
            StealthInvisibility, StealthCloakRecharge, StealthDeception,
            StealthMimicry, StealthInfiltrator, StealthDecoyProjector,
            StealthPhaseShiftPassive, StealthVanish, StealthAssassin,
            StealthPerfectCamouflage, StealthShadowWalker, StealthPhantom,
        ],
        ShipClass::Sniper => vec![
            SniperRangeExtension1, SniperRangeExtension2, SniperRangeExtension3,
            SniperPrecisionTargeting, SniperScopeEnhancement1, SniperProjectileSpeed,
            SniperScopeEnhancement2, SniperChargeWeapons1, SniperChargeWeapons2,
            SniperLongShot, SniperFocusFire, SniperPerfectAim1,
            SniperRangeCalculator, SniperLongRangeRailgun, SniperHeadhunter,
            SniperSteadyAim, SniperChargedShot, SniperCriticalRange,
            SniperPerfectAim2, SniperOverwatch, SniperLongDistance,
            SniperOneShotOneKill, SniperPerfectAccuracy, SniperArmorPiercing,
            SniperExecutioner, SniperChargedDamage, SniperPatientHunter,
            SniperDeadlyPrecision, SniperMarksman, SniperUltraRange,
            SniperChargedDevastation, SniperTargetLock, SniperPerfectShotPassive,
            SniperSnipeFromAnywhere, SniperInstantKill, SniperGhostShot,
            SniperDeadeye,
        ],
        ShipClass::MissileTanker => vec![
            MissileCapacity1, MissileCapacity2, MissileCapacity3,
            MissileReloadSpeed1, MissileReloadSpeed2, MissileTrackingBasic,
            MissileTrackingSystems, MissileSwarmMissiles, MissileMultiLaunch,
            MissileAmmoReserves, MissileFastReload, MissileLockSpeed,
            MissileVolley, MissileClusterWarheads, MissileHomingAI,
            MissileDamage1, MissileDamage2, MissileProximityFuse,
            MissileMultiTarget, MissileSmartGuidance, MissileBarragePassive,
            MissileBarrage, MissileAOEExplosions, MissileFireAndForget,
            MissileClusterBombs, MissileNuclearWarheads, MissileCarpetBombing,
            MissileSeeker, MissileOverwhelm, MissileTacticalNukes,
            MissileSmartMissiles, MissileDevastatorWarheads, MissileInfiniteMissiles,
            MissileStormPassive, MissileApocalypse, MissileArmageddon,
            MissileSupremacy,
        ],
    }
}

/// Update skill node button states based on purchase status
pub fn update_skill_node_states_system(
    upgrades: Res<PlayerUpgrades>,
    inventory: Res<Inventory>,
    mut node_query: Query<(&SkillNode, &mut BackgroundColor, &mut BorderColor)>,
) {
    if !upgrades.is_changed() && !inventory.is_changed() {
        return;
    }
    
    for (node, mut bg_color, mut border_color) in node_query.iter_mut() {
        let is_purchased = upgrades.has_upgrade(node.upgrade_type);
        let can_afford = inventory.can_afford(&node.upgrade_type.cost());
        let can_purchase = upgrades.can_purchase(node.upgrade_type);
        
        // Update colors based on state
        if is_purchased {
            *bg_color = Color::srgb(0.2, 0.4, 0.2).into(); // Green
            *border_color = Color::srgb(0.4, 0.8, 0.4).into();
        } else if can_purchase && can_afford {
            *bg_color = Color::srgb(0.3, 0.3, 0.2).into(); // Gold
            *border_color = Color::srgb(0.8, 0.8, 0.4).into();
        } else if can_afford {
            *bg_color = Color::srgb(0.2, 0.2, 0.3).into(); // Blue (locked)
            *border_color = Color::srgb(0.4, 0.4, 0.6).into();
        } else {
            *bg_color = Color::srgb(0.15, 0.15, 0.2).into(); // Gray
            *border_color = Color::srgb(0.3, 0.3, 0.4).into();
        }
    }
}

/// Handle class tab clicks
pub fn handle_class_tab_clicks_system(
    mut active_tab: ResMut<ActiveClassTab>,
    mut tab_query: Query<(&Interaction, &ClassTab, &mut BackgroundColor, &mut BorderColor), Changed<Interaction>>,
) {
    for (interaction, tab, mut bg_color, mut border_color) in tab_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                println!("[Skill Tree UI] Switching to {} tree", tab.class.name());
                active_tab.0 = tab.class;
                // UI will update automatically via reactive system
            }
            Interaction::Hovered => {
                if tab.class != active_tab.0 {
                    *bg_color = Color::srgb(0.2, 0.2, 0.3).into();
                }
            }
            Interaction::None => {
                if tab.class == active_tab.0 {
                    *bg_color = Color::srgb(0.3, 0.3, 0.5).into();
                    *border_color = Color::srgb(0.6, 0.8, 1.0).into();
                } else {
                    *bg_color = Color::srgb(0.15, 0.15, 0.2).into();
                    *border_color = Color::srgb(0.3, 0.3, 0.3).into();
                }
            }
        }
    }
}

/// Handle skill node clicks for purchasing
pub fn handle_skill_node_clicks_system(
    mut node_query: Query<(&Interaction, &SkillNode), Changed<Interaction>>,
    mut upgrades: ResMut<PlayerUpgrades>,
    mut inventory: ResMut<Inventory>,
    mut class_progression: ResMut<ClassProgression>,
) {
    for (interaction, node) in node_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            let can_purchase = upgrades.can_purchase(node.upgrade_type);
            let can_afford = inventory.can_afford(&node.upgrade_type.cost());
            
            if can_purchase && can_afford {
                let cost = node.upgrade_type.cost();
                inventory.deduct(&cost);
                upgrades.purchase(node.upgrade_type);
                
                // Track class progression
                class_progression.add_point(node.upgrade_type.class());
                
                println!("[Skill Tree UI] Purchased: {}", node.upgrade_type.name());
            } else if !can_purchase {
                println!("[Skill Tree UI] Cannot purchase - prerequisites not met");
            } else {
                println!("[Skill Tree UI] Cannot afford - need more resources");
            }
        }
    }
}

/// Update stat panel with current ship stats
pub fn update_stat_panel_system(
    upgrades: Res<PlayerUpgrades>,
    player_query: Query<(
        &crate::components::combat::Health,
        &crate::components::combat::Shield,
        &crate::components::combat::Energy,
        &crate::components::ship::Ship,
        &crate::components::ship_classes::ClassBonuses,
    ), With<Player>>,
    mut stat_text_query: Query<&mut Text, With<StatListText>>,
) {
    // Update whenever upgrades change or when player is available
    if let Ok((health, shield, energy, ship, bonuses)) = player_query.get_single() {
        for mut text in stat_text_query.iter_mut() {
            text.sections[0].value = format!(
                "Health: {:.0} / {:.0}\n\
                Shields: {:.0} / {:.0}\n\
                Energy: {:.0} / {:.0}\n\
                Speed: {:.1}\n\
                Turn Rate: {:.1}\n\
                \n\
                Bonuses:\n\
                Damage: +{:.0}%\n\
                Fire Rate: +{:.0}%\n\
                Damage Reduction: {:.0}%\n\
                Evasion: {:.0}%",
                health.current, health.max,
                shield.current, shield.max,
                energy.current, energy.max,
                ship.max_speed,
                ship.turn_rate,
                (bonuses.damage_multiplier - 1.0) * 100.0,
                (bonuses.fire_rate_multiplier - 1.0) * 100.0,
                bonuses.damage_reduction * 100.0,
                bonuses.evasion_chance * 100.0,
            );
        }
    }
}

/// Handle node hover for preview
pub fn handle_node_hover_preview_system(
    mut hovered_node: ResMut<HoveredNode>,
    all_nodes_query: Query<(&Interaction, &SkillNode)>,
) {
    // Find the currently hovered node from all nodes
    let mut currently_hovered: Option<UpgradeType> = None;
    for (interaction, node) in all_nodes_query.iter() {
        if *interaction == Interaction::Hovered {
            currently_hovered = Some(node.upgrade_type);
            break;
        }
    }
    
    // Only update if something changed (to avoid unnecessary updates)
    if currently_hovered != hovered_node.0 {
        hovered_node.0 = currently_hovered;
        if let Some(upgrade) = currently_hovered {
            println!("[Skill Tree UI] Hovering over: {}", upgrade.name());
        }
    }
}

/// Cleanup skill tree UI
pub fn cleanup_skill_tree_ui(
    mut commands: Commands,
    query: Query<Entity, With<SkillTreeRoot>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handle ESC key to close skill tree
pub fn handle_skill_tree_close_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<crate::resources::GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("[Skill Tree UI] Closing skill tree");
        next_state.set(crate::resources::GameState::InGame);
    }
}

/// Rebuild skill tree content when active tab changes
pub fn rebuild_skill_tree_on_tab_change_system(
    mut commands: Commands,
    active_tab: Res<ActiveClassTab>,
    class_progression: Res<ClassProgression>,
    content_query: Query<Entity, With<SkillTreeContent>>,
    mut tree_title_query: Query<&mut Text, Without<StatListText>>,
) {
    if !active_tab.is_changed() {
        return;
    }
    
    println!("[Skill Tree UI] Rebuilding tree for: {}", active_tab.0.name());
    
    // Update tree title if it exists
    for mut text in tree_title_query.iter_mut() {
        if text.sections.len() > 0 && text.sections[0].value.contains("TREE") {
            text.sections[0].value = format!("{} TREE", active_tab.0.name().to_uppercase());
            text.sections[0].style.color = active_tab.0.primary_color();
        }
    }
    
    // Rebuild skill tree content
    if let Ok(content_entity) = content_query.get_single() {
        // Despawn all children of the content entity
        commands.entity(content_entity).despawn_descendants();
        
        // Rebuild with new class nodes
        commands.entity(content_entity).with_children(|content| {
            // Inner content wrapper that will be scrolled
            content.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        margin: UiRect::top(Val::Px(0.0)),
                        ..default()
                    },
                    ..default()
                },
                SkillTreeInnerContent,
            )).with_children(|inner| {
                spawn_skill_nodes(inner, active_tab.0, &class_progression);
            });
        });
    }
}

/// Handle skill tree scrolling with mouse wheel or trackpad
pub fn skill_tree_scroll_system(
    mut scroll_events: EventReader<bevy::input::mouse::MouseWheel>,
    mut scroll_query: Query<&mut Style, With<SkillTreeInnerContent>>,
) {
    let mut scroll_amount = 0.0;
    
    for event in scroll_events.read() {
        // Handle both pixel-based (trackpad) and line-based (mouse wheel) scrolling
        scroll_amount += match event.unit {
            bevy::input::mouse::MouseScrollUnit::Line => event.y * 50.0,
            bevy::input::mouse::MouseScrollUnit::Pixel => event.y,
        };
    }
    
    if scroll_amount != 0.0 {
        for mut style in scroll_query.iter_mut() {
            // Get current top margin or default to 0
            let current_top = match style.margin.top {
                Val::Px(px) => px,
                _ => 0.0,
            };
            
            // Apply scroll (positive = scroll up, negative = scroll down)
            // Allow scrolling up to 3000px of content
            let new_top = (current_top + scroll_amount).clamp(-3000.0, 0.0);
            style.margin.top = Val::Px(new_top);
        }
    }
}

/// Display tooltip for hovered skill node
pub fn display_skill_node_tooltip_system(
    hovered_node: Res<HoveredNode>,
    upgrades: Res<PlayerUpgrades>,
    inventory: Res<Inventory>,
    mut commands: Commands,
    tooltip_query: Query<Entity, With<NodeTooltip>>,
    root_query: Query<Entity, With<SkillTreeRoot>>,
) {
    // Remove existing tooltip
    for entity in tooltip_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Spawn new tooltip if hovering over a node
    if let Some(upgrade_type) = hovered_node.0 {
        if let Ok(root_entity) = root_query.get_single() {
            let is_purchased = upgrades.has_upgrade(upgrade_type);
            let can_afford = inventory.can_afford(&upgrade_type.cost());
            let can_purchase = upgrades.can_purchase(upgrade_type);
            let cost = upgrade_type.cost();
            
            commands.entity(root_entity).with_children(|parent| {
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(20.0),
                            top: Val::Px(100.0),
                            width: Val::Px(400.0),
                            padding: UiRect::all(Val::Px(15.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::srgb(0.1, 0.1, 0.15).into(),
                        border_color: if is_purchased {
                            Color::srgb(0.4, 0.8, 0.4)
                        } else if can_purchase && can_afford {
                            Color::srgb(0.8, 0.8, 0.4)
                        } else {
                            Color::srgb(0.4, 0.4, 0.6)
                        }.into(),
                        ..default()
                    },
                    NodeTooltip { upgrade_type },
                )).with_children(|tooltip| {
                    // Skill name
                    tooltip.spawn(TextBundle::from_section(
                        upgrade_type.name(),
                        TextStyle {
                            font_size: 24.0,
                            color: upgrade_type.class().primary_color(),
                            ..default()
                        },
                    ).with_style(Style {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    }));
                    
                    // Description
                    tooltip.spawn(TextBundle::from_section(
                        upgrade_type.description(),
                        TextStyle {
                            font_size: 16.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ).with_style(Style {
                        margin: UiRect::bottom(Val::Px(15.0)),
                        ..default()
                    }));
                    
                    // Cost section
                    tooltip.spawn(TextBundle::from_section(
                        "Cost:",
                        TextStyle {
                            font_size: 18.0,
                            color: Color::srgb(0.8, 0.8, 0.8),
                            ..default()
                        },
                    ).with_style(Style {
                        margin: UiRect::bottom(Val::Px(5.0)),
                        ..default()
                    }));
                    
                    tooltip.spawn(TextBundle::from_section(
                        format!(
                            "Scrap: {} | Energy: {} | Minerals: {} | Tech: {}",
                            cost.scrap_metal, cost.energy_cores, cost.rare_minerals, cost.tech_components
                        ),
                        TextStyle {
                            font_size: 14.0,
                            color: if can_afford {
                                Color::srgb(0.7, 0.9, 0.7)
                            } else {
                                Color::srgb(0.9, 0.7, 0.7)
                            },
                            ..default()
                        },
                    ).with_style(Style {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    }));
                    
                    // Status
                    let status_text = if is_purchased {
                        "Status: PURCHASED".to_string()
                    } else if can_purchase && can_afford {
                        "Status: AVAILABLE".to_string()
                    } else if !can_purchase {
                        "Status: LOCKED (Prerequisites not met)".to_string()
                    } else {
                        "Status: LOCKED (Cannot afford)".to_string()
                    };
                    
                    let status_color = if is_purchased {
                        Color::srgb(0.4, 0.8, 0.4)
                    } else if can_purchase && can_afford {
                        Color::srgb(0.8, 0.8, 0.4)
                    } else {
                        Color::srgb(0.7, 0.7, 0.7)
                    };
                    
                    tooltip.spawn(TextBundle::from_section(
                        status_text,
                        TextStyle {
                            font_size: 16.0,
                            color: status_color,
                            ..default()
                        },
                    ).with_style(Style {
                        max_width: Val::Px(370.0),
                        ..default()
                    }));
                });
            });
        }
    }
}

