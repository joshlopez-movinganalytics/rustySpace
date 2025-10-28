use bevy::prelude::*;
use crate::components::ship::Player;
use crate::components::combat::{Health, Shield, Energy, WeaponMount};
use crate::components::resources::Inventory;
use crate::components::upgrades::{PlayerUpgrades, UpgradeType, UpgradeCategory};
use crate::resources::GameState;

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

/// Update HUD system
pub fn update_hud_system(
    mut commands: Commands,
    player_query: Query<(&Health, &Shield, &Energy, &WeaponMount), With<Player>>,
    inventory: Res<Inventory>,
    hud_query: Query<Entity, With<HudRoot>>,
    health_bar_query: Query<Entity, With<HealthBar>>,
    shield_bar_query: Query<Entity, With<ShieldBar>>,
    energy_bar_query: Query<Entity, With<EnergyBar>>,
) {
    // Create HUD if it doesn't exist
    if hud_query.is_empty() {
        setup_hud(&mut commands);
    }
    
    if let Ok((health, shield, energy, weapon_mount)) = player_query.get_single() {
        // Update health bar
        for entity in health_bar_query.iter() {
            let health_percent = (health.current / health.max).clamp(0.0, 1.0);
            commands.entity(entity).insert(Style {
                width: Val::Percent(health_percent * 100.0),
                height: Val::Px(20.0),
                ..default()
            });
        }
        
        // Update shield bar
        for entity in shield_bar_query.iter() {
            let shield_percent = (shield.current / shield.max).clamp(0.0, 1.0);
            commands.entity(entity).insert(Style {
                width: Val::Percent(shield_percent * 100.0),
                height: Val::Px(20.0),
                ..default()
            });
        }
        
        // Update energy bar
        for entity in energy_bar_query.iter() {
            let energy_percent = (energy.current / energy.max).clamp(0.0, 1.0);
            commands.entity(entity).insert(Style {
                width: Val::Percent(energy_percent * 100.0),
                height: Val::Px(20.0),
                ..default()
            });
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
}

fn setup_hud(commands: &mut Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(20.0),
                    top: Val::Px(20.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            // Health bar container
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                // Health label
                parent.spawn(TextBundle::from_section(
                    "HULL",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                
                // Health bar background
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(20.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                    border_color: Color::srgb(0.5, 0.5, 0.5).into(),
                    ..default()
                }).with_children(|parent| {
                    // Health bar fill
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(20.0),
                                ..default()
                            },
                            background_color: Color::srgb(0.8, 0.2, 0.2).into(),
                            ..default()
                        },
                        HealthBar,
                    ));
                });
            });
            
            // Shield bar container
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                // Shield label
                parent.spawn(TextBundle::from_section(
                    "SHIELDS",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                
                // Shield bar background
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(20.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                    border_color: Color::srgb(0.5, 0.5, 0.5).into(),
                    ..default()
                }).with_children(|parent| {
                    // Shield bar fill
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(20.0),
                                ..default()
                            },
                            background_color: Color::srgb(0.2, 0.5, 1.0).into(),
                            ..default()
                        },
                        ShieldBar,
                    ));
                });
            });
            
            // Energy bar container
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(5.0),
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                // Energy label
                parent.spawn(TextBundle::from_section(
                    "ENERGY",
                    TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
                
                // Energy bar background
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(20.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                    border_color: Color::srgb(0.5, 0.5, 0.5).into(),
                    ..default()
                }).with_children(|parent| {
                    // Energy bar fill
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(20.0),
                                ..default()
                            },
                            background_color: Color::srgb(0.2, 0.8, 0.3).into(),
                            ..default()
                        },
                        EnergyBar,
                    ));
                });
            });
        });
}

/// Main menu marker
#[derive(Component)]
pub struct MainMenuRoot;

/// Setup main menu
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
                background_color: Color::srgb(0.0, 0.0, 0.1).into(),
                ..default()
            },
            MainMenuRoot,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "SPACE COMBAT",
                    TextStyle {
                        font_size: 60.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
            
            parent.spawn(
                TextBundle::from_section(
                    "Press ENTER to Start",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::srgb(0.7, 0.7, 0.7),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );
            
            parent.spawn(
                TextBundle::from_section(
                    "Controls:\nWASD - Move | Arrow Keys - Rotate | Space/Ctrl - Up/Down\nMouse - Fire | Q/E - Roll | Shift - Boost\n1/2/3 - Switch Weapons | U - Upgrades",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(0.5, 0.5, 0.5),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                })
                .with_text_justify(JustifyText::Center),
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
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        println!("[UI System] Starting game...");
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

/// Upgrade menu marker
#[derive(Component)]
pub struct UpgradeMenuRoot;

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
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                }),
                ResourceDisplay,
            ));
            
            // Categories
            for category in [
                UpgradeCategory::Hull,
                UpgradeCategory::Shields,
                UpgradeCategory::Engines,
                UpgradeCategory::PowerPlant,
                UpgradeCategory::Weapons,
            ] {
                parent.spawn(
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
                    
                    parent.spawn((
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
            
            // Instructions
            parent.spawn(
                TextBundle::from_section(
                    "Click buttons or press number keys (1-9) to purchase upgrades | ESC to return",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::srgb(0.6, 0.6, 0.6),
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(Val::Px(30.0)),
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

/// Apply purchased upgrades to player ship
pub fn apply_upgrades_to_player(
    upgrades: Res<PlayerUpgrades>,
    mut player_query: Query<
        (&mut Health, &mut Shield, &mut Energy, &mut crate::components::ship::Ship, &mut WeaponMount),
        With<Player>,
    >,
) {
    if !upgrades.is_changed() {
        return;
    }
    
    if let Ok((mut health, mut shield, mut energy, mut ship, mut weapon_mount)) = player_query.get_single_mut() {
        // Base stats
        let base_health = 100.0;
        let base_shield = 100.0;
        let base_shield_recharge = 10.0;
        let base_energy = 100.0;
        let base_energy_recharge = 20.0;
        let base_speed = 80.0;
        let base_turn_rate = 4.0;
        
        // Calculate hull upgrades
        let mut health_multiplier = 1.0;
        if upgrades.has_upgrade(UpgradeType::HullIntegrity1) {
            health_multiplier += 0.25;
        }
        if upgrades.has_upgrade(UpgradeType::HullIntegrity2) {
            health_multiplier += 0.50;
        }
        if upgrades.has_upgrade(UpgradeType::HullIntegrity3) {
            health_multiplier += 1.0;
        }
        
        health.max = base_health * health_multiplier;
        health.current = health.current.min(health.max);
        
        // Calculate shield upgrades
        let mut shield_multiplier = 1.0;
        if upgrades.has_upgrade(UpgradeType::ShieldCapacity1) {
            shield_multiplier += 0.25;
        }
        if upgrades.has_upgrade(UpgradeType::ShieldCapacity2) {
            shield_multiplier += 0.50;
        }
        if upgrades.has_upgrade(UpgradeType::ShieldCapacity3) {
            shield_multiplier += 1.0;
        }
        
        shield.max = base_shield * shield_multiplier;
        shield.current = shield.current.min(shield.max);
        
        let mut shield_recharge_multiplier = 1.0;
        if upgrades.has_upgrade(UpgradeType::ShieldRecharge1) {
            shield_recharge_multiplier += 0.50;
        }
        if upgrades.has_upgrade(UpgradeType::ShieldRecharge2) {
            shield_recharge_multiplier += 1.0;
        }
        
        shield.recharge_rate = base_shield_recharge * shield_recharge_multiplier;
        
        // Calculate engine upgrades
        let mut speed_multiplier = 1.0;
        if upgrades.has_upgrade(UpgradeType::EngineSpeed1) {
            speed_multiplier += 0.20;
        }
        if upgrades.has_upgrade(UpgradeType::EngineSpeed2) {
            speed_multiplier += 0.40;
        }
        if upgrades.has_upgrade(UpgradeType::EngineSpeed3) {
            speed_multiplier += 0.60;
        }
        
        ship.max_speed = base_speed * speed_multiplier;
        
        let mut turn_rate_multiplier = 1.0;
        if upgrades.has_upgrade(UpgradeType::Maneuverability1) {
            turn_rate_multiplier += 0.30;
        }
        if upgrades.has_upgrade(UpgradeType::Maneuverability2) {
            turn_rate_multiplier += 0.60;
        }
        
        ship.turn_rate = base_turn_rate * turn_rate_multiplier;
        
        // Calculate power plant upgrades
        let mut energy_multiplier = 1.0;
        if upgrades.has_upgrade(UpgradeType::PowerCapacity1) {
            energy_multiplier += 0.50;
        }
        if upgrades.has_upgrade(UpgradeType::PowerCapacity2) {
            energy_multiplier += 1.0;
        }
        
        energy.max = base_energy * energy_multiplier;
        energy.current = energy.current.min(energy.max);
        
        let mut energy_recharge_multiplier = 1.0;
        if upgrades.has_upgrade(UpgradeType::PowerRecharge1) {
            energy_recharge_multiplier += 0.50;
        }
        if upgrades.has_upgrade(UpgradeType::PowerRecharge2) {
            energy_recharge_multiplier += 1.0;
        }
        
        energy.recharge_rate = base_energy_recharge * energy_recharge_multiplier;
        
        // Add unlocked weapons
        let has_plasma = weapon_mount.weapons.iter().any(|w| matches!(w.weapon_type, crate::components::combat::WeaponType::Plasma));
        let has_missile = weapon_mount.weapons.iter().any(|w| matches!(w.weapon_type, crate::components::combat::WeaponType::Missile));
        let has_railgun = weapon_mount.weapons.iter().any(|w| matches!(w.weapon_type, crate::components::combat::WeaponType::Railgun));
        
        if upgrades.has_upgrade(UpgradeType::UnlockPlasma) && !has_plasma {
            weapon_mount.weapons.push(crate::components::combat::Weapon::plasma());
            println!("[UI System] Unlocked Plasma Cannon!");
        }
        
        if upgrades.has_upgrade(UpgradeType::UnlockMissile) && !has_missile {
            weapon_mount.weapons.push(crate::components::combat::Weapon::missile());
            println!("[UI System] Unlocked Missile Launcher!");
        }
        
        if upgrades.has_upgrade(UpgradeType::UnlockRailgun) && !has_railgun {
            weapon_mount.weapons.push(crate::components::combat::Weapon::railgun());
            println!("[UI System] Unlocked Railgun!");
        }
        
        println!("[UI System] Applied upgrades to player ship");
    }
}

fn get_upgrades_for_category(category: UpgradeCategory) -> Vec<UpgradeType> {
    use UpgradeType::*;
    match category {
        UpgradeCategory::Hull => vec![
            HullIntegrity1, HullIntegrity2, HullIntegrity3,
            ArmorPlating1, ArmorPlating2,
        ],
        UpgradeCategory::Shields => vec![
            ShieldCapacity1, ShieldCapacity2, ShieldCapacity3,
            ShieldRecharge1, ShieldRecharge2,
        ],
        UpgradeCategory::Engines => vec![
            EngineSpeed1, EngineSpeed2, EngineSpeed3,
            Maneuverability1, Maneuverability2,
        ],
        UpgradeCategory::PowerPlant => vec![
            PowerCapacity1, PowerCapacity2,
            PowerRecharge1, PowerRecharge2,
        ],
        UpgradeCategory::Weapons => vec![
            WeaponDamage1, WeaponDamage2,
            WeaponFireRate1, WeaponFireRate2,
            UnlockPlasma, UnlockMissile, UnlockRailgun,
        ],
    }
}

/// Enemy health bar marker
#[derive(Component)]
pub struct EnemyHealthBar {
    pub parent_ship: Entity,
}

/// Setup 3D health bars for enemies
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
        
        // Spawn health bar background
        let bar_background = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(2.0, 0.2, 0.01)),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.2, 0.2, 0.2),
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 5.0, 0.0),
                ..default()
            },
            EnemyHealthBar {
                parent_ship: enemy_entity,
            },
        )).id();
        
        // Spawn health bar fill (child of background)
        commands.entity(bar_background).with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(2.0, 0.2, 0.01)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::srgb(0.2, 0.8, 0.2),
                        unlit: true,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.0, 0.01),
                    ..default()
                },
                HealthBarFill,
            ));
            
            // Spawn shield bar fill (on top of health)
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(2.0, 0.15, 0.01)),
                    material: materials.add(StandardMaterial {
                        base_color: Color::srgb(0.2, 0.5, 1.0),
                        unlit: true,
                        alpha_mode: AlphaMode::Blend,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.2, 0.02),
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

