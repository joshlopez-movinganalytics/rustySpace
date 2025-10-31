use bevy::prelude::*;
use crate::components::ship::Player;
use crate::components::combat::{Health, Shield, Energy, WeaponMount};
use crate::components::resources::Inventory;
use crate::components::upgrades::{PlayerUpgrades, UpgradeType, UpgradeCategory};
use crate::resources::{GameState, Galaxy};
use crate::systems::save_load;
use crate::systems::ui_theme::{colors, borders, PanelConfig};
use crate::systems::ui_animations::{PulseAnimation, GlitchEffect};

/// HUD root marker
#[derive(Component)]
pub struct HudRoot;

/// Health bar marker
#[derive(Component)]
pub struct HealthBar;

/// Shield bar marker
#[derive(Component)]
pub struct ShieldBar;

/// Energy bar marker
#[derive(Component)]
pub struct EnergyBar;

/// Resource display marker
#[derive(Component)]
pub struct ResourceDisplay;

/// Weapon display marker
#[derive(Component)]
pub struct WeaponDisplay;

/// Weapon name text marker
#[derive(Component)]
pub struct WeaponNameText;

/// Heat bar marker
#[derive(Component)]
pub struct HeatBar;

/// Ammo text marker
#[derive(Component)]
pub struct AmmoText;

/// Charge bar marker (for plasma charging)
#[derive(Component)]
pub struct ChargeBar;

/// Reload indicator marker
#[derive(Component)]
pub struct ReloadIndicator;

/// Health percentage text marker
#[derive(Component)]
pub struct HealthPercentText;

/// Shield percentage text marker
#[derive(Component)]
pub struct ShieldPercentText;

/// Energy percentage text marker
#[derive(Component)]
pub struct EnergyPercentText;

/// Resource text marker
#[derive(Component)]
pub struct ResourceText {
    pub resource_type: crate::components::resources::ResourceType,
}

/// Upgrade notification marker
#[derive(Component)]
pub struct UpgradeNotification;

/// Upgrade notification pulse animation
#[derive(Component)]
pub struct UpgradeNotificationPulse {
    pub pulse_timer: f32,
}

/// Targeting reticule marker
#[derive(Component)]
pub struct TargetingReticule;

/// Reticule center dot
#[derive(Component)]
pub struct ReticuleCenter;

/// Reticule circle
#[derive(Component)]
pub struct ReticuleCircle;

/// Reticule corner brackets (rotating elements)
#[derive(Component)]
pub struct ReticuleCorner {
    pub corner_index: usize,
}

/// Lead indicator marker (shows where to aim for moving targets)
#[derive(Component)]
pub struct LeadIndicator;

/// FPS display marker
#[derive(Component)]
pub struct FpsText;

/// Frame time display marker
#[derive(Component)]
pub struct FrameTimeText;

/// Performance monitor root
#[derive(Component)]
pub struct PerformanceMonitor;

/// Autofire status text marker
#[derive(Component)]
pub struct AutofireStatusText;

/// Setup targeting reticule - HOLOGRAPHIC CYBERPUNK VERSION
pub fn setup_targeting_reticule(mut commands: Commands) {
    println!("[UI System] Setting up holographic targeting reticule");
    
    // Create reticule as an absolutely positioned container
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            z_index: ZIndex::Global(100),
            background_color: Color::NONE.into(),
            ..default()
        },
        TargetingReticule,
    )).with_children(|parent| {
        // Outer circle with pulsing effect
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    border: UiRect::all(Val::Px(2.0)),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                background_color: Color::NONE.into(),
                border_color: colors::NEON_GREEN.into(),
                border_radius: BorderRadius::all(Val::Px(25.0)),
                ..default()
            },
            ReticuleCircle,
            PulseAnimation::new(2.0, colors::NEON_GREEN).with_range(0.6, 1.0),
        ));
        
        // Inner circle
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                border: UiRect::all(Val::Px(1.0)),
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..default()
            },
            background_color: Color::NONE.into(),
            border_color: Color::srgba(0.0, 1.0, 0.3, 0.4).into(),
            border_radius: BorderRadius::all(Val::Px(15.0)),
            ..default()
        });
        
        // Corner brackets (4 corners)
        let corner_size = 12.0;
        let offset = -6.0;
        let corners = [
            (offset, offset),           // Top-left
            (50.0 - corner_size + 6.0, offset),           // Top-right
            (offset, 50.0 - corner_size + 6.0),           // Bottom-left
            (50.0 - corner_size + 6.0, 50.0 - corner_size + 6.0), // Bottom-right
        ];
        
        for (i, (x, y)) in corners.iter().enumerate() {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(corner_size),
                        height: Val::Px(corner_size),
                        position_type: PositionType::Absolute,
                        left: Val::Px(*x),
                        top: Val::Px(*y),
                        border: UiRect {
                            left: if i % 2 == 0 { Val::Px(2.0) } else { Val::Px(0.0) },
                            right: if i % 2 == 1 { Val::Px(2.0) } else { Val::Px(0.0) },
                            top: if i < 2 { Val::Px(2.0) } else { Val::Px(0.0) },
                            bottom: if i >= 2 { Val::Px(2.0) } else { Val::Px(0.0) },
                        },
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    border_color: colors::NEON_CYAN.into(),
                    ..default()
                },
                ReticuleCorner { corner_index: i },
            ));
        }
        
        // Horizontal line
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(20.0),
                height: Val::Px(2.0),
                position_type: PositionType::Absolute,
                left: Val::Px(15.0),
                top: Val::Px(24.0),
                ..default()
            },
            background_color: colors::NEON_GREEN.into(),
            ..default()
        });
        
        // Vertical line
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(2.0),
                height: Val::Px(20.0),
                position_type: PositionType::Absolute,
                left: Val::Px(24.0),
                top: Val::Px(15.0),
                ..default()
            },
            background_color: colors::NEON_GREEN.into(),
            ..default()
        });
        
        // Center dot (pulsing)
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(6.0),
                    height: Val::Px(6.0),
                    position_type: PositionType::Relative,
                    ..default()
                },
                background_color: colors::NEON_CYAN.into(),
                border_radius: BorderRadius::all(Val::Px(3.0)),
                ..default()
            },
            ReticuleCenter,
            PulseAnimation::new(3.0, colors::NEON_CYAN).with_range(0.7, 1.0),
        ));
    });
    
    // Create lead indicator (separate from main reticule) - Enhanced
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            z_index: ZIndex::Global(99),
            background_color: Color::NONE.into(),
            border_color: colors::NEON_ORANGE.into(),
            visibility: Visibility::Hidden, // Hidden by default
            ..default()
        },
        LeadIndicator,
        PulseAnimation::new(4.0, colors::NEON_ORANGE).with_range(0.5, 1.0),
    ));
}

/// Cleanup targeting reticule
pub fn cleanup_targeting_reticule(
    mut commands: Commands,
    reticule_query: Query<Entity, With<TargetingReticule>>,
    lead_query: Query<Entity, With<LeadIndicator>>,
) {
    let count = reticule_query.iter().count();
    if count > 0 {
        println!("[UI System] Cleaning up {} targeting reticule(s)", count);
    }
    for entity in reticule_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in lead_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Update weapon status HUD system
pub fn update_weapon_hud_system(
    mut commands: Commands,
    player_query: Query<(&WeaponMount, &crate::components::combat::AutofireController), With<Player>>,
    mut weapon_name_query: Query<&mut Text, With<WeaponNameText>>,
    mut heat_bar_query: Query<Entity, With<HeatBar>>,
    mut ammo_text_query: Query<&mut Text, (With<AmmoText>, Without<WeaponNameText>, Without<ReloadIndicator>, Without<AutofireStatusText>)>,
    mut charge_bar_query: Query<Entity, With<ChargeBar>>,
    mut reload_text_query: Query<&mut Text, (With<ReloadIndicator>, Without<AmmoText>, Without<WeaponNameText>, Without<AutofireStatusText>)>,
    mut autofire_text_query: Query<&mut Text, (With<AutofireStatusText>, Without<AmmoText>, Without<WeaponNameText>, Without<ReloadIndicator>)>,
) {
    if let Ok((weapon_mount, autofire)) = player_query.get_single() {
        if let Some(weapon) = weapon_mount.weapons.get(weapon_mount.current_weapon) {
            // Update weapon name
            for mut text in weapon_name_query.iter_mut() {
                let weapon_name = match weapon.weapon_type {
                    crate::components::combat::WeaponType::Laser => "LASER",
                    crate::components::combat::WeaponType::Autocannon => "AUTOCANNON",
                    crate::components::combat::WeaponType::Plasma => "PLASMA",
                    crate::components::combat::WeaponType::Missile => "MISSILE",
                    crate::components::combat::WeaponType::Railgun => "RAILGUN",
                    crate::components::combat::WeaponType::IonCannon => "ION CANNON",
                    crate::components::combat::WeaponType::FlakCannon => "FLAK CANNON",
                    crate::components::combat::WeaponType::BeamLaser => "BEAM LASER",
                };
                text.sections[0].value = format!(">> WEAPON: {}", weapon_name);
            }
            
            // Update heat bar
            for entity in heat_bar_query.iter_mut() {
                let heat_percent = if weapon.max_heat > 0.0 {
                    (weapon.heat / weapon.max_heat).clamp(0.0, 1.0) * 100.0
                } else {
                    0.0
                };
                
                commands.entity(entity).insert(Style {
                    width: Val::Percent(heat_percent),
                    height: Val::Px(15.0),
                    ..default()
                });
            }
            
            // Update ammo text
            for mut text in ammo_text_query.iter_mut() {
                if weapon.max_ammo > 0 {
                    text.sections[0].value = format!("[ AMMO: {}/{} ]", weapon.current_ammo, weapon.reserve_ammo);
                    // Color based on ammo status
                    if weapon.current_ammo == 0 {
                        text.sections[0].style.color = colors::DANGER_COLOR;
                    } else if weapon.current_ammo < weapon.max_ammo / 4 {
                        text.sections[0].style.color = colors::WARNING_COLOR;
                    } else {
                        text.sections[0].style.color = Color::srgb(0.9, 0.9, 1.0);
                    }
                } else {
                    text.sections[0].value = "[ AMMO: ∞ ]".to_string();
                    text.sections[0].style.color = Color::srgb(0.7, 0.7, 0.8);
                }
            }
            
            // Update charge bar (for plasma)
            for entity in charge_bar_query.iter_mut() {
                let charge_percent = if weapon.weapon_type == crate::components::combat::WeaponType::Plasma {
                    (weapon.alt_fire_charge / 2.0).clamp(0.0, 1.0) * 100.0 // Max charge is 2.0 seconds
                } else {
                    0.0
                };
                
                commands.entity(entity).insert(Style {
                    width: Val::Percent(charge_percent),
                    height: Val::Px(15.0),
                    ..default()
                });
            }
            
            // Update reload indicator
            for mut text in reload_text_query.iter_mut() {
                if weapon.is_reloading {
                    let reload_percent = ((weapon.reload_timer / weapon.reload_time) * 100.0).min(100.0);
                    text.sections[0].value = format!("RELOADING... {}%", reload_percent as u32);
                    text.sections[0].style.color = Color::srgb(1.0, 0.8, 0.2);
                } else {
                    text.sections[0].value = "".to_string();
                }
            }
        }
        
        // Update autofire status (outside weapon check since it's independent)
        for mut text in autofire_text_query.iter_mut() {
            if autofire.enabled {
                if autofire.current_target.is_some() {
                    text.sections[0].value = "⟨⟨ AUTOFIRE: LOCKED ⟩⟩".to_string();
                    text.sections[0].style.color = colors::NEON_CYAN;
                } else {
                    text.sections[0].value = "⟨⟨ AUTOFIRE: SEARCHING ⟩⟩".to_string();
                    text.sections[0].style.color = colors::NEON_YELLOW;
                }
            } else {
                text.sections[0].value = "".to_string();
            }
        }
    }
}

/// Update HUD system
pub fn update_hud_system(
    mut commands: Commands,
    player_query: Query<(&Health, &Shield, &Energy, &WeaponMount), With<Player>>,
    inventory: Res<Inventory>,
    hud_query: Query<Entity, With<HudRoot>>,
    health_bar_query: Query<Entity, With<HealthBar>>,
    shield_bar_query: Query<Entity, With<ShieldBar>>,
    energy_bar_query: Query<Entity, With<EnergyBar>>,
    mut health_percent_query: Query<&mut Text, (With<HealthPercentText>, Without<ShieldPercentText>, Without<EnergyPercentText>)>,
    mut shield_percent_query: Query<&mut Text, (With<ShieldPercentText>, Without<HealthPercentText>, Without<EnergyPercentText>)>,
    mut energy_percent_query: Query<&mut Text, (With<EnergyPercentText>, Without<HealthPercentText>, Without<ShieldPercentText>)>,
    mut resource_text_query: Query<(&mut Text, &ResourceText), (Without<HealthPercentText>, Without<ShieldPercentText>, Without<EnergyPercentText>)>,
) {
    // Create HUD if it doesn't exist
    if hud_query.is_empty() {
        setup_hud(&mut commands);
    }
    
    if let Ok((health, shield, energy, _weapon_mount)) = player_query.get_single() {
        // Update health bar
        for entity in health_bar_query.iter() {
            let health_percent = (health.current / health.max).clamp(0.0, 1.0);
            commands.entity(entity).insert(Style {
                width: Val::Percent(health_percent * 100.0),
                height: Val::Percent(100.0),
                ..default()
            });
        }
        
        // Update health percentage text
        for mut text in health_percent_query.iter_mut() {
            let health_percent = ((health.current / health.max).clamp(0.0, 1.0) * 100.0) as u32;
            text.sections[0].value = format!("{}%", health_percent);
        }
        
        // Update shield bar
        for entity in shield_bar_query.iter() {
            let shield_percent = (shield.current / shield.max).clamp(0.0, 1.0);
            commands.entity(entity).insert(Style {
                width: Val::Percent(shield_percent * 100.0),
                height: Val::Percent(100.0),
                ..default()
            });
        }
        
        // Update shield percentage text
        for mut text in shield_percent_query.iter_mut() {
            let shield_percent = ((shield.current / shield.max).clamp(0.0, 1.0) * 100.0) as u32;
            text.sections[0].value = format!("{}%", shield_percent);
        }
        
        // Update energy bar
        for entity in energy_bar_query.iter() {
            let energy_percent = (energy.current / energy.max).clamp(0.0, 1.0);
            commands.entity(entity).insert(Style {
                width: Val::Percent(energy_percent * 100.0),
                height: Val::Percent(100.0),
                ..default()
            });
        }
        
        // Update energy percentage text
        for mut text in energy_percent_query.iter_mut() {
            let energy_percent = ((energy.current / energy.max).clamp(0.0, 1.0) * 100.0) as u32;
            text.sections[0].value = format!("{}%", energy_percent);
        }
        
        if inventory.is_changed() {
            println!("[UI System] Resources - Scrap: {}, Energy Cores: {}, Minerals: {}, Tech: {}",
                inventory.scrap_metal,
                inventory.energy_cores,
                inventory.rare_minerals,
                inventory.tech_components
            );
        }
    }
    
    // Update resource text displays
    for (mut text, resource_text) in resource_text_query.iter_mut() {
        use crate::components::resources::ResourceType;
        let (icon, name, value) = match resource_text.resource_type {
            ResourceType::ScrapMetal => ("[■]", "SCRAP", inventory.scrap_metal),
            ResourceType::EnergyCores => ("[●]", "CORES", inventory.energy_cores),
            ResourceType::RareMinerals => ("[◆]", "MINERALS", inventory.rare_minerals),
            ResourceType::TechComponents => ("[▲]", "TECH", inventory.tech_components),
        };
        text.sections[0].value = format!("{} {}: {}", icon, name, value);
    }
}

/// Setup performance monitor widget at top left
pub fn setup_performance_monitor(mut commands: Commands) {
    println!("[UI System] Setting up performance monitor");
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(20.0),
                    top: Val::Px(20.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(3.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                border_color: colors::NEON_CYAN.into(),
                ..default()
            },
            PerformanceMonitor,
        ))
        .with_children(|parent| {
            // FPS text
            parent.spawn((
                TextBundle::from_section(
                    "FPS: --",
                    TextStyle {
                        font_size: 16.0,
                        color: colors::NEON_CYAN,
                        ..default()
                    },
                ),
                FpsText,
            ));
            
            // Frame time text
            parent.spawn((
                TextBundle::from_section(
                    "MS: --",
                    TextStyle {
                        font_size: 14.0,
                        color: Color::srgb(0.7, 0.7, 0.8),
                        ..default()
                    },
                ),
                FrameTimeText,
            ));
        });
}

/// Update performance monitor with current FPS and frame time
pub fn update_performance_monitor(
    commands: Commands,
    time: Res<Time>,
    monitor_query: Query<Entity, With<PerformanceMonitor>>,
    mut fps_query: Query<&mut Text, (With<FpsText>, Without<FrameTimeText>)>,
    mut frame_time_query: Query<&mut Text, (With<FrameTimeText>, Without<FpsText>)>,
) {
    // Create performance monitor if it doesn't exist
    if monitor_query.is_empty() {
        setup_performance_monitor(commands);
        return;
    }
    
    // Calculate FPS and frame time
    let delta = time.delta_seconds();
    let fps = if delta > 0.0 { 1.0 / delta } else { 0.0 };
    let frame_time_ms = delta * 1000.0;
    
    // Update FPS text with color coding
    for mut text in fps_query.iter_mut() {
        let fps_rounded = fps.round() as u32;
        
        // Color based on FPS performance
        let color = if fps >= 60.0 {
            colors::NEON_CYAN // Good performance
        } else if fps >= 30.0 {
            Color::srgb(1.0, 1.0, 0.0) // Warning - yellow
        } else {
            Color::srgb(1.0, 0.0, 0.0) // Poor performance - red
        };
        
        text.sections[0].value = format!("FPS: {}", fps_rounded);
        text.sections[0].style.color = color;
    }
    
    // Update frame time text
    for mut text in frame_time_query.iter_mut() {
        text.sections[0].value = format!("MS: {:.1}", frame_time_ms);
    }
}

fn setup_hud(commands: &mut Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(20.0),
                    top: Val::Px(100.0), // Moved down to make space for performance monitor
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    ..default()
                },
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            // Health bar container - CYBERPUNK STYLE
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                // Health label with neon glow
                parent.spawn(TextBundle::from_section(
                    "// HULL INTEGRITY",
                    TextStyle {
                        font_size: 14.0,
                        color: colors::HEALTH_COLOR,
                        ..default()
                    },
                ));
                
                // Health bar background - hollow design
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(220.0),
                        height: Val::Px(24.0),
                        border: borders::THICK_BORDER,
                        position_type: PositionType::Relative,
                        ..default()
                    },
                    background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                    border_color: colors::HEALTH_COLOR.into(),
                    border_radius: BorderRadius::all(Val::Px(2.0)),
                    ..default()
                }).with_children(|parent| {
                    // Health bar fill with glow
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: colors::HEALTH_COLOR.into(),
                            ..default()
                        },
                        HealthBar,
                    ));
                    
                    // Percentage text overlay
                    parent.spawn((
                        TextBundle::from_section(
                            "100%",
                            TextStyle {
                                font_size: 14.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ).with_style(Style {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(50.0),
                            top: Val::Percent(50.0),
                            ..default()
                        }),
                        HealthPercentText,
                    ));
                });
            });
            
            // Shield bar container - CYBERPUNK STYLE
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                // Shield label with neon glow
                parent.spawn(TextBundle::from_section(
                    "// SHIELD MATRIX",
                    TextStyle {
                        font_size: 14.0,
                        color: colors::SHIELD_COLOR,
                        ..default()
                    },
                ));
                
                // Shield bar background - hollow design with pulse
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(220.0),
                            height: Val::Px(24.0),
                            border: borders::THICK_BORDER,
                            position_type: PositionType::Relative,
                            ..default()
                        },
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                        border_color: colors::SHIELD_COLOR.into(),
                        border_radius: BorderRadius::all(Val::Px(2.0)),
                        ..default()
                    },
                    PulseAnimation::new(1.5, colors::SHIELD_COLOR).with_range(0.7, 1.0),
                )).with_children(|parent| {
                    // Shield bar fill with glow
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: colors::SHIELD_COLOR.into(),
                            ..default()
                        },
                        ShieldBar,
                    ));
                    
                    // Percentage text overlay
                    parent.spawn((
                        TextBundle::from_section(
                            "100%",
                            TextStyle {
                                font_size: 14.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ).with_style(Style {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(50.0),
                            top: Val::Percent(50.0),
                            ..default()
                        }),
                        ShieldPercentText,
                    ));
                });
            });
            
            // Energy bar container - CYBERPUNK STYLE
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                // Energy label with neon glow
                parent.spawn(TextBundle::from_section(
                    "// POWER CORE",
                    TextStyle {
                        font_size: 14.0,
                        color: colors::ENERGY_COLOR,
                        ..default()
                    },
                ));
                
                // Energy bar background - hollow design
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(220.0),
                        height: Val::Px(24.0),
                        border: borders::THICK_BORDER,
                        position_type: PositionType::Relative,
                        ..default()
                    },
                    background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                    border_color: colors::ENERGY_COLOR.into(),
                    border_radius: BorderRadius::all(Val::Px(2.0)),
                    ..default()
                }).with_children(|parent| {
                    // Energy bar fill with glow
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: colors::ENERGY_COLOR.into(),
                            ..default()
                        },
                        EnergyBar,
                    ));
                    
                    // Percentage text overlay
                    parent.spawn((
                        TextBundle::from_section(
                            "100%",
                            TextStyle {
                                font_size: 14.0,
                                color: Color::srgb(0.0, 0.0, 0.0),
                                ..default()
                            },
                        ).with_style(Style {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(50.0),
                            top: Val::Percent(50.0),
                            ..default()
                        }),
                        EnergyPercentText,
                    ));
                });
            });
            
            // Weapon status section - CYBERPUNK STYLE
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                // Weapon name with glitch effect
                parent.spawn((
                    TextBundle::from_section(
                        ">> WEAPON: LASER",
                        TextStyle {
                            font_size: 15.0,
                            color: colors::NEON_ORANGE,
                            ..default()
                        },
                    ),
                    WeaponNameText,
                    GlitchEffect {
                        interval: 5.0,
                        duration: 0.08,
                        intensity: 0.5,
                        ..default()
                    },
                ));
                
                // Heat bar (for lasers) - CYBERPUNK
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(3.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "// HEAT LEVEL",
                        TextStyle {
                            font_size: 11.0,
                            color: Color::srgb(0.7, 0.7, 0.8),
                            ..default()
                        },
                    ));
                    
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(220.0),
                            height: Val::Px(16.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                        border_color: colors::NEON_ORANGE.into(),
                        border_radius: BorderRadius::all(Val::Px(2.0)),
                        ..default()
                    }).with_children(|parent| {
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(0.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                background_color: colors::NEON_ORANGE.into(),
                                ..default()
                            },
                            HeatBar,
                        ));
                    });
                });
                
                // Ammo display - CYBERPUNK
                parent.spawn((
                    TextBundle::from_section(
                        "[ AMMO: --/-- ]",
                        TextStyle {
                            font_size: 13.0,
                            color: Color::srgb(0.9, 0.9, 1.0),
                            ..default()
                        },
                    ),
                    AmmoText,
                ));
                
                // Charge bar (for plasma) - CYBERPUNK
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(3.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "// CHARGE STATUS",
                        TextStyle {
                            font_size: 11.0,
                            color: Color::srgb(0.7, 0.7, 0.8),
                            ..default()
                        },
                    ));
                    
                    parent.spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(220.0),
                            height: Val::Px(16.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                        border_color: colors::NEON_GREEN.into(),
                        border_radius: BorderRadius::all(Val::Px(2.0)),
                        ..default()
                    }).with_children(|parent| {
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(0.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                background_color: colors::NEON_GREEN.into(),
                                ..default()
                            },
                            ChargeBar,
                        ));
                    });
                });
                
                // Reload indicator - CYBERPUNK
                parent.spawn((
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font_size: 13.0,
                            color: colors::NEON_YELLOW,
                            ..default()
                        },
                    ),
                    ReloadIndicator,
                ));
                
                // Autofire status indicator - CYBERPUNK
                parent.spawn((
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font_size: 13.0,
                            color: colors::NEON_CYAN,
                            ..default()
                        },
                    ),
                    AutofireStatusText,
                    PulseAnimation::new(1.0, colors::NEON_CYAN).with_range(0.8, 1.0),
                ));
            });
            
            // Resource display section - CYBERPUNK CHIPS
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(6.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                use crate::components::resources::ResourceType;
                
                parent.spawn(TextBundle::from_section(
                    "// INVENTORY",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.6, 0.7, 0.8),
                        ..default()
                    },
                ));
                
                // Scrap Metal - CYBERPUNK CHIP
                parent.spawn((
                    TextBundle::from_section(
                        "[■] SCRAP: 0",
                        TextStyle {
                            font_size: 13.0,
                            color: colors::SCRAP_COLOR,
                            ..default()
                        },
                    ),
                    ResourceText { resource_type: ResourceType::ScrapMetal },
                ));
                
                // Energy Cores - CYBERPUNK CHIP
                parent.spawn((
                    TextBundle::from_section(
                        "[●] CORES: 0",
                        TextStyle {
                            font_size: 13.0,
                            color: colors::ENERGY_CORE_COLOR,
                            ..default()
                        },
                    ),
                    ResourceText { resource_type: ResourceType::EnergyCores },
                ));
                
                // Rare Minerals - CYBERPUNK CHIP
                parent.spawn((
                    TextBundle::from_section(
                        "[◆] MINERALS: 0",
                        TextStyle {
                            font_size: 13.0,
                            color: colors::MINERAL_COLOR,
                            ..default()
                        },
                    ),
                    ResourceText { resource_type: ResourceType::RareMinerals },
                ));
                
                // Tech Components - CYBERPUNK CHIP
                parent.spawn((
                    TextBundle::from_section(
                        "[▲] TECH: 0",
                        TextStyle {
                            font_size: 13.0,
                            color: colors::TECH_COLOR,
                            ..default()
                        },
                    ),
                    ResourceText { resource_type: ResourceType::TechComponents },
                ));
            });
            
            // Upgrade notification - CYBERPUNK with PULSE
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        ">> UPGRADES AVAILABLE (U)",
                        TextStyle {
                            font_size: 15.0,
                            color: colors::NEON_YELLOW,
                            ..default()
                        },
                    ),
                    style: Style {
                        margin: UiRect::top(Val::Px(25.0)),
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                },
                UpgradeNotification,
                UpgradeNotificationPulse { pulse_timer: 0.0 },
            ));
        });
}

/// Main menu marker
#[derive(Component)]
pub struct MainMenuRoot;

/// Setup main menu - CYBERPUNK REDESIGN
pub fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::srgb(0.02, 0.0, 0.08).into(),
                ..default()
            },
            MainMenuRoot,
        ))
        .with_children(|parent| {
            // Cyberpunk title with glitch effect
            parent.spawn((
                TextBundle::from_section(
                    "◢ SPACE COMBAT ◣",
                    TextStyle {
                        font_size: 80.0,
                        color: colors::NEON_CYAN,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
                GlitchEffect {
                    interval: 2.0,
                    duration: 0.15,
                    intensity: 1.0,
                    ..default()
                },
            ));
            
            // Subtitle with neon glow
            parent.spawn((
                TextBundle::from_section(
                    "// NEURAL INTERFACE ACTIVE",
                    TextStyle {
                        font_size: 22.0,
                        color: colors::NEON_MAGENTA,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                }),
                PulseAnimation::new(1.0, colors::NEON_MAGENTA).with_range(0.6, 1.0),
            ));
            
            // Start prompt with pulsing animation
            parent.spawn((
                TextBundle::from_section(
                    ">> PRESS [ENTER] TO INITIALIZE <<",
                    TextStyle {
                        font_size: 28.0,
                        color: colors::NEON_GREEN,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(30.0)),
                    ..default()
                }),
                PulseAnimation::new(2.0, colors::NEON_GREEN).with_range(0.5, 1.0),
            ));
            
            // Controls panel - Holographic style
            parent.spawn((
                PanelConfig::new()
                    .with_width(Val::Px(700.0))
                    .with_padding(UiRect::all(Val::Px(25.0)))
                    .with_border_color(colors::ELECTRIC_PURPLE)
                    .dark()
                    .build(),
                PulseAnimation::new(0.8, colors::ELECTRIC_PURPLE).with_range(0.8, 1.0),
            )).with_children(|panel| {
                panel.spawn(TextBundle::from_section(
                    "// CONTROL MATRIX //",
                    TextStyle {
                        font_size: 20.0,
                        color: colors::ELECTRIC_PURPLE,
                        ..default()
                    },
                ).with_style(Style {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                }));
                
                panel.spawn(
                    TextBundle::from_section(
                        "MOVEMENT    > WASD\nROTATION    > Arrow Keys\nALTITUDE    > Space / Ctrl\nFIRE        > Mouse / Click\nROLL        > Q / E\nBOOST       > Shift\nWEAPONS     > 1 / 2 / 3\nUPGRADES    > U\nGALAXY MAP  > M",
                        TextStyle {
                            font_size: 16.0,
                            color: Color::srgb(0.7, 0.8, 0.9),
                            ..default()
                        },
                    )
                    .with_style(Style {
                        ..default()
                    }),
                );
            });
            
            // Version/credits in corner
            parent.spawn(
                TextBundle::from_section(
                    "v1.0.0 // NEURAL SYSTEMS ONLINE",
                    TextStyle {
                        font_size: 12.0,
                        color: Color::srgb(0.4, 0.5, 0.6),
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    right: Val::Px(20.0),
                    ..default()
                }),
            );
        });
}

/// Cleanup main menu
pub fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Main menu system
pub fn main_menu_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    player_query: Query<Entity, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        println!("[UI System] Starting game...");
        
        // If no player exists (e.g., after dying and returning to menu), restart the game
        if player_query.is_empty() {
            println!("[UI System] No player found, setting restart flag");
            commands.insert_resource(RestartGameFlag);
        }
        
        next_state.set(GameState::InGame);
    }
}

/// Check for upgrade key press
pub fn check_upgrade_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyU) {
        println!("[UI System] Opening upgrade menu...");
        next_state.set(GameState::Upgrade);
    }
}

/// Check for galaxy map key press
pub fn check_galaxy_map_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        println!("[UI System] Opening galaxy map...");
        next_state.set(GameState::GalaxyMap);
    }
}

/// Upgrade menu marker
#[derive(Component)]
pub struct UpgradeMenuRoot;

/// Scrollable upgrade container marker (the outer wrapper that clips)
#[derive(Component)]
pub struct UpgradeScrollContainer;

/// Scrollable upgrade content (the inner content that moves)
#[derive(Component)]
pub struct UpgradeScrollContent;

/// Upgrade button marker
#[derive(Component)]
pub struct UpgradeButton {
    pub upgrade_type: UpgradeType,
    pub index: usize,
}

/// Marker for upgrade button text
#[derive(Component)]
pub struct UpgradeButtonText;

/// Setup upgrade menu
pub fn setup_upgrade_menu(
    mut commands: Commands,
    inventory: Res<Inventory>,
    upgrades: Res<PlayerUpgrades>,
) {
    let mut upgrade_index = 0;
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(20.0)),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                background_color: Color::srgb(0.0, 0.0, 0.1).into(),
                ..default()
            },
            UpgradeMenuRoot,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(
                TextBundle::from_section(
                    "SHIP UPGRADES",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                }),
            );
            
            // Resources display
            parent.spawn((
                TextBundle::from_section(
                    format!(
                        "Resources: Scrap: {} | Energy: {} | Minerals: {} | Tech: {}",
                        inventory.scrap_metal,
                        inventory.energy_cores,
                        inventory.rare_minerals,
                        inventory.tech_components
                    ),
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(0.8, 0.8, 0.8),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                }),
                ResourceDisplay,
            ));
            
            // Scrollable container wrapper (clips overflow)
            parent.spawn((
                NodeBundle {
                    style: Style {
                        overflow: Overflow::clip_y(),
                        max_height: Val::Vh(65.0),
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                UpgradeScrollContainer,
            )).with_children(|clip_parent| {
                // Scrollable content (this actually moves)
                clip_parent.spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    },
                    UpgradeScrollContent,
                )).with_children(|scroll_parent| {
            // Categories
            for category in [
                UpgradeCategory::Hull,
                UpgradeCategory::Shields,
                UpgradeCategory::Engines,
                UpgradeCategory::PowerPlant,
                UpgradeCategory::Weapons,
            ] {
                scroll_parent.spawn(
                    TextBundle::from_section(
                        format!("{:?} Upgrades", category),
                        TextStyle {
                            font_size: 25.0,
                            color: Color::srgb(1.0, 0.8, 0.2),
                            ..default()
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::vertical(Val::Px(10.0)),
                        ..default()
                    }),
                );
                
                // Show upgrades for this category as buttons
                let upgrades_in_category = get_upgrades_for_category(category);
                for upgrade_type in upgrades_in_category {
                    let purchased = upgrades.has_upgrade(upgrade_type);
                    let can_purchase = upgrades.can_purchase(upgrade_type) 
                        && inventory.can_afford(&upgrade_type.cost());
                    
                    let bg_color = if purchased {
                        Color::srgb(0.1, 0.3, 0.1)
                    } else if can_purchase {
                        Color::srgb(0.2, 0.2, 0.3)
                    } else {
                        Color::srgb(0.15, 0.15, 0.15)
                    };
                    
                    let text_color = if purchased {
                        Color::srgb(0.2, 0.8, 0.2)
                    } else if can_purchase {
                        Color::srgb(1.0, 1.0, 1.0)
                    } else {
                        Color::srgb(0.4, 0.4, 0.4)
                    };
                    
                    let status = if purchased {
                        "[PURCHASED]"
                    } else if !upgrades.can_purchase(upgrade_type) {
                        "[LOCKED - Missing Prerequisites]"
                    } else if !inventory.can_afford(&upgrade_type.cost()) {
                        "[INSUFFICIENT RESOURCES]"
                    } else {
                        ""
                    };
                    
                    let cost = upgrade_type.cost();
                    let button_text = if status.is_empty() {
                            format!(
                            "[{}] {} - {} (Cost: S:{} E:{} M:{} T:{})",
                            upgrade_index + 1,
                            upgrade_type.name(),
                            upgrade_type.description(),
                            cost.scrap_metal,
                            cost.energy_cores,
                            cost.rare_minerals,
                            cost.tech_components
                        )
                    } else {
                        format!(
                            "[{}] {} - {} {} (Cost: S:{} E:{} M:{} T:{})",
                            upgrade_index + 1,
                                upgrade_type.name(),
                                upgrade_type.description(),
                                status,
                                cost.scrap_metal,
                                cost.energy_cores,
                                cost.rare_minerals,
                                cost.tech_components
                        )
                    };
                    
                    scroll_parent.spawn((
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(10.0)),
                                margin: UiRect::new(Val::Px(20.0), Val::Px(10.0), Val::Px(5.0), Val::Px(5.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            background_color: bg_color.into(),
                            border_color: if can_purchase && !purchased {
                                Color::srgb(0.4, 0.6, 1.0).into()
                            } else {
                                Color::srgb(0.3, 0.3, 0.3).into()
                            },
                            ..default()
                        },
                        UpgradeButton {
                            upgrade_type,
                            index: upgrade_index,
                        },
                    )).with_children(|button_parent| {
                        button_parent.spawn((
                            TextBundle::from_section(
                                button_text,
                            TextStyle {
                                font_size: 16.0,
                                    color: text_color,
                                ..default()
                            },
                            ),
                            UpgradeButtonText,
                        ));
                    });
                    
                    upgrade_index += 1;
                }
            }
                }); // End scroll_parent (UpgradeScrollContent)
            }); // End clip_parent (UpgradeScrollContainer)
            
            // Instructions (outside scroll area, always visible)
            parent.spawn(
                TextBundle::from_section(
                    "Click buttons or press number keys (1-9) to purchase upgrades | ESC to return | Scroll with mouse wheel",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb(0.6, 0.6, 0.6),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(Val::Px(15.0)),
                    ..default()
                }),
            );
        });
}

/// Cleanup upgrade menu
pub fn cleanup_upgrade_menu(
    mut commands: Commands,
    query: Query<Entity, With<UpgradeMenuRoot>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handle upgrade menu scrolling with mouse wheel or trackpad
pub fn upgrade_menu_scroll_system(
    mut scroll_events: EventReader<bevy::input::mouse::MouseWheel>,
    mut scroll_query: Query<&mut Style, With<UpgradeScrollContent>>,
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
            let new_top = (current_top + scroll_amount).clamp(-2000.0, 0.0);
            style.margin.top = Val::Px(new_top);
        }
    }
}

/// Upgrade menu system - handles button interactions and keyboard shortcuts
pub fn upgrade_menu_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut inventory: ResMut<Inventory>,
    mut upgrades: ResMut<PlayerUpgrades>,
    mut button_query: Query<
        (&Interaction, &UpgradeButton, &mut BackgroundColor, &mut BorderColor),
        Changed<Interaction>,
    >,
    all_buttons_query: Query<&UpgradeButton>,
    menu_root: Query<Entity, With<UpgradeMenuRoot>>,
) {
    // Check for ESC to close menu
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("[UI System] Closing upgrade menu...");
        next_state.set(GameState::InGame);
        return;
    }
    
    let mut purchased_upgrade: Option<UpgradeType> = None;
    
    // Handle button clicks
    for (interaction, button, mut bg_color, mut border_color) in button_query.iter_mut() {
        let can_purchase = upgrades.can_purchase(button.upgrade_type) 
            && inventory.can_afford(&button.upgrade_type.cost());
        let already_purchased = upgrades.has_upgrade(button.upgrade_type);
        
        match *interaction {
            Interaction::Pressed => {
                if can_purchase && !already_purchased {
                    purchased_upgrade = Some(button.upgrade_type);
                    println!("[UI System] Purchasing upgrade via click: {:?}", button.upgrade_type);
                } else if already_purchased {
                    println!("[UI System] Upgrade already purchased: {:?}", button.upgrade_type);
                } else {
                    println!("[UI System] Cannot purchase upgrade: {:?}", button.upgrade_type);
                }
            }
            Interaction::Hovered => {
                if can_purchase && !already_purchased {
                    *bg_color = Color::srgb(0.3, 0.3, 0.5).into();
                    *border_color = Color::srgb(0.6, 0.8, 1.0).into();
                }
            }
            Interaction::None => {
                let normal_bg = if already_purchased {
                    Color::srgb(0.1, 0.3, 0.1)
                } else if can_purchase {
                    Color::srgb(0.2, 0.2, 0.3)
                } else {
                    Color::srgb(0.15, 0.15, 0.15)
                };
                
                let normal_border = if can_purchase && !already_purchased {
                    Color::srgb(0.4, 0.6, 1.0)
                } else {
                    Color::srgb(0.3, 0.3, 0.3)
                };
                
                *bg_color = normal_bg.into();
                *border_color = normal_border.into();
            }
        }
    }
    
    // Handle number key shortcuts (1-9)
    let number_keys = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ];
    
    for (i, key) in number_keys.iter().enumerate() {
        if keyboard.just_pressed(*key) {
            // Find the button with this index
            for button in all_buttons_query.iter() {
                if button.index == i {
                    let can_purchase = upgrades.can_purchase(button.upgrade_type) 
                        && inventory.can_afford(&button.upgrade_type.cost());
                    let already_purchased = upgrades.has_upgrade(button.upgrade_type);
                    
                    if can_purchase && !already_purchased {
                        purchased_upgrade = Some(button.upgrade_type);
                        println!("[UI System] Purchasing upgrade via key {}: {:?}", i + 1, button.upgrade_type);
                    } else if already_purchased {
                        println!("[UI System] Upgrade already purchased: {:?}", button.upgrade_type);
                    } else {
                        println!("[UI System] Cannot purchase upgrade: {:?}", button.upgrade_type);
                    }
                    break;
                }
            }
        }
    }
    
    // Execute purchase if one was selected
    if let Some(upgrade_type) = purchased_upgrade {
        let cost = upgrade_type.cost();
        inventory.deduct(&cost);
        upgrades.purchase(upgrade_type);
        
        println!("[UI System] Successfully purchased: {:?}", upgrade_type);
        println!("[UI System] Remaining resources - S:{} E:{} M:{} T:{}",
            inventory.scrap_metal,
            inventory.energy_cores,
            inventory.rare_minerals,
            inventory.tech_components
        );
        
        // Refresh the menu to show updated state
        if let Ok(root_entity) = menu_root.get_single() {
            commands.entity(root_entity).despawn_recursive();
        }
        setup_upgrade_menu(commands, inventory.into(), upgrades.into());
    }
}

/// Apply purchased upgrades to player ship (New skill tree system)
/// Runs in both InGame and Upgrade states to keep stats updated
pub fn apply_upgrades_to_player(
    upgrades: Res<PlayerUpgrades>,
    mut player_query: Query<
        (&mut Health, &mut Shield, &mut Energy, &mut crate::components::ship::Ship, &mut WeaponMount, &mut crate::components::ship_classes::ClassBonuses, &mut crate::components::abilities::AbilityController),
        With<Player>,
    >,
) {
    // Always recalculate bonuses when upgrades change
    // This ensures stats panel shows correct values even when in Upgrade state
    // Note: We still need to check if player exists, but we want to recalculate whenever upgrades change
    if !upgrades.is_changed() {
        return;
    }
    
    if let Ok((mut health, mut shield, mut energy, mut ship, mut weapon_mount, mut bonuses, mut ability_controller)) = player_query.get_single_mut() {
        // Base stats
        let base_health = 100.0;
        let base_shield = 100.0;
        let base_shield_recharge = 10.0;
        let base_energy = 100.0;
        let base_energy_recharge = 20.0;
        let base_speed = 80.0;
        let base_turn_rate = 4.0;
        
        // Reset all bonuses to default values before applying upgrades
        bonuses.health_multiplier = 1.0;
        bonuses.shield_multiplier = 1.0;
        bonuses.speed_multiplier = 1.0;
        bonuses.turn_rate_multiplier = 1.0;
        bonuses.damage_multiplier = 1.0;
        bonuses.fire_rate_multiplier = 1.0;
        bonuses.energy_multiplier = 1.0;
        bonuses.shield_recharge_multiplier = 1.0;
        bonuses.energy_recharge_multiplier = 1.0;
        bonuses.damage_reduction = 0.0;
        bonuses.evasion_chance = 0.0;
        bonuses.critical_chance = 0.0;
        bonuses.critical_multiplier = 1.5;
        bonuses.stealth_level = 0.0;
        bonuses.detection_range_multiplier = 1.0;
        bonuses.projectile_speed_multiplier = 1.0;
        bonuses.missile_count_multiplier = 1.0;
        
        // Apply all upgrades using comprehensive system
        let mut unlocked_count = 0;
        for upgrade in &upgrades.purchased {
            let unlocked = crate::systems::upgrade_effects::apply_upgrade_effect(
                *upgrade,
                &mut bonuses,
                &mut weapon_mount,
            );
            if unlocked {
                unlocked_count += 1;
                println!("[UI System] Unlocked: {}", upgrade.name());
            }
        }
        
        if unlocked_count > 0 {
            println!("[UI System] Unlocked {} new abilities/weapons", unlocked_count);
        }
        
        // Unlock special abilities for capstone upgrades
        for upgrade in &upgrades.purchased {
            if let Some(ability) = upgrade.unlocks_ability() {
                ability_controller.unlock_ability(ability);
            }
        }
        
        // Apply calculated bonuses
        health.max = base_health * bonuses.health_multiplier;
        health.current = health.current.min(health.max);
        
        shield.max = base_shield * bonuses.shield_multiplier;
        shield.current = shield.current.min(shield.max);
        shield.recharge_rate = base_shield_recharge * bonuses.shield_recharge_multiplier;
        
        ship.max_speed = base_speed * bonuses.speed_multiplier;
        ship.turn_rate = base_turn_rate * bonuses.turn_rate_multiplier;
        
        energy.max = base_energy * bonuses.energy_multiplier;
        energy.current = energy.current.min(energy.max);
        energy.recharge_rate = base_energy_recharge * bonuses.energy_recharge_multiplier;
        
        println!("[UI System] Applied {} upgrades to player ship", upgrades.purchased.len());
        println!("[UI System] Damage bonus: +{:.0}%, Fire Rate bonus: +{:.0}%", 
            (bonuses.damage_multiplier - 1.0) * 100.0,
            (bonuses.fire_rate_multiplier - 1.0) * 100.0);
    }
}

fn get_upgrades_for_category(category: UpgradeCategory) -> Vec<UpgradeType> {
    use UpgradeType::*;
    // Temporary mapping to new class-based system - showing a subset for compatibility
    match category {
        UpgradeCategory::Hull => vec![
            TankHullPlating1, TankHullPlating2, TankHullPlating3,
            TankArmorThick1,
        ],
        UpgradeCategory::Shields => vec![
            TankShieldCapacity1, TankShieldCapacity2, TankShieldCapacity3,
            TankShieldHardening, TankShieldRegeneration,
        ],
        UpgradeCategory::Engines => vec![
            FighterEngineBoost1, FighterEngineBoost2,
            FighterStrafeSpeed1, FighterStrafeSpeed2,
        ],
        UpgradeCategory::PowerPlant => vec![
            GunnerWeaponDamage1, GunnerWeaponDamage2,
        ],
        UpgradeCategory::Weapons => vec![
            GunnerWeaponDamage1, GunnerWeaponDamage2, GunnerWeaponDamage3,
            GunnerFireRate1, GunnerFireRate2,
            GunnerPlasmaWeapons, GunnerRailgunUnlock,
        ],
    }
}

/// Check for pause key press
pub fn check_pause_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::InGame => {
                println!("[UI System] Game paused");
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                println!("[UI System] Game resumed");
                next_state.set(GameState::InGame);
            }
            _ => {}
        }
    }
}

/// Pause menu marker
#[derive(Component)]
pub struct PauseMenuRoot;

/// Pause menu button types
#[derive(Component, Clone, Copy)]
pub enum PauseMenuButton {
    Resume,
    Save,
    Exit,
}

/// Setup pause menu - CYBERPUNK REDESIGN
pub fn setup_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::srgba(0.02, 0.0, 0.08, 0.9).into(),
                ..default()
            },
            PauseMenuRoot,
        ))
        .with_children(|parent| {
            // Main pause panel with holographic style
            parent.spawn((
                PanelConfig::new()
                    .with_width(Val::Px(500.0))
                    .with_padding(UiRect::all(Val::Px(40.0)))
                    .with_border_color(colors::NEON_CYAN)
                    .darker()
                    .build(),
                PulseAnimation::new(1.0, colors::NEON_CYAN).with_range(0.7, 1.0),
            )).with_children(|panel| {
                // Paused title with glitch
                panel.spawn((
                    TextBundle::from_section(
                        "// SYSTEM PAUSED //",
                        TextStyle {
                            font_size: 50.0,
                            color: colors::NEON_CYAN,
                            ..default()
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::bottom(Val::Px(40.0)),
                        ..default()
                    }),
                    GlitchEffect {
                        interval: 3.0,
                        duration: 0.1,
                        intensity: 0.7,
                        ..default()
                    },
                ));

                // Resume button
                spawn_cyberpunk_menu_button(panel, ">> RESUME [ESC]", PauseMenuButton::Resume, colors::NEON_GREEN);
                
                // Save button
                spawn_cyberpunk_menu_button(panel, ">> SAVE PROGRESS", PauseMenuButton::Save, colors::NEON_CYAN);
                
                // Exit button
                spawn_cyberpunk_menu_button(panel, ">> EXIT TO MENU", PauseMenuButton::Exit, colors::NEON_ORANGE);
            });
        });
}

/// Cleanup pause menu
pub fn cleanup_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenuRoot>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Pause menu system
pub fn pause_menu_system(
    mut next_state: ResMut<NextState<GameState>>,
    mut button_query: Query<
        (&Interaction, &PauseMenuButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    player_query: Query<(&Transform, &Health, &Shield, &Energy), With<Player>>,
    inventory: Res<Inventory>,
    upgrades: Res<PlayerUpgrades>,
    galaxy: Option<Res<Galaxy>>,
) {
    for (interaction, button_type, mut bg_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    PauseMenuButton::Resume => {
                        println!("[UI System] Resuming game...");
                        next_state.set(GameState::InGame);
                    }
                    PauseMenuButton::Save => {
                        println!("[UI System] Saving game...");
                        match save_load::save_game(&player_query, &inventory, &upgrades, galaxy.as_deref()) {
                            Ok(_) => println!("[UI System] Game saved successfully!"),
                            Err(e) => println!("[UI System] Failed to save game: {}", e),
                        }
                    }
                    PauseMenuButton::Exit => {
                        println!("[UI System] Exiting to main menu...");
                        next_state.set(GameState::MainMenu);
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = Color::srgb(0.4, 0.4, 0.5).into();
            }
            Interaction::None => {
                *bg_color = Color::srgb(0.2, 0.2, 0.3).into();
            }
        }
    }
}

/// Game over menu marker
#[derive(Component)]
pub struct GameOverMenuRoot;

/// Game over button types
#[derive(Component, Clone, Copy)]
pub enum GameOverButton {
    Restart,
    LoadSave,
    MainMenu,
}

/// Setup game over menu - CYBERPUNK REDESIGN
pub fn setup_game_over_menu(mut commands: Commands) {
    let save_exists = save_load::save_exists();
    
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::srgba(0.1, 0.0, 0.05, 0.95).into(),
                ..default()
            },
            GameOverMenuRoot,
        ))
        .with_children(|parent| {
            // Main game over panel
            parent.spawn((
                PanelConfig::new()
                    .with_width(Val::Px(600.0))
                    .with_padding(UiRect::all(Val::Px(50.0)))
                    .with_border_color(colors::DANGER_COLOR)
                    .darker()
                    .build(),
                PulseAnimation::new(1.5, colors::DANGER_COLOR).with_range(0.6, 1.0),
            )).with_children(|panel| {
                // Game over title with glitch effect
                panel.spawn((
                    TextBundle::from_section(
                        "// CRITICAL FAILURE //",
                        TextStyle {
                            font_size: 60.0,
                            color: colors::DANGER_COLOR,
                            ..default()
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::bottom(Val::Px(30.0)),
                        ..default()
                    }),
                    GlitchEffect {
                        interval: 1.0,
                        duration: 0.2,
                        intensity: 1.5,
                        ..default()
                    },
                ));

                // Subtitle
                panel.spawn(
                    TextBundle::from_section(
                        "HULL INTEGRITY: 0%\nSYSTEMS OFFLINE",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::srgb(0.8, 0.5, 0.5),
                            ..default()
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::bottom(Val::Px(40.0)),
                        ..default()
                    })
                    .with_text_justify(JustifyText::Center),
                );

                // Restart button
                spawn_cyberpunk_menu_button(panel, ">> RESPAWN", GameOverButton::Restart, colors::NEON_GREEN);
                
                // Load save button (only if save exists)
                if save_exists {
                    spawn_cyberpunk_menu_button(panel, ">> RESTORE BACKUP", GameOverButton::LoadSave, colors::NEON_CYAN);
                }
                
                // Main menu button
                spawn_cyberpunk_menu_button(panel, ">> EXIT TO MENU", GameOverButton::MainMenu, colors::NEON_ORANGE);
            });
        });
}

/// Cleanup game over menu
pub fn cleanup_game_over_menu(
    mut commands: Commands,
    query: Query<Entity, With<GameOverMenuRoot>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Cleanup HUD when entering game over
pub fn cleanup_hud_on_game_over(
    mut commands: Commands,
    hud_query: Query<Entity, With<HudRoot>>,
) {
    for entity in hud_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Game over menu system
pub fn game_over_menu_system(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut button_query: Query<
        (&Interaction, &GameOverButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, button_type, mut bg_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                match button_type {
                    GameOverButton::Restart => {
                        println!("[UI System] Restarting game...");
                        // The restart will be handled by despawning and respawning the player
                        commands.insert_resource(RestartGameFlag);
                        next_state.set(GameState::InGame);
                    }
                    GameOverButton::LoadSave => {
                        println!("[UI System] Loading saved game...");
                        commands.insert_resource(LoadGameFlag);
                        next_state.set(GameState::InGame);
                    }
                    GameOverButton::MainMenu => {
                        println!("[UI System] Returning to main menu...");
                        next_state.set(GameState::MainMenu);
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = Color::srgb(0.4, 0.4, 0.5).into();
            }
            Interaction::None => {
                *bg_color = Color::srgb(0.2, 0.2, 0.3).into();
            }
        }
    }
}

/// Helper function to spawn a menu button - LEGACY (kept for compatibility)
fn spawn_menu_button<T: Component>(parent: &mut ChildBuilder, text: &str, button_type: T) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                padding: UiRect::all(Val::Px(20.0)),
                margin: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgb(0.2, 0.2, 0.3).into(),
            border_color: Color::srgb(0.4, 0.6, 1.0).into(),
            ..default()
        },
        button_type,
    )).with_children(|button_parent| {
        button_parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 28.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

/// Helper function to spawn a cyberpunk-styled menu button
fn spawn_cyberpunk_menu_button<T: Component>(parent: &mut ChildBuilder, text: &str, button_type: T, border_color: Color) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                padding: UiRect::all(Val::Px(18.0)),
                margin: UiRect::all(Val::Px(12.0)),
                border: borders::THICK_BORDER,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                ..default()
            },
            background_color: colors::BUTTON_BG.into(),
            border_color: border_color.into(),
            border_radius: BorderRadius::all(Val::Px(2.0)),
            ..default()
        },
        button_type,
        PulseAnimation::new(1.5, border_color).with_range(0.7, 1.0),
    )).with_children(|button_parent| {
        button_parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 24.0,
                color: border_color,
                ..default()
            },
        ));
    });
}

/// Flag resource to indicate game restart
#[derive(Resource)]
pub struct RestartGameFlag;

/// Flag resource to indicate game load
#[derive(Resource)]
pub struct LoadGameFlag;

/// Check upgrade availability and show/hide notification
pub fn check_upgrade_availability_system(
    inventory: Res<Inventory>,
    upgrades: Res<PlayerUpgrades>,
    mut notification_query: Query<&mut Visibility, With<UpgradeNotification>>,
) {
    let mut any_affordable = false;
    
    // Check all upgrade categories
    for category in [
        UpgradeCategory::Hull,
        UpgradeCategory::Shields,
        UpgradeCategory::Engines,
        UpgradeCategory::PowerPlant,
        UpgradeCategory::Weapons,
    ] {
        let upgrades_in_category = get_upgrades_for_category(category);
        for upgrade_type in upgrades_in_category {
            if upgrades.can_purchase(upgrade_type) && inventory.can_afford(&upgrade_type.cost()) {
                any_affordable = true;
                break;
            }
        }
        if any_affordable {
            break;
        }
    }
    
    // Show or hide notification
    for mut visibility in notification_query.iter_mut() {
        *visibility = if any_affordable {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Pulse animation for upgrade notification
pub fn update_upgrade_notification_pulse(
    time: Res<Time>,
    mut query: Query<(&mut UpgradeNotificationPulse, &mut Transform), With<UpgradeNotification>>,
) {
    for (mut pulse, mut transform) in query.iter_mut() {
        pulse.pulse_timer += time.delta_seconds();
        
        // Pulse period: 1.5 seconds
        let pulse_value = (pulse.pulse_timer * std::f32::consts::TAU / 1.5).sin();
        // Scale between 1.0 and 1.15
        let scale = 1.0 + pulse_value * 0.075 + 0.075;
        
        transform.scale = Vec3::splat(scale);
    }
}

/// Calculate intercept point for a moving target using analytical solution
/// 
/// This solves the classic interception problem:
/// Given:
///   - P0: Target's current position
///   - V0: Target's velocity vector
///   - P1: Shooter's position (projectile origin)
///   - s1: Projectile speed (scalar, not vector)
/// 
/// Find:
///   - V: The direction vector for the projectile
///   - t: Time of impact
/// 
/// Solution:
/// At time t, projectile position = P1 + V*t where |V| = s1
/// At time t, target position = P0 + V0*t
/// 
/// These must be equal: P1 + V*t = P0 + V0*t
/// Since |V| = s1, we have: |P0 + V0*t - P1| = s1*t
/// 
/// Squaring both sides and solving the quadratic equation gives us t.
fn calculate_intercept_point(
    shooter_pos: Vec3,
    _shooter_velocity: Vec3,  // Not used since projectiles don't inherit momentum
    target_pos: Vec3,
    target_velocity: Vec3,
    projectile_speed: f32,
    _shoot_direction: Vec3,
) -> Option<Vec3> {
    // Safety checks
    if !shooter_pos.is_finite() || !target_pos.is_finite() {
        return None;
    }
    if !target_velocity.is_finite() {
        return None;
    }
    if !projectile_speed.is_finite() || projectile_speed <= 0.0 {
        return None;
    }
    
    // Relative position from shooter to target
    let to_target = target_pos - shooter_pos;
    
    // Solve for time t when projectile intercepts target
    // |to_target + target_velocity * t| = projectile_speed * t
    // Squaring both sides:
    // |to_target|^2 + 2*(to_target · target_velocity)*t + |target_velocity|^2*t^2 = projectile_speed^2 * t^2
    // 
    // Rearranging into standard quadratic form: a*t^2 + b*t + c = 0
    // (|target_velocity|^2 - projectile_speed^2) * t^2 + 2*(to_target · target_velocity) * t + |to_target|^2 = 0
    
    let a = target_velocity.length_squared() - projectile_speed * projectile_speed;
    let b = 2.0 * to_target.dot(target_velocity);
    let c = to_target.length_squared();
    
    // Safety check
    if !a.is_finite() || !b.is_finite() || !c.is_finite() {
        return None;
    }
    
    // Special case: target very close to shooter
    if c < 0.01 {
        return Some(target_pos);
    }
    
    // Solve the quadratic equation
    let time_to_intercept = if a.abs() < 0.0001 {
        // Linear case: target speed ≈ projectile speed
        // Equation becomes: b*t + c = 0
        if b.abs() > 0.0001 {
            let t = -c / b;
            if t > 0.0 { t } else { return None; }
        } else {
            // Target nearly stationary or moving perpendicular
            let distance = to_target.length();
            distance / projectile_speed
        }
    } else {
        // Quadratic case: standard solution
        let discriminant = b * b - 4.0 * a * c;
        
        // Check if intercept is possible
        if discriminant < 0.0 {
            // No real solution - target too fast to intercept
            // Use simple prediction as fallback
            let distance = to_target.length();
            let simple_time = distance / projectile_speed;
            let fallback_pos = target_pos + target_velocity * simple_time;
            return if fallback_pos.is_finite() {
                Some(fallback_pos)
            } else {
                None
            };
        }
        
        // Two solutions - we want the smallest positive one (earliest intercept)
        let sqrt_disc = discriminant.sqrt();
        let t1 = (-b - sqrt_disc) / (2.0 * a);
        let t2 = (-b + sqrt_disc) / (2.0 * a);
        
        // Choose the earliest positive time
        if t1 > 0.0 && (t2 <= 0.0 || t1 < t2) {
            t1
        } else if t2 > 0.0 {
            t2
        } else {
            // Both negative - cannot intercept (target moving away faster than we can catch)
            return None;
        }
    };
    
    // Validate time is reasonable
    if !time_to_intercept.is_finite() || time_to_intercept < 0.0 || time_to_intercept > 100.0 {
        return None;
    }
    
    // Calculate the intercept point
    let intercept_pos = target_pos + target_velocity * time_to_intercept;
    
    // Final validation
    if intercept_pos.is_finite() {
        Some(intercept_pos)
    } else {
        None
    }
}

/// Update targeting reticule position and color based on where bullets will go
/// Now accounts for player velocity, enemy velocity, and projectile speed for accurate prediction
pub fn update_targeting_reticule_system(
    player_query: Query<(&Transform, &WeaponMount, &crate::components::ship::Velocity, &crate::components::ship_classes::ClassBonuses), With<Player>>,
    enemy_query: Query<(&Transform, &crate::components::ship::Velocity), (With<crate::components::ai::Enemy>, Without<Player>)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<crate::components::camera::CameraController>>,
    mut reticule_query: Query<&mut Style, (With<TargetingReticule>, Without<LeadIndicator>)>,
    mut circle_query: Query<&mut BorderColor, (With<ReticuleCircle>, Without<Player>, Without<LeadIndicator>)>,
    mut center_query: Query<&mut BackgroundColor, (With<ReticuleCenter>, Without<Player>)>,
    mut lead_query: Query<(&mut Style, &mut Visibility, &mut BorderColor), (With<LeadIndicator>, Without<TargetingReticule>, Without<ReticuleCircle>)>,
) {
    let Ok((player_transform, weapon_mount, player_velocity, bonuses)) = player_query.get_single() else {
        return;
    };
    
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };
    
    // Get current weapon's projectile speed and apply bonuses
    let base_projectile_speed = if let Some(weapon) = weapon_mount.weapons.get(weapon_mount.current_weapon) {
        weapon.projectile_speed
    } else {
        150.0 // Default speed if no weapon
    };
    let projectile_speed = base_projectile_speed * bonuses.projectile_speed_multiplier;
    
    let forward = player_transform.forward();
    let forward_vec = forward.as_vec3();
    
    // Safety check: ensure forward vector is valid (not NaN or zero)
    if !forward_vec.is_finite() || forward_vec.length_squared() < 0.01 {
        // Forward vector is invalid, don't update reticule
        return;
    }
    
    let player_pos = player_transform.translation;
    let player_vel = player_velocity.0;
    
    // Safety check: ensure player position is valid
    if !player_pos.is_finite() {
        return;
    }
    
    // Find closest enemy in front of player
    let target_cone_angle = 15.0_f32.to_radians(); // Wider cone for lead indicator
    let mut closest_enemy: Option<(Vec3, Vec3, f32)> = None; // (position, velocity, distance)
    let mut closest_dist = f32::MAX;
    
    for (enemy_transform, enemy_velocity) in enemy_query.iter() {
        let to_enemy = enemy_transform.translation - player_pos;
        let distance = to_enemy.length();
        
        // Safety check: skip invalid distances
        if !distance.is_finite() || distance < 0.1 || distance > 300.0 {
            continue;
        }
        
        let direction = to_enemy.normalize();
        
        // Safety check: ensure direction is valid
        if !direction.is_finite() {
            continue;
        }
        
        let dot = forward_vec.dot(direction);
        
        // Check if enemy is in front and within cone
        if dot.is_finite() && dot > target_cone_angle.cos() && distance < closest_dist {
            closest_dist = distance;
            closest_enemy = Some((enemy_transform.translation, enemy_velocity.0, distance));
        }
    }
    
    // Calculate main reticule position (unchanged - uses forward direction)
    // Use raycast along forward direction to find aiming point
    let aim_distance = closest_enemy.as_ref()
        .map(|(_, _, d)| d.clamp(10.0, 500.0)) // Clamp distance to safe range
        .unwrap_or(100.0);
    let aim_point = player_pos + forward_vec * aim_distance;
    
    // Safety check: ensure aim point is valid before projecting
    if !aim_point.is_finite() {
        return;
    }
    
    // Project 3D aim point to screen coordinates
    if let Some(screen_pos) = camera.world_to_viewport(camera_transform, aim_point) {
        // Safety check: ensure screen position is valid
        if screen_pos.x.is_finite() && screen_pos.y.is_finite() {
            // Position reticule at screen coordinates (centered on the aim point)
            for mut style in reticule_query.iter_mut() {
                style.left = Val::Px(screen_pos.x - 25.0); // Center the 50px reticule
                style.top = Val::Px(screen_pos.y - 25.0);
            }
        }
    }
    
    // Update lead indicator (prediction square) - shows accurate intercept point
    // Only accounts for enemy velocity since projectiles no longer inherit momentum
    if let Ok((mut lead_style, mut lead_visibility, mut lead_border)) = lead_query.get_single_mut() {
        if let Some((target_pos, target_vel, distance)) = closest_enemy {
            // Check if target is moving significantly
            let target_speed = target_vel.length();
            
            // Show lead indicator if target is moving significantly
            if target_speed > 5.0 {
                // Calculate intercept point using the improved algorithm
                if let Some(intercept_point) = calculate_intercept_point(
                    player_pos,
                    player_vel,
                    target_pos,
                    target_vel,
                    projectile_speed,
                    forward_vec,
                ) {
                    // Safety check: ensure intercept point is valid
                    if intercept_point.is_finite() {
                        // Project intercept point to screen
                        if let Some(lead_screen_pos) = camera.world_to_viewport(camera_transform, intercept_point) {
                            // Safety check: ensure screen position is valid
                            if lead_screen_pos.x.is_finite() && lead_screen_pos.y.is_finite() {
                                lead_style.left = Val::Px(lead_screen_pos.x - 12.0); // Center the 24px indicator
                                lead_style.top = Val::Px(lead_screen_pos.y - 12.0);
                                *lead_visibility = Visibility::Visible;
                                
                                // Color based on lead distance - more noticeable for larger leads
                                let lead_offset = (intercept_point - target_pos).length();
                                if lead_offset.is_finite() {
                                    let color = if lead_offset > distance * 0.3 {
                                        Color::srgba(1.0, 1.0, 0.0, 0.8) // Yellow for large lead
                                    } else {
                                        Color::srgba(1.0, 0.5, 0.0, 0.8) // Orange for good lead
                                    };
                                    *lead_border = color.into();
                                }
                            } else {
                                *lead_visibility = Visibility::Hidden;
                            }
                        } else {
                            *lead_visibility = Visibility::Hidden;
                        }
                    } else {
                        *lead_visibility = Visibility::Hidden;
                    }
                } else {
                    *lead_visibility = Visibility::Hidden;
                }
            } else {
                *lead_visibility = Visibility::Hidden;
            }
        } else {
            *lead_visibility = Visibility::Hidden;
        }
    }
    
    // Check if any enemy is within precise crosshair cone (±5 degrees)
    let precise_cone_angle = 5.0_f32.to_radians();
    let mut enemy_in_crosshair = false;
    
    for (enemy_transform, _) in enemy_query.iter() {
        let to_enemy = enemy_transform.translation - player_pos;
        let distance = to_enemy.length();
        
        // Safety check: skip invalid distances
        if !distance.is_finite() || distance < 0.1 {
            continue;
        }
        
        let direction = to_enemy.normalize();
        
        // Safety check: ensure direction is valid
        if !direction.is_finite() {
            continue;
        }
        
        let dot = forward_vec.dot(direction);
        
        // dot = cos(angle), so if angle < threshold, we're aiming at enemy
        if dot.is_finite() && dot > precise_cone_angle.cos() {
            enemy_in_crosshair = true;
            break;
        }
    }
    
    // Update reticule colors based on targeting - CYBERPUNK COLORS
    let (circle_color, dot_color) = if enemy_in_crosshair {
        (colors::DANGER_COLOR, colors::DANGER_COLOR) // Red when targeting (locked on)
    } else {
        (colors::NEON_GREEN, colors::NEON_CYAN) // Green circle, cyan dot when not targeting
    };
    
    for mut border_color in circle_query.iter_mut() {
        *border_color = circle_color.into();
    }
    
    for mut bg_color in center_query.iter_mut() {
        *bg_color = dot_color.into();
    }
}

/// Enemy health bar marker
#[derive(Component)]
pub struct EnemyHealthBar {
    pub parent_ship: Entity,
}

/// Setup 3D health bars for enemies - CYBERPUNK HOLOGRAPHIC STYLE
pub fn setup_enemy_health_bars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    enemy_query: Query<Entity, (With<crate::components::ai::Enemy>, Without<EnemyHealthBar>)>,
    existing_bars: Query<&EnemyHealthBar>,
) {
    for enemy_entity in enemy_query.iter() {
        // Check if this enemy already has a health bar
        let already_has_bar = existing_bars.iter().any(|bar| bar.parent_ship == enemy_entity);
        if already_has_bar {
            continue;
        }
        
        // Spawn health bar background - HOLOGRAPHIC FRAME
        let bar_background = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(2.2, 0.35, 0.02)),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgba(0.0, 0.0, 0.0, 0.8), // Dark translucent
                    unlit: true,
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 5.0, 0.0),
                ..default()
            },
            EnemyHealthBar {
                parent_ship: enemy_entity,
            },
        )).id();
        
        // Spawn neon border frame (outline)
        commands.entity(bar_background).with_children(|parent| {
            // Top border
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(2.2, 0.05, 0.01)),
                material: materials.add(StandardMaterial {
                    base_color: colors::NEON_CYAN,
                    emissive: (colors::NEON_CYAN.to_linear() * 2.0).into(),
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.2, 0.03),
                ..default()
            });
            
            // Bottom border
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cuboid::new(2.2, 0.05, 0.01)),
                material: materials.add(StandardMaterial {
                    base_color: colors::NEON_CYAN,
                    emissive: (colors::NEON_CYAN.to_linear() * 2.0).into(),
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, -0.2, 0.03),
                ..default()
            });
            
            // Health bar fill - NEON RED
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(2.0, 0.25, 0.01)),
                    material: materials.add(StandardMaterial {
                        base_color: colors::HEALTH_COLOR,
                        emissive: (colors::HEALTH_COLOR.to_linear() * 1.5).into(),
                        unlit: true,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.0, 0.02),
                    ..default()
                },
                HealthBarFill,
            ));
            
            // Shield bar fill (on top of health) - NEON CYAN
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(2.0, 0.2, 0.01)),
                    material: materials.add(StandardMaterial {
                        base_color: colors::SHIELD_COLOR,
                        emissive: (colors::SHIELD_COLOR.to_linear() * 1.5).into(),
                        unlit: true,
                        alpha_mode: AlphaMode::Blend,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.0, 0.03),
                    ..default()
                },
                ShieldBarFill,
            ));
        });
    }
}

/// Health bar fill marker
#[derive(Component)]
pub struct HealthBarFill;

/// Shield bar fill marker (for 3D bars)
#[derive(Component)]
pub struct ShieldBarFill;

/// Update 3D enemy health bars
pub fn update_enemy_health_bars(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform, &Health, &Shield), With<crate::components::ai::Enemy>>,
    mut bar_query: Query<(Entity, &EnemyHealthBar, &mut Transform), (Without<crate::components::ai::Enemy>, Without<HealthBarFill>, Without<ShieldBarFill>)>,
    mut fill_query: Query<(&Parent, &mut Transform, &mut Visibility), (With<HealthBarFill>, Without<crate::components::ai::Enemy>, Without<EnemyHealthBar>, Without<ShieldBarFill>)>,
    mut shield_fill_query: Query<(&Parent, &mut Transform, &mut Visibility), (With<ShieldBarFill>, Without<crate::components::ai::Enemy>, Without<HealthBarFill>, Without<EnemyHealthBar>)>,
    camera_query: Query<&Transform, (With<crate::components::camera::CameraController>, Without<crate::components::ai::Enemy>, Without<EnemyHealthBar>, Without<HealthBarFill>, Without<ShieldBarFill>)>,
) {
    let camera_transform = camera_query.get_single().ok();
    
    // Update bar positions and rotations
    for (bar_entity, health_bar, mut bar_transform) in bar_query.iter_mut() {
        if let Ok((_enemy_entity, enemy_transform, _health, _shield)) = enemy_query.get(health_bar.parent_ship) {
            // Position bar above enemy
            bar_transform.translation = enemy_transform.translation + Vec3::new(0.0, 5.0, 0.0);
            
            // Billboard effect - face camera
            if let Some(cam_transform) = camera_transform {
                let to_camera = (cam_transform.translation - bar_transform.translation).normalize();
                bar_transform.look_to(to_camera, Vec3::Y);
            }
        } else {
            // Enemy is dead, despawn health bar
            commands.entity(bar_entity).despawn_recursive();
        }
    }
    
    // Update health bar fills
    for (parent, mut fill_transform, mut visibility) in fill_query.iter_mut() {
        if let Ok((_bar_entity, health_bar, _)) = bar_query.get(parent.get()) {
            if let Ok((_, _, health, _)) = enemy_query.get(health_bar.parent_ship) {
                let health_percent = (health.current / health.max).clamp(0.0, 1.0);
                
                // Only show if damaged
                *visibility = if health_percent < 1.0 {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
                
                // Scale width and adjust position
                fill_transform.scale.x = health_percent;
                fill_transform.translation.x = -1.0 * (1.0 - health_percent);
                
                // Color based on health
                // This would require updating the material, skipping for performance
            }
        }
    }
    
    // Update shield bar fills
    for (parent, mut fill_transform, mut visibility) in shield_fill_query.iter_mut() {
        if let Ok((_bar_entity, health_bar, _)) = bar_query.get(parent.get()) {
            if let Ok((_, _, _, shield)) = enemy_query.get(health_bar.parent_ship) {
                let shield_percent = (shield.current / shield.max).clamp(0.0, 1.0);
                
                // Only show if shields exist
                *visibility = if shield_percent > 0.0 {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
                
                // Scale width and adjust position
                fill_transform.scale.x = shield_percent;
                fill_transform.translation.x = -1.0 * (1.0 - shield_percent);
            }
        }
    }
}

