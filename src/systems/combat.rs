use bevy::prelude::*;
use crate::components::ship::*;
use crate::components::combat::*;
use crate::components::ai::Enemy;
use crate::resources::GameState;

/// Weapon state management system (heat, ammo, reload)
pub fn weapon_state_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut WeaponMount, With<Player>>,
) {
    let dt = time.delta_seconds();
    
    for mut weapon_mount in query.iter_mut() {
        for weapon in weapon_mount.weapons.iter_mut() {
            // Heat dissipation
            if weapon.max_heat > 0.0 && weapon.heat > 0.0 {
                weapon.heat = (weapon.heat - weapon.cooling_rate * dt).max(0.0);
            }
            
            // Handle reloading
            if weapon.is_reloading {
                weapon.reload_timer += dt;
                if weapon.reload_timer >= weapon.reload_time {
                    // Reload complete
                    let ammo_needed = weapon.max_ammo - weapon.current_ammo;
                    let ammo_to_load = ammo_needed.min(weapon.reserve_ammo);
                    weapon.current_ammo += ammo_to_load;
                    weapon.reserve_ammo -= ammo_to_load;
                    weapon.is_reloading = false;
                    weapon.reload_timer = 0.0;
                    println!("[Combat] Reload complete! Ammo: {}/{}", weapon.current_ammo, weapon.reserve_ammo);
                }
            }
        }
        
        // Manual reload with R key
        if keyboard.just_pressed(KeyCode::KeyR) {
            let current_weapon_idx = weapon_mount.current_weapon;
            if let Some(weapon) = weapon_mount.weapons.get_mut(current_weapon_idx) {
                // Only reload if weapon has ammo system and not already reloading
                if weapon.max_ammo > 0 && !weapon.is_reloading && weapon.current_ammo < weapon.max_ammo && weapon.reserve_ammo > 0 {
                    weapon.is_reloading = true;
                    weapon.reload_timer = 0.0;
                    println!("[Combat] Reloading...");
                }
            }
        }
    }
}

/// Weapon firing system
pub fn weapon_firing_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &Transform, &Velocity, &mut WeaponMount, &mut Energy, &crate::components::ship_classes::ClassBonuses), With<Player>>,
) {
    let dt = time.delta_seconds();
    
    for (entity, transform, velocity, mut weapon_mount, mut energy, bonuses) in query.iter_mut() {
        // Update cooldown timers
        for weapon in weapon_mount.weapons.iter_mut() {
            weapon.cooldown_timer = (weapon.cooldown_timer - dt).max(0.0);
            
            // Update alt-fire charge for plasma
            if weapon.weapon_type == WeaponType::Plasma && mouse.pressed(MouseButton::Right) {
                weapon.alt_fire_charge = (weapon.alt_fire_charge + dt).min(2.0);
            } else if weapon.weapon_type == WeaponType::Plasma && mouse.just_released(MouseButton::Right) {
                // Will be handled in firing logic
            } else if weapon.weapon_type == WeaponType::Plasma {
                weapon.alt_fire_charge = 0.0;
            }
        }
        
        // Switch weapons
        if keyboard.just_pressed(KeyCode::Digit1) && weapon_mount.weapons.len() > 0 {
            weapon_mount.current_weapon = 0;
        }
        if keyboard.just_pressed(KeyCode::Digit2) && weapon_mount.weapons.len() > 1 {
            weapon_mount.current_weapon = 1;
        }
        if keyboard.just_pressed(KeyCode::Digit3) && weapon_mount.weapons.len() > 2 {
            weapon_mount.current_weapon = 2;
        }
        if keyboard.just_pressed(KeyCode::Digit4) && weapon_mount.weapons.len() > 3 {
            weapon_mount.current_weapon = 3;
        }
        
        let current_weapon_idx = weapon_mount.current_weapon;
        
        // Primary fire (Left Mouse)
        if mouse.pressed(MouseButton::Left) {
            if let Some(weapon) = weapon_mount.weapons.get_mut(current_weapon_idx) {
                // Check all firing conditions
                let can_fire = weapon.cooldown_timer <= 0.0 
                    && energy.current >= weapon.energy_cost
                    && !weapon.is_reloading
                    && (weapon.max_heat == 0.0 || weapon.heat < weapon.max_heat) // Not overheated
                    && (weapon.max_ammo == 0 || weapon.current_ammo > 0); // Has ammo or infinite
                
                if can_fire {
                    fire_weapon(&mut commands, &mut meshes, &mut materials, entity, transform, velocity, weapon, &mut energy, bonuses, false);
                    // Apply fire rate multiplier to cooldown
                    weapon.cooldown_timer = (1.0 / weapon.fire_rate) / bonuses.fire_rate_multiplier;
                } else if weapon.max_ammo > 0 && weapon.current_ammo == 0 && !weapon.is_reloading {
                    // Auto-reload when trying to fire with empty mag
                    if weapon.reserve_ammo > 0 {
                        weapon.is_reloading = true;
                        weapon.reload_timer = 0.0;
                        println!("[Combat] Auto-reloading...");
                    }
                }
            }
        }
        
        // Alt-fire (Right Mouse)
        if mouse.just_pressed(MouseButton::Right) {
            if let Some(weapon) = weapon_mount.weapons.get_mut(current_weapon_idx) {
                // Burst fire for laser
                if weapon.weapon_type == WeaponType::Laser && weapon.cooldown_timer <= 0.0 && energy.current >= weapon.energy_cost * 3.0 {
                    // Fire 3-shot burst
                    for _i in 0..3 {
                        // For simplicity, fire all 3 immediately with slight spread
                        fire_weapon(&mut commands, &mut meshes, &mut materials, entity, transform, velocity, weapon, &mut energy, bonuses, true);
                    }
                    weapon.cooldown_timer = (1.0 / weapon.fire_rate) / bonuses.fire_rate_multiplier;
                }
                // Autocannon spread mode
                else if weapon.weapon_type == WeaponType::Autocannon && weapon.cooldown_timer <= 0.0 && energy.current >= weapon.energy_cost * 5.0 {
                    // Fire 5-shot spread (shotgun)
                    for _ in 0..5 {
                        fire_weapon_spread(&mut commands, &mut meshes, &mut materials, entity, transform, velocity, weapon, &mut energy);
                    }
                    weapon.cooldown_timer = 1.0 / weapon.fire_rate;
                }
                // Missile swarm
                else if weapon.weapon_type == WeaponType::Missile && weapon.cooldown_timer <= 0.0 && energy.current >= weapon.energy_cost * 2.0 {
                    // Fire 3 weaker missiles
                    for _ in 0..3 {
                        fire_missile_swarm(&mut commands, &mut meshes, &mut materials, entity, transform, velocity, weapon, &mut energy);
                    }
                    weapon.cooldown_timer = 1.0 / weapon.fire_rate;
                }
                // Railgun piercing
                else if weapon.weapon_type == WeaponType::Railgun && weapon.cooldown_timer <= 0.0 && energy.current >= weapon.energy_cost * 1.5 {
                    fire_piercing_railgun(&mut commands, &mut meshes, &mut materials, entity, transform, velocity, weapon, &mut energy);
                    weapon.cooldown_timer = 1.0 / weapon.fire_rate;
                }
            }
        }
        
        // Plasma charged shot (release)
        if mouse.just_released(MouseButton::Right) {
            if let Some(weapon) = weapon_mount.weapons.get_mut(current_weapon_idx) {
                if weapon.weapon_type == WeaponType::Plasma && weapon.alt_fire_charge > 0.5 {
                    let charge_mult = weapon.alt_fire_charge;
                    if energy.current >= weapon.energy_cost * charge_mult {
                        fire_charged_plasma(&mut commands, &mut meshes, &mut materials, entity, transform, velocity, weapon, &mut energy, charge_mult);
                        weapon.alt_fire_charge = 0.0;
                        weapon.cooldown_timer = 1.0 / weapon.fire_rate;
                    }
                }
            }
        }
    }
}

fn fire_weapon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    owner: Entity,
    transform: &Transform,
    velocity: &Velocity,
    weapon: &mut Weapon,
    energy: &mut Energy,
    bonuses: &crate::components::ship_classes::ClassBonuses,
    is_burst: bool,
) {
    // Cooldown is set by caller with fire_rate_multiplier
    energy.current -= weapon.energy_cost;
    
    // Consume heat
    if weapon.max_heat > 0.0 {
        weapon.heat += weapon.heat_per_shot;
        if weapon.heat >= weapon.max_heat {
            println!("[Combat] {} overheated! Cooling down...", match weapon.weapon_type {
                WeaponType::Laser => "Laser",
                WeaponType::BeamLaser => "Beam Laser",
                _ => "Weapon",
            });
        }
    }
    
    // Consume ammo
    if weapon.max_ammo > 0 {
        weapon.current_ammo = weapon.current_ammo.saturating_sub(1);
        if weapon.current_ammo == 0 {
            println!("[Combat] {} out of ammo!", match weapon.weapon_type {
                WeaponType::Autocannon => "Autocannon",
                WeaponType::Missile => "Missiles",
                _ => "Weapon",
            });
        }
    }
    
    let forward = transform.forward();
    let projectile_pos = transform.translation + forward.as_vec3() * 3.0;
    
    // Add spread
    let spread_mult = if is_burst { 1.5 } else { 1.0 };
    let spread_x = (rand::random::<f32>() - 0.5) * weapon.spread * spread_mult;
    let spread_y = (rand::random::<f32>() - 0.5) * weapon.spread * spread_mult;
    let spread_rotation = Quat::from_euler(EulerRot::XYZ, spread_y, spread_x, 0.0);
    let projectile_direction = (spread_rotation * forward.as_vec3()).normalize();
    
    let projectile_velocity = projectile_direction * weapon.projectile_speed + velocity.0;
    
    // Calculate final damage first (needed for laser color)
    let base_damage = weapon.damage;
    let mut final_damage = base_damage * bonuses.damage_multiplier;
    let is_critical = rand::random::<f32>() < bonuses.critical_chance;
    if is_critical {
        final_damage *= bonuses.critical_multiplier;
    }
    
    // Determine piercing for lasers - check if projectile should pierce
    // This will be set on the Projectile component based on upgrades
    // For now, lasers don't pierce by default (only railgun does)
    let laser_piercing = false;
    
    // Get weapon visual - for lasers, calculate color based on damage and piercing
    let (mesh, base_color) = get_weapon_visual(weapon.weapon_type, meshes);
    let color = if weapon.weapon_type == WeaponType::Laser {
        calculate_laser_color(base_damage, final_damage, laser_piercing)
    } else {
        base_color
    };
    
    // Determine homing and area damage based on weapon type
    let (homing_strength, area_damage, piercing) = match weapon.weapon_type {
        WeaponType::Missile => (15.0, 8.0, false),      // Strong homing, large area
        WeaponType::FlakCannon => (0.0, 5.0, false),    // No homing, medium area
        WeaponType::Railgun => (0.0, 0.0, true),        // Piercing rounds
        _ => (0.0, 0.0, false),                         // No special effects
    };
    
    // Calculate rotation based on projectile direction instead of ship rotation
    // This ensures lasers always face the direction they were shot in
    let projectile_rotation = if projectile_direction.length() > 0.1 {
        Transform::from_translation(projectile_pos)
            .looking_to(projectile_direction, Vec3::Y)
    } else {
        Transform::from_translation(projectile_pos)
            .with_rotation(transform.rotation)
    };
    
    // Set emissive for bloom glow effect - keep color consistent, bloom will add the glow
    // For bloom to work, emissive should be bright enough to exceed the bloom threshold
    let emissive_intensity = if weapon.weapon_type == WeaponType::Laser || weapon.weapon_type == WeaponType::BeamLaser {
        LinearRgba::from(color) * 8.0 // Bright enough for bloom, but color stays consistent
    } else {
        LinearRgba::from(color) * 4.0
    };
    
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color: color,
                emissive: emissive_intensity,
                ..default()
            }),
            transform: projectile_rotation,
            ..default()
        },
        Projectile {
            damage: final_damage,
            lifetime: 5.0,
            owner,
            weapon_type: weapon.weapon_type,
            shield_damage_multiplier: weapon.shield_damage_multiplier,
            hull_damage_multiplier: weapon.hull_damage_multiplier,
            piercing,
            area_damage,
            homing_strength,
            homing_target: None,
            initial_direction: projectile_direction,
        },
        Velocity(projectile_velocity),
        Faction::Player,
    ));
}

fn fire_weapon_spread(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    owner: Entity,
    transform: &Transform,
    velocity: &Velocity,
    weapon: &mut Weapon,
    energy: &mut Energy,
) {
    energy.current -= weapon.energy_cost;
    
    // Consume ammo for spread shots
    if weapon.max_ammo > 0 {
        weapon.current_ammo = weapon.current_ammo.saturating_sub(1);
    }
    
    let forward = transform.forward();
    let projectile_pos = transform.translation + forward.as_vec3() * 3.0;
    
    // Extra spread for shotgun effect
    let spread_x = (rand::random::<f32>() - 0.5) * 0.15;
    let spread_y = (rand::random::<f32>() - 0.5) * 0.15;
    let spread_rotation = Quat::from_euler(EulerRot::XYZ, spread_y, spread_x, 0.0);
    let projectile_direction = (spread_rotation * forward.as_vec3()).normalize();
    
    let projectile_velocity = projectile_direction * weapon.projectile_speed + velocity.0;
    
    let (mesh, color) = get_weapon_visual(weapon.weapon_type, meshes);
    
    // Calculate rotation based on projectile direction
    let projectile_rotation = if projectile_direction.length() > 0.1 {
        Transform::from_translation(projectile_pos)
            .looking_to(projectile_direction, Vec3::Y)
    } else {
        Transform::from_translation(projectile_pos)
            .with_rotation(transform.rotation)
    };
    
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color: color,
                emissive: LinearRgba::from(color) * 4.0, // Bright enough for bloom glow
                ..default()
            }),
            transform: projectile_rotation,
            ..default()
        },
        Projectile {
            damage: weapon.damage * 0.6, // Reduced damage per pellet
            lifetime: 3.0,
            owner,
            weapon_type: weapon.weapon_type,
            shield_damage_multiplier: weapon.shield_damage_multiplier,
            hull_damage_multiplier: weapon.hull_damage_multiplier,
            piercing: false,
            area_damage: 0.0,
            homing_strength: 0.0,
            homing_target: None,
            initial_direction: projectile_direction,
        },
        Velocity(projectile_velocity),
        Faction::Player,
    ));
}

fn fire_missile_swarm(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    owner: Entity,
    transform: &Transform,
    velocity: &Velocity,
    weapon: &mut Weapon,
    energy: &mut Energy,
) {
    energy.current -= weapon.energy_cost * 0.7;
    
    let forward = transform.forward();
    let right = transform.right();
    let offsets = [-1.0, 0.0, 1.0];
    
    for offset in offsets.iter() {
        let projectile_pos = transform.translation + forward.as_vec3() * 3.0 + right.as_vec3() * *offset;
        let projectile_direction = forward.as_vec3().normalize();
        let projectile_velocity = forward.as_vec3() * weapon.projectile_speed + velocity.0;
        
        let (mesh, color) = get_weapon_visual(WeaponType::Missile, meshes);
        
        commands.spawn((
            PbrBundle {
                mesh,
                material: materials.add(StandardMaterial {
                    base_color: color,
                    emissive: LinearRgba::from(color) * 4.0, // Bright enough for bloom glow
                    ..default()
                }),
                transform: Transform::from_translation(projectile_pos)
                    .looking_to(projectile_direction, Vec3::Y),
                ..default()
            },
            Projectile {
                damage: weapon.damage * 0.5,
                lifetime: 5.0,
                owner,
                weapon_type: weapon.weapon_type,
                shield_damage_multiplier: weapon.shield_damage_multiplier,
                hull_damage_multiplier: weapon.hull_damage_multiplier,
                piercing: false,
                area_damage: 6.0,        // Swarm missiles have smaller area
                homing_strength: 12.0,    // Slightly weaker homing
                homing_target: None,
                initial_direction: projectile_direction,
            },
            Velocity(projectile_velocity),
            Faction::Player,
        ));
    }
}

fn fire_piercing_railgun(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    owner: Entity,
    transform: &Transform,
    velocity: &Velocity,
    weapon: &mut Weapon,
    energy: &mut Energy,
) {
    energy.current -= weapon.energy_cost * 1.5;
    
    let forward = transform.forward();
    let projectile_pos = transform.translation + forward.as_vec3() * 3.0;
    let projectile_direction = forward.as_vec3().normalize();
    let projectile_velocity = forward.as_vec3() * weapon.projectile_speed + velocity.0;
    
    let (mesh, color) = get_weapon_visual(WeaponType::Railgun, meshes);
    
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color: color,
                emissive: LinearRgba::from(color) * 4.0, // Bright enough for bloom glow
                ..default()
            }),
            transform: Transform::from_translation(projectile_pos)
                .looking_to(projectile_direction, Vec3::Y),
            ..default()
        },
        Projectile {
            damage: weapon.damage * 1.3,
            lifetime: 8.0,
            owner,
            weapon_type: weapon.weapon_type,
            shield_damage_multiplier: weapon.shield_damage_multiplier,
            hull_damage_multiplier: weapon.hull_damage_multiplier,
            piercing: true, // Pierces through enemies
            area_damage: 0.0,
            homing_strength: 0.0,
            homing_target: None,
            initial_direction: projectile_direction,
        },
        Velocity(projectile_velocity),
        Faction::Player,
    ));
}

fn fire_charged_plasma(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    owner: Entity,
    transform: &Transform,
    velocity: &Velocity,
    weapon: &mut Weapon,
    energy: &mut Energy,
    charge: f32,
) {
    energy.current -= weapon.energy_cost * charge;
    
    let forward = transform.forward();
    let projectile_pos = transform.translation + forward.as_vec3() * 3.0;
    let projectile_direction = forward.as_vec3().normalize();
    let projectile_velocity = forward.as_vec3() * weapon.projectile_speed + velocity.0;
    
    let size = 0.3 + charge * 0.3;
    let mesh = meshes.add(Sphere::new(size));
    let color = Color::srgb(0.2 + charge * 0.3, 1.0, 0.2);
    
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color: color,
                emissive: LinearRgba::from(color) * 4.0, // Bright enough for bloom glow
                ..default()
            }),
            transform: Transform::from_translation(projectile_pos)
                .looking_to(projectile_direction, Vec3::Y),
            ..default()
        },
        Projectile {
            damage: weapon.damage * (1.0 + charge),
            lifetime: 5.0,
            owner,
            weapon_type: weapon.weapon_type,
            shield_damage_multiplier: weapon.shield_damage_multiplier,
            hull_damage_multiplier: weapon.hull_damage_multiplier,
            piercing: false,
            area_damage: charge * 3.0,  // Charged shots have area damage
            homing_strength: 0.0,
            homing_target: None,
            initial_direction: projectile_direction,
        },
        Velocity(projectile_velocity),
        Faction::Player,
    ));
}

/// Calculate laser color based on damage and special properties
/// Green (weakest) -> Yellow -> Orange -> Red (strongest) gradient
/// Purple for special properties like piercing
pub fn calculate_laser_color(base_damage: f32, final_damage: f32, piercing: bool) -> Color {
    if piercing {
        // Special property: purple for piercing lasers
        return Color::srgb(0.8, 0.2, 1.0);
    }
    
    // Base damage is reference point (green)
    // Scale from green to red based on damage multiplier
    let damage_multiplier = final_damage / base_damage.max(1.0);
    
    // Clamp multiplier for color range (1.0 = green, 3.0+ = red)
    let intensity = ((damage_multiplier - 1.0) / 2.0).clamp(0.0, 1.0);
    
    // Gradient: Green -> Yellow -> Orange -> Red
    if intensity < 0.33 {
        // Green to Yellow
        let t = intensity / 0.33;
        Color::srgb(t, 1.0, 0.0)
    } else if intensity < 0.66 {
        // Yellow to Orange
        let t = (intensity - 0.33) / 0.33;
        Color::srgb(1.0, 1.0 - t * 0.5, 0.0)
    } else {
        // Orange to Red
        let t = (intensity - 0.66) / 0.34;
        Color::srgb(1.0, 0.5 - t * 0.5, 0.0)
    }
}

fn get_weapon_visual(weapon_type: WeaponType, meshes: &mut ResMut<Assets<Mesh>>) -> (Handle<Mesh>, Color) {
    match weapon_type {
        WeaponType::Laser => (
            meshes.add(Capsule3d::new(0.05, 1.5)), // Smaller and sharper (thinner, longer)
            Color::srgb(0.0, 1.0, 0.0), // Default green (will be overridden with actual damage)
        ),
        WeaponType::Plasma => (
            meshes.add(Sphere::new(0.3)),
            Color::srgb(0.2, 1.0, 0.2),
        ),
        WeaponType::Missile => (
            meshes.add(Capsule3d::new(0.15, 0.8)),
            Color::srgb(0.8, 0.8, 0.2),
        ),
        WeaponType::Railgun => (
            meshes.add(Capsule3d::new(0.08, 2.0)),
            Color::srgb(0.2, 0.5, 1.0),
        ),
        WeaponType::Autocannon => (
            meshes.add(Capsule3d::new(0.08, 0.6)),
            Color::srgb(1.0, 0.6, 0.0),
        ),
        WeaponType::IonCannon => (
            meshes.add(Sphere::new(0.25)),
            Color::srgb(0.3, 0.3, 1.0),
        ),
        WeaponType::FlakCannon => (
            meshes.add(Sphere::new(0.35)),
            Color::srgb(0.9, 0.5, 0.1),
        ),
        WeaponType::BeamLaser => (
            meshes.add(Capsule3d::new(0.05, 1.5)),
            Color::srgb(0.0, 1.0, 1.0),
        ),
    }
}

/// Projectile movement system
pub fn projectile_movement_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform, &Projectile)>,
) {
    let dt = time.delta_seconds();
    
    for (velocity, mut transform, projectile) in query.iter_mut() {
        transform.translation += velocity.0 * dt;
        
        // Orient projectile in its initial firing direction (not velocity, which includes ship movement)
        // This ensures lasers always face the direction they were shot in
        if projectile.initial_direction.length() > 0.1 {
            transform.look_to(projectile.initial_direction, Vec3::Y);
        } else if velocity.0.length() > 0.1 {
            // Fallback: use velocity direction if initial_direction is invalid
            transform.look_to(velocity.0.normalize(), Vec3::Y);
        }
    }
}

/// Homing projectile system - makes missiles track enemies
pub fn homing_projectile_system(
    time: Res<Time>,
    mut projectiles: Query<(&mut Projectile, &mut Velocity, &Transform, &Faction)>,
    enemies: Query<(Entity, &Transform, &Faction), (With<Health>, Without<Projectile>)>,
) {
    let dt = time.delta_seconds();
    
    for (mut projectile, mut velocity, proj_transform, proj_faction) in projectiles.iter_mut() {
        // Only process homing projectiles
        if projectile.homing_strength <= 0.0 {
            continue;
        }
        
        // Try to find target
        let target_pos = if let Some(target_entity) = projectile.homing_target {
            // Check if target still exists
            if let Ok((_, target_transform, _)) = enemies.get(target_entity) {
                Some(target_transform.translation)
            } else {
                // Target destroyed, find new one
                projectile.homing_target = None;
                None
            }
        } else {
            None
        };
        
        // If no target, find closest enemy
        let target_pos = target_pos.or_else(|| {
            let mut closest_dist = f32::MAX;
            let mut closest_pos = None;
            let mut closest_entity = None;
            
            for (entity, enemy_transform, enemy_faction) in enemies.iter() {
                // Don't target same faction
                if proj_faction == enemy_faction {
                    continue;
                }
                
                let dist = proj_transform.translation.distance(enemy_transform.translation);
                if dist < closest_dist && dist < 100.0 { // Max lock range
                    closest_dist = dist;
                    closest_pos = Some(enemy_transform.translation);
                    closest_entity = Some(entity);
                }
            }
            
            projectile.homing_target = closest_entity;
            closest_pos
        });
        
        // Apply homing if we have a target
        if let Some(target_pos) = target_pos {
            let to_target = (target_pos - proj_transform.translation).normalize();
            let current_dir = velocity.0.normalize();
            
            // Blend current direction with target direction
            let homing_factor = projectile.homing_strength * dt;
            let new_dir = (current_dir + to_target * homing_factor).normalize();
            
            // Update velocity maintaining speed
            let speed = velocity.0.length();
            velocity.0 = new_dir * speed;
            
            // Debug: Log when missile first acquires target (only once)
            if *proj_faction == Faction::Player && projectile.lifetime > 4.9 && projectile.lifetime < 5.0 {
                println!("[Combat] Missile acquired target, homing strength: {}", projectile.homing_strength);
            }
        }
    }
}

/// Projectile lifetime system
pub fn projectile_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile)>,
) {
    let dt = time.delta_seconds();
    
    for (entity, mut projectile) in query.iter_mut() {
        projectile.lifetime -= dt;
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Projectile collision system with area damage support
pub fn projectile_collision_system(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &Projectile, &Faction)>,
    mut ships: Query<(Entity, &Transform, &mut Health, &mut Shield, &Faction), Without<Projectile>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (proj_entity, proj_transform, projectile, proj_faction) in projectiles.iter() {
        let mut hit_primary = false;
        
        for (ship_entity, ship_transform, mut health, mut shield, ship_faction) in ships.iter_mut() {
            // Don't hit own faction or owner
            if proj_faction == ship_faction || projectile.owner == ship_entity {
                continue;
            }
            
            // Simple sphere collision for direct hit
            let distance = proj_transform.translation.distance(ship_transform.translation);
            
            // Check for area damage hit
            let in_area = projectile.area_damage > 0.0 && distance < projectile.area_damage;
            let direct_hit = distance < 2.0;
            
            if direct_hit || (in_area && hit_primary) {
                // Calculate damage based on hit type
                let damage_mult = if direct_hit { 1.0 } else { 0.5 }; // Area damage is 50%
                
                let hull_damage = projectile.damage * projectile.hull_damage_multiplier * damage_mult;
                let shield_damage = projectile.damage * projectile.shield_damage_multiplier * damage_mult;
                
                // Debug logging (only for player projectiles hitting enemies)
                if *proj_faction == Faction::Player && direct_hit {
                    println!("[Combat] {:?} hit: Shield dmg={:.1} (base={:.1}x{:.2}), Hull dmg={:.1} (base={:.1}x{:.2})", 
                        projectile.weapon_type, 
                        shield_damage, projectile.damage, projectile.shield_damage_multiplier,
                        hull_damage, projectile.damage, projectile.hull_damage_multiplier
                    );
                }
                
                // Hit shields first
                if shield.current > 0.0 {
                    let shield_before = shield.current;
                    shield.current -= shield_damage;
                    shield.time_since_last_hit = 0.0;
                    
                    // Spawn shield hit effect
                    if direct_hit {
                        crate::systems::effects::spawn_shield_hit_effect(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            ship_transform.translation,
                            proj_transform.translation,
                        );
                    }
                    
                    if shield.current <= 0.0 {
                        // Shield broken! Spawn break effect
                        if shield_before > 0.0 {
                            crate::systems::effects::spawn_shield_break_effect(
                                &mut commands,
                                &mut meshes,
                                &mut materials,
                                ship_transform.translation,
                            );
                        }
                        
                        // Overflow damage to hull (using hull multiplier)
                        health.current += shield.current * projectile.hull_damage_multiplier / projectile.shield_damage_multiplier;
                        shield.current = 0.0;
                    }
                } else {
                    // Hull hit - spawn sparks
                    health.current -= hull_damage;
                    
                    if direct_hit {
                        let projectile_dir = (proj_transform.translation - ship_transform.translation).normalize();
                        crate::systems::effects::spawn_hull_spark_effect(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            proj_transform.translation,
                            projectile_dir,
                        );
                    }
                }
                
                if direct_hit {
                    hit_primary = true;
                    
                    // Spawn area damage effect for missiles
                    if projectile.area_damage > 0.0 {
                        crate::systems::effects::spawn_explosion(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            proj_transform.translation,
                        );
                    }
                    
                    // Only despawn if not piercing
                    if !projectile.piercing {
                        commands.entity(proj_entity).despawn();
                        break;
                    }
                }
            }
        }
    }
}

/// Damage system
pub fn damage_system(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
) {
    for (entity, health) in query.iter() {
        if health.current <= 0.0 {
            commands.entity(entity).try_insert(DeadShip);
        }
    }
}

/// Shield recharge system
pub fn shield_recharge_system(
    time: Res<Time>,
    mut query: Query<&mut Shield>,
) {
    let dt = time.delta_seconds();
    
    for mut shield in query.iter_mut() {
        shield.time_since_last_hit += dt;
        
        if shield.time_since_last_hit >= shield.recharge_delay {
            shield.current = (shield.current + shield.recharge_rate * dt).min(shield.max);
        }
    }
}

/// Dead ship marker
#[derive(Component)]
pub struct DeadShip;

/// Ship death system
pub fn ship_death_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<(Entity, &Transform, Option<&Enemy>), With<DeadShip>>,
    player_query: Query<Entity, With<Player>>,
) {
    for (entity, transform, enemy) in query.iter() {
        // Spawn explosion effect
        crate::systems::effects::spawn_explosion(
            &mut commands,
            &mut meshes,
            &mut materials,
            transform.translation,
        );
        
        // Check if it's the player
        if player_query.contains(entity) {
            println!("[Combat System] Player died! Game Over");
            next_state.set(GameState::GameOver);
        } else if let Some(enemy) = enemy {
            println!("[Combat System] Enemy ship destroyed");
            // Spawn loot immediately
            crate::systems::resources_system::spawn_loot_for_enemy(
                &mut commands,
                &mut meshes,
                &mut materials,
                transform,
                enemy,
            );
        }
        
        commands.entity(entity).despawn_recursive();
    }
}

/// Marker for ships that should drop loot
#[derive(Component)]
pub struct ShouldSpawnLoot;

/// Energy recharge system
pub fn energy_recharge_system(
    time: Res<Time>,
    mut query: Query<&mut Energy>,
) {
    let dt = time.delta_seconds();
    
    for mut energy in query.iter_mut() {
        energy.current = (energy.current + energy.recharge_rate * dt).min(energy.max);
    }
}

