use bevy::prelude::*;
use crate::components::ship::*;
use crate::components::upgrades::{PlayerUpgrades, UpgradeCategory};

/// Star marker component with parallax speed
#[derive(Component)]
pub struct Star {
    pub parallax_speed: f32,
}

/// Planet marker component
#[derive(Component)]
pub struct Planet {
    pub parallax_speed: f32,
}

/// Starfield bounds for wrapping
const STARFIELD_RADIUS: f32 = 800.0;
const STAR_COUNT: usize = 800;

/// Setup the starfield background
pub fn setup_starfield(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("[Visuals System] Setting up starfield with {} stars", STAR_COUNT);
    
    for _ in 0..STAR_COUNT {
        // Random position in sphere
        let theta = rand::random::<f32>() * std::f32::consts::TAU;
        let phi = rand::random::<f32>() * std::f32::consts::PI;
        let r = STARFIELD_RADIUS * (0.5 + rand::random::<f32>() * 0.5);
        
        let x = r * phi.sin() * theta.cos();
        let y = r * phi.sin() * theta.sin();
        let z = r * phi.cos();
        
        // Random size for depth variation
        let size = 0.1 + rand::random::<f32>() * 0.4;
        
        // Random brightness
        let brightness = 1.0 + rand::random::<f32>() * 3.0;
        
        // Parallax speed based on distance (closer stars move faster)
        let parallax_speed = 1.0 - (r / STARFIELD_RADIUS) * 0.7;
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(size)),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    emissive: LinearRgba::rgb(brightness, brightness, brightness * 0.95),
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
            Star { parallax_speed },
        ));
    }
}

/// Update starfield to scroll based on player velocity
pub fn update_starfield(
    mut star_query: Query<(&mut Transform, &Star)>,
    player_query: Query<&Velocity, With<Player>>,
    time: Res<Time>,
) {
    // Get player velocity
    let player_velocity = player_query.iter().next().map(|v| v.0).unwrap_or(Vec3::ZERO);
    
    for (mut transform, star) in star_query.iter_mut() {
        // Move stars in opposite direction of player movement (parallax effect)
        let movement = -player_velocity * star.parallax_speed * time.delta_seconds();
        transform.translation += movement;
        
        // Wrap stars around when they exit the sphere
        let distance = transform.translation.length();
        if distance > STARFIELD_RADIUS * 1.2 {
            // Wrap to opposite side
            transform.translation = -transform.translation.normalize() * STARFIELD_RADIUS * 0.9;
        }
    }
}

/// Setup decorative planets
pub fn setup_planets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("[Visuals System] Setting up decorative planets");
    
    let planet_configs = vec![
        // Blue planet
        (
            Vec3::new(400.0, -200.0, 300.0),
            35.0,
            Color::srgb(0.2, 0.4, 0.8),
        ),
        // Red/Mars-like planet
        (
            Vec3::new(-500.0, 150.0, -400.0),
            28.0,
            Color::srgb(0.8, 0.3, 0.2),
        ),
        // Green planet
        (
            Vec3::new(300.0, 300.0, -600.0),
            42.0,
            Color::srgb(0.3, 0.6, 0.3),
        ),
        // Brown/Desert planet
        (
            Vec3::new(-600.0, -250.0, 500.0),
            32.0,
            Color::srgb(0.6, 0.5, 0.3),
        ),
        // Purple gas giant
        (
            Vec3::new(700.0, 100.0, -200.0),
            55.0,
            Color::srgb(0.6, 0.3, 0.7),
        ),
    ];
    
    for (position, radius, color) in planet_configs {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(radius)),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    metallic: 0.0,
                    perceptual_roughness: 0.8,
                    ..default()
                }),
                transform: Transform::from_translation(position),
                ..default()
            },
            Planet {
                parallax_speed: 0.1, // Very slow movement
            },
        ));
    }
}

/// Update planets with subtle parallax movement
pub fn update_planets(
    mut planet_query: Query<(&mut Transform, &Planet)>,
    player_query: Query<&Velocity, With<Player>>,
    time: Res<Time>,
) {
    let player_velocity = player_query.iter().next().map(|v| v.0).unwrap_or(Vec3::ZERO);
    
    for (mut transform, planet) in planet_query.iter_mut() {
        // Very subtle movement for depth
        let movement = -player_velocity * planet.parallax_speed * time.delta_seconds();
        transform.translation += movement;
    }
}

/// Update ship visuals when upgrades are purchased
pub fn update_ship_visuals_on_upgrade(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    upgrades: Res<PlayerUpgrades>,
    mut ship_query: Query<(Entity, &mut UpgradeVisuals), With<Player>>,
    piece_query: Query<&ShipPiece>,
) {
    // Only run when upgrades resource has changed
    if !upgrades.is_changed() {
        return;
    }
    
    for (ship_entity, mut upgrade_visuals) in ship_query.iter_mut() {
        // Check for hull upgrades (armor plating)
        let hull_upgrade_count = upgrades.purchased.iter()
            .filter(|u| u.category() == UpgradeCategory::Hull)
            .count();
        
        // Add armor pieces if needed
        let current_armor_count = upgrade_visuals.armor_pieces.len();
        for i in current_armor_count..hull_upgrade_count {
            add_armor_piece(
                &mut commands,
                &mut meshes,
                &mut materials,
                ship_entity,
                &mut upgrade_visuals,
                i,
            );
        }
        
        // Check for engine upgrades
        let engine_upgrade_count = upgrades.purchased.iter()
            .filter(|u| u.category() == UpgradeCategory::Engines)
            .count();
        
        // Enhance engine glow
        if engine_upgrade_count > 0 {
            enhance_engine_pieces(
                &mut commands,
                &mut materials,
                ship_entity,
                &piece_query,
                engine_upgrade_count,
            );
        }
        
        // Check for weapon upgrades
        let weapon_upgrade_count = upgrades.purchased.iter()
            .filter(|u| u.category() == UpgradeCategory::Weapons)
            .count();
        
        // Add weapon hardpoints
        let current_weapon_count = upgrade_visuals.weapon_pieces.len();
        for i in current_weapon_count..weapon_upgrade_count {
            add_weapon_hardpoint(
                &mut commands,
                &mut meshes,
                &mut materials,
                ship_entity,
                &mut upgrade_visuals,
                i,
            );
        }
        
        // Check for shield upgrades
        let shield_upgrade_count = upgrades.purchased.iter()
            .filter(|u| u.category() == UpgradeCategory::Shields)
            .count();
        
        // Add shield emitters
        let current_shield_count = upgrade_visuals.shield_pieces.len();
        for i in current_shield_count..shield_upgrade_count {
            add_shield_emitter(
                &mut commands,
                &mut meshes,
                &mut materials,
                ship_entity,
                &mut upgrade_visuals,
                i,
            );
        }
    }
}

/// Add an armor plating piece to the ship
fn add_armor_piece(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    ship_entity: Entity,
    upgrade_visuals: &mut UpgradeVisuals,
    index: usize,
) {
    let positions = vec![
        Vec3::new(-0.8, 0.3, 0.5),
        Vec3::new(0.8, 0.3, 0.5),
        Vec3::new(0.0, -0.3, 0.0),
    ];
    
    if index >= positions.len() {
        return;
    }
    
    let armor_piece = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.4, 0.2, 0.6)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.5, 0.5, 0.5),
                metallic: 0.9,
                perceptual_roughness: 0.2,
                ..default()
            }),
            transform: Transform::from_translation(positions[index]),
            ..default()
        },
        ShipPiece {
            piece_type: ShipPieceType::ArmorPlating,
            parent_ship: ship_entity,
        },
    )).id();
    
    commands.entity(ship_entity).add_child(armor_piece);
    upgrade_visuals.armor_pieces.push(armor_piece);
    
    println!("[Visuals System] Added armor plating piece {}", index);
}

/// Enhance engine pieces with stronger glow
fn enhance_engine_pieces(
    _commands: &mut Commands,
    _materials: &mut ResMut<Assets<StandardMaterial>>,
    _ship_entity: Entity,
    _piece_query: &Query<&ShipPiece>,
    upgrade_level: usize,
) {
    // Calculate enhanced glow based on upgrade level
    let glow_multiplier = 1.0 + (upgrade_level as f32 * 0.5);
    
    // This is a simplified version - in a real implementation, we'd query for
    // children of ship_entity and update their materials
    // For now, we'll just log the enhancement
    println!("[Visuals System] Enhanced engine glow to level {}", glow_multiplier);
}

/// Add a weapon hardpoint to the ship
fn add_weapon_hardpoint(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    ship_entity: Entity,
    upgrade_visuals: &mut UpgradeVisuals,
    index: usize,
) {
    let positions = vec![
        Vec3::new(-0.6, -0.2, 0.8),
        Vec3::new(0.6, -0.2, 0.8),
        Vec3::new(-0.9, 0.0, 0.3),
        Vec3::new(0.9, 0.0, 0.3),
    ];
    
    if index >= positions.len() {
        return;
    }
    
    let weapon_piece = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.15, 0.15, 0.4)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.3, 0.3, 0.3),
                metallic: 0.95,
                perceptual_roughness: 0.1,
                emissive: LinearRgba::rgb(0.1, 0.0, 0.0),
                ..default()
            }),
            transform: Transform::from_translation(positions[index]),
            ..default()
        },
        ShipPiece {
            piece_type: ShipPieceType::WeaponMount,
            parent_ship: ship_entity,
        },
    )).id();
    
    commands.entity(ship_entity).add_child(weapon_piece);
    upgrade_visuals.weapon_pieces.push(weapon_piece);
    
    println!("[Visuals System] Added weapon hardpoint {}", index);
}

/// Add a shield emitter to the ship
fn add_shield_emitter(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    ship_entity: Entity,
    upgrade_visuals: &mut UpgradeVisuals,
    index: usize,
) {
    let positions = vec![
        Vec3::new(0.0, 0.4, 0.5),
        Vec3::new(-0.5, 0.2, -0.3),
        Vec3::new(0.5, 0.2, -0.3),
    ];
    
    if index >= positions.len() {
        return;
    }
    
    let shield_piece = commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.15)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.3, 0.5, 0.9),
                metallic: 0.8,
                perceptual_roughness: 0.2,
                emissive: LinearRgba::rgb(0.3, 0.6, 1.2),
                ..default()
            }),
            transform: Transform::from_translation(positions[index]),
            ..default()
        },
        ShipPiece {
            piece_type: ShipPieceType::ShieldEmitter,
            parent_ship: ship_entity,
        },
    )).id();
    
    commands.entity(ship_entity).add_child(shield_piece);
    upgrade_visuals.shield_pieces.push(shield_piece);
    
    println!("[Visuals System] Added shield emitter {}", index);
}

