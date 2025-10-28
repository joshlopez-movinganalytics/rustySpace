use bevy::prelude::*;
use crate::components::resources::*;
use crate::components::ship::Player;
use crate::systems::combat::ShouldSpawnLoot;
use crate::components::ai::Enemy;

/// Loot collection system
pub fn loot_collection_system(
    mut commands: Commands,
    mut inventory: ResMut<Inventory>,
    player_query: Query<&Transform, With<Player>>,
    loot_query: Query<(Entity, &Transform, &Loot)>,
) {
    let collection_radius = 5.0;
    
    for player_transform in player_query.iter() {
        for (loot_entity, loot_transform, loot) in loot_query.iter() {
            let distance = player_transform.translation.distance(loot_transform.translation);
            
            if distance < collection_radius {
                inventory.add_resource(loot.resource_type, loot.amount);
                println!("[Resources System] Collected {:?} x{}", loot.resource_type, loot.amount);
                commands.entity(loot_entity).despawn();
            }
        }
    }
}

/// Spawn loot when ships are destroyed
pub fn spawn_loot_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Transform, &Enemy), With<ShouldSpawnLoot>>,
) {
    for (entity, transform, enemy) in query.iter() {
        let loot_count = match enemy.enemy_type {
            crate::components::ai::EnemyType::Fighter => 1,
            crate::components::ai::EnemyType::Corvette => 2,
            crate::components::ai::EnemyType::Frigate => 3,
            crate::components::ai::EnemyType::CapitalShip => 5,
        };
        
        for _ in 0..loot_count {
            let resource_type = match rand::random::<u32>() % 4 {
                0 => ResourceType::ScrapMetal,
                1 => ResourceType::EnergyCores,
                2 => ResourceType::RareMinerals,
                _ => ResourceType::TechComponents,
            };
            
            let offset = Vec3::new(
                (rand::random::<f32>() - 0.5) * 5.0,
                (rand::random::<f32>() - 0.5) * 5.0,
                (rand::random::<f32>() - 0.5) * 5.0,
            );
            
            let color = match resource_type {
                ResourceType::ScrapMetal => Color::srgb(0.7, 0.7, 0.7),
                ResourceType::EnergyCores => Color::srgb(0.2, 0.8, 1.0),
                ResourceType::RareMinerals => Color::srgb(0.8, 0.2, 0.8),
                ResourceType::TechComponents => Color::srgb(1.0, 0.8, 0.2),
            };
            
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Sphere::new(0.3)),
                    material: materials.add(StandardMaterial {
                        base_color: color,
                        emissive: color.into(),
                        ..default()
                    }),
                    transform: Transform::from_translation(transform.translation + offset),
                    ..default()
                },
                Loot {
                    resource_type,
                    amount: 1,
                },
            ));
        }
        
        commands.entity(entity).remove::<ShouldSpawnLoot>();
    }
}

