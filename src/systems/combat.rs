use bevy::prelude::*;
use crate::components::ship::*;
use crate::components::combat::*;

/// Weapon firing system
pub fn weapon_firing_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &Transform, &Velocity, &mut WeaponMount, &mut Energy), With<Player>>,
) {
    let dt = time.delta_seconds();
    
    for (entity, transform, velocity, mut weapon_mount, mut energy) in query.iter_mut() {
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
                if weapon.cooldown_timer <= 0.0 && energy.current >= weapon.energy_cost {
                    fire_weapon(&mut commands, &mut meshes, &mut materials, entity, transform, velocity, weapon, &mut energy, false);
                }
            }
        }
        
        // Alt-fire (Right Mouse)
        if mouse.just_pressed(MouseButton::Right) {
            if let Some(weapon) = weapon_mount.weapons.get_mut(current_weapon_idx) {
                // Burst fire for laser
                if weapon.weapon_type == WeaponType::Laser && weapon.cooldown_timer <= 0.0 && energy.current >= weapon.energy_cost * 3.0 {
                    // Fire 3-shot burst
                    for i in 0..3 {
                        let delay = i as f32 * 0.05;
                        // For simplicity, fire all 3 immediately with slight spread
                        fire_weapon(&mut commands, &mut meshes, &mut materials, entity, transform, velocity, weapon, &mut energy, true);
                    }
                    weapon.cooldown_timer = 1.0 / weapon.fire_rate;
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
    is_burst: bool,
) {
    weapon.cooldown_timer = 1.0 / weapon.fire_rate;
    energy.current -= weapon.energy_cost;
    
    let forward = transform.forward();
    let projectile_pos = transform.translation + forward.as_vec3() * 3.0;
    
    // Add spread
    let spread_mult = if is_burst { 1.5 } else { 1.0 };
    let spread_x = (rand::random::<f32>() - 0.5) * weapon.spread * spread_mult;
    let spread_y = (rand::random::<f32>() - 0.5) * weapon.spread * spread_mult;
    let spread_rotation = Quat::from_euler(EulerRot::XYZ, spread_y, spread_x, 0.0);
    let projectile_direction = (spread_rotation * forward.as_vec3()).normalize();
    
    let projectile_velocity = projectile_direction * weapon.projectile_speed + velocity.0;
    
    let (mesh, color) = get_weapon_visual(weapon.weapon_type, meshes);
    
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color: color,
                emissive: color.into(),
                ..default()
            }),
            transform: Transform::from_translation(projectile_pos)
                .with_rotation(transform.rotation),
            ..default()
        },
        Projectile {
            damage: weapon.damage,
            lifetime: 5.0,
            owner,
            weapon_type: weapon.weapon_type,
            shield_damage_multiplier: weapon.shield_damage_multiplier,
            piercing: false,
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
    
    let forward = transform.forward();
    let projectile_pos = transform.translation + forward.as_vec3() * 3.0;
    
    // Extra spread for shotgun effect
    let spread_x = (rand::random::<f32>() - 0.5) * 0.15;
    let spread_y = (rand::random::<f32>() - 0.5) * 0.15;
    let spread_rotation = Quat::from_euler(EulerRot::XYZ, spread_y, spread_x, 0.0);
    let projectile_direction = (spread_rotation * forward.as_vec3()).normalize();
    
    let projectile_velocity = projectile_direction * weapon.projectile_speed + velocity.0;
    
    let (mesh, color) = get_weapon_visual(weapon.weapon_type, meshes);
    
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color: color,
                emissive: color.into(),
                ..default()
            }),
            transform: Transform::from_translation(projectile_pos)
                .with_rotation(transform.rotation),
            ..default()
        },
        Projectile {
            damage: weapon.damage * 0.6, // Reduced damage per pellet
            lifetime: 3.0,
            owner,
            weapon_type: weapon.weapon_type,
            shield_damage_multiplier: weapon.shield_damage_multiplier,
            piercing: false,
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
        let projectile_velocity = forward.as_vec3() * weapon.projectile_speed + velocity.0;
        
        let (mesh, color) = get_weapon_visual(WeaponType::Missile, meshes);
        
        commands.spawn((
            PbrBundle {
                mesh,
                material: materials.add(StandardMaterial {
                    base_color: color,
                    emissive: color.into(),
                    ..default()
                }),
                transform: Transform::from_translation(projectile_pos)
                    .with_rotation(transform.rotation),
                ..default()
            },
            Projectile {
                damage: weapon.damage * 0.5,
                lifetime: 5.0,
                owner,
                weapon_type: weapon.weapon_type,
                shield_damage_multiplier: weapon.shield_damage_multiplier,
                piercing: false,
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
    let projectile_velocity = forward.as_vec3() * weapon.projectile_speed + velocity.0;
    
    let (mesh, color) = get_weapon_visual(WeaponType::Railgun, meshes);
    
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color: color,
                emissive: color.into(),
                ..default()
            }),
            transform: Transform::from_translation(projectile_pos)
                .with_rotation(transform.rotation),
            ..default()
        },
        Projectile {
            damage: weapon.damage * 1.3,
            lifetime: 8.0,
            owner,
            weapon_type: weapon.weapon_type,
            shield_damage_multiplier: weapon.shield_damage_multiplier,
            piercing: true, // Pierces through enemies
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
    let projectile_velocity = forward.as_vec3() * weapon.projectile_speed + velocity.0;
    
    let size = 0.3 + charge * 0.3;
    let mesh = meshes.add(Sphere::new(size));
    let color = Color::srgb(0.2 + charge * 0.3, 1.0, 0.2);
    
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color: color,
                emissive: color.into(),
                ..default()
            }),
            transform: Transform::from_translation(projectile_pos)
                .with_rotation(transform.rotation),
            ..default()
        },
        Projectile {
            damage: weapon.damage * (1.0 + charge),
            lifetime: 5.0,
            owner,
            weapon_type: weapon.weapon_type,
            shield_damage_multiplier: weapon.shield_damage_multiplier,
            piercing: false,
        },
        Velocity(projectile_velocity),
        Faction::Player,
    ));
}

fn get_weapon_visual(weapon_type: WeaponType, meshes: &mut ResMut<Assets<Mesh>>) -> (Handle<Mesh>, Color) {
    match weapon_type {
        WeaponType::Laser => (
            meshes.add(Capsule3d::new(0.1, 1.0)),
            Color::srgb(1.0, 0.2, 0.2),
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
    mut query: Query<(&Velocity, &mut Transform), With<Projectile>>,
) {
    let dt = time.delta_seconds();
    
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * dt;
        
        // Orient projectile in direction of travel
        if velocity.0.length() > 0.1 {
            transform.look_to(velocity.0, Vec3::Y);
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

/// Projectile collision system
pub fn projectile_collision_system(
    mut commands: Commands,
    projectiles: Query<(Entity, &Transform, &Projectile, &Faction)>,
    mut ships: Query<(Entity, &Transform, &mut Health, &mut Shield, &Faction), Without<Projectile>>,
) {
    for (proj_entity, proj_transform, projectile, proj_faction) in projectiles.iter() {
        let mut hit_something = false;
        
        for (ship_entity, ship_transform, mut health, mut shield, ship_faction) in ships.iter_mut() {
            // Don't hit own faction or owner
            if proj_faction == ship_faction || projectile.owner == ship_entity {
                continue;
            }
            
            // Simple sphere collision
            let distance = proj_transform.translation.distance(ship_transform.translation);
            if distance < 2.0 {
                // Apply damage with shield multiplier
                let hull_damage = projectile.damage;
                let shield_damage = projectile.damage * projectile.shield_damage_multiplier;
                
                // Hit shields first
                if shield.current > 0.0 {
                    shield.current -= shield_damage;
                    shield.time_since_last_hit = 0.0;
                    if shield.current < 0.0 {
                        // Overflow damage to hull (not multiplied)
                        health.current += shield.current / projectile.shield_damage_multiplier;
                        shield.current = 0.0;
                    }
                } else {
                    health.current -= hull_damage;
                }
                
                hit_something = true;
                
                // Only despawn if not piercing, or break after hit
                if !projectile.piercing {
                    commands.entity(proj_entity).despawn();
                    break;
                }
            }
        }
        
        // Piercing projectiles still despawn after hitting, just not on first hit
        // This is handled by the flag above
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
    query: Query<(Entity, &Transform), With<DeadShip>>,
    player_query: Query<Entity, With<Player>>,
) {
    for (entity, transform) in query.iter() {
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
            // TODO: Handle game over
        } else {
            println!("[Combat System] Enemy ship destroyed");
            // Spawn loot before despawning
            commands.entity(entity).try_insert(ShouldSpawnLoot);
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

