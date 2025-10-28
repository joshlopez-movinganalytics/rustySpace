use bevy::prelude::*;
use crate::components::ship::*;
use crate::components::combat::*;
use crate::components::ai::*;
use crate::resources::SpawnTimer;
use crate::utils::ship_builder;

/// Enemy spawner system
pub fn enemy_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Enemy>,
) {
    spawn_timer.0.tick(time.delta());
    
    if !spawn_timer.0.finished() {
        return;
    }
    
    // Limit number of enemies (increased from 10 to 15 for more action)
    let enemy_count = enemy_query.iter().count();
    if enemy_count >= 15 {
        return;
    }
    
    // Get player position if available
    let player_pos = player_query.iter().next().map(|t| t.translation).unwrap_or(Vec3::ZERO);
    
    // Random spawn position around player (increased distance for better spawning)
    let angle = rand::random::<f32>() * std::f32::consts::TAU;
    let distance = 120.0 + rand::random::<f32>() * 60.0;
    let spawn_pos = player_pos + Vec3::new(
        angle.cos() * distance,
        (rand::random::<f32>() - 0.5) * 30.0,
        angle.sin() * distance,
    );
    
    // Weighted random enemy type (more fighters and corvettes, fewer capital ships)
    let roll = rand::random::<f32>();
    let enemy_type = if roll < 0.4 {
        EnemyType::Fighter
    } else if roll < 0.7 {
        EnemyType::Corvette
    } else if roll < 0.9 {
        EnemyType::Frigate
    } else {
        EnemyType::CapitalShip
    };
    
    let (ship, ai, health, shield, weapon_mount, ship_type, color) = match enemy_type {
        EnemyType::Fighter => (
            Ship::fighter(),
            AIController::fighter(),
            Health { current: 50.0, max: 50.0 },
            Shield { current: 30.0, max: 30.0, recharge_rate: 5.0, recharge_delay: 2.0, time_since_last_hit: 10.0 },
            WeaponMount {
                weapons: vec![Weapon::laser()],
                current_weapon: 0,
            },
            ShipType::Fighter,
            Color::srgb(0.8, 0.2, 0.2),
        ),
        EnemyType::Corvette => (
            Ship::corvette(),
            AIController::corvette(),
            Health { current: 100.0, max: 100.0 },
            Shield { current: 80.0, max: 80.0, recharge_rate: 8.0, recharge_delay: 2.5, time_since_last_hit: 10.0 },
            WeaponMount {
                weapons: vec![Weapon::laser(), Weapon::plasma()],
                current_weapon: 0,
            },
            ShipType::Corvette,
            Color::srgb(0.7, 0.3, 0.2),
        ),
        EnemyType::Frigate => (
            Ship::frigate(),
            AIController::frigate(),
            Health { current: 200.0, max: 200.0 },
            Shield { current: 150.0, max: 150.0, recharge_rate: 12.0, recharge_delay: 3.0, time_since_last_hit: 10.0 },
            WeaponMount {
                weapons: vec![Weapon::laser(), Weapon::plasma(), Weapon::missile()],
                current_weapon: 0,
            },
            ShipType::Frigate,
            Color::srgb(0.6, 0.2, 0.3),
        ),
        EnemyType::CapitalShip => (
            Ship::capital_ship(),
            AIController::capital_ship(),
            Health { current: 500.0, max: 500.0 },
            Shield { current: 400.0, max: 400.0, recharge_rate: 20.0, recharge_delay: 4.0, time_since_last_hit: 10.0 },
            WeaponMount {
                weapons: vec![Weapon::laser(), Weapon::plasma(), Weapon::missile(), Weapon::railgun()],
                current_weapon: 0,
            },
            ShipType::CapitalShip,
            Color::srgb(0.5, 0.1, 0.2),
        ),
    };
    
    println!("[Spawning System] Spawning {:?} at position {:?}", enemy_type, spawn_pos);
    
    let enemy_ship = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(spawn_pos).looking_at(player_pos, Vec3::Y),
            ..default()
        },
        ship,
        ai,
        health,
        shield,
        weapon_mount,
        Energy {
            current: 100.0,
            max: 100.0,
            recharge_rate: 15.0,
        },
        Velocity(Vec3::ZERO),
        AngularVelocity(Vec3::ZERO),
        Enemy { enemy_type },
        Faction::Enemy,
    )).id();

    // Build modular ship visuals
    ship_builder::build_ship(
        &mut commands,
        &mut meshes,
        &mut materials,
        ship_type,
        enemy_ship,
        color,
    );
}

