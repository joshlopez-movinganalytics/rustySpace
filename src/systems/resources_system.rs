use bevy::prelude::*;
use crate::components::resources::*;
use crate::components::ship::{Player, Velocity};
use crate::systems::combat::ShouldSpawnLoot;
use crate::components::ai::Enemy;

/// Marker component for loot entities
#[derive(Component)]
pub struct LootVisual {
    pub lifetime: f32,
    pub rotation_speed: f32,
}

/// Loot collection system with magnetic pull effect
pub fn loot_collection_system(
    mut commands: Commands,
    time: Res<Time>,
    mut inventory: ResMut<Inventory>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    mut loot_query: Query<(Entity, &mut Transform, &Loot, &mut Velocity), Without<Player>>,
    galaxy: Option<Res<crate::resources::Galaxy>>,
) {
    let collection_radius = 3.0; // Close range for actual collection
    let attraction_radius = 15.0; // Wider range for magnetic pull
    let attraction_strength = 25.0;
    let dt = time.delta_seconds();
    
    // Get resource multipliers from current system
    let resource_multipliers = galaxy
        .as_ref()
        .and_then(|g| g.current_system())
        .map(|s| &s.resource_multipliers);
    
    for player_transform in player_query.iter() {
        for (loot_entity, mut loot_transform, loot, mut velocity) in loot_query.iter_mut() {
            let to_player = player_transform.translation - loot_transform.translation;
            let distance = to_player.length();
            
            // Magnetic pull when within attraction radius
            if distance < attraction_radius && distance > collection_radius {
                let pull_direction = to_player.normalize();
                let pull_force = attraction_strength * (1.0 - distance / attraction_radius);
                velocity.0 += pull_direction * pull_force * dt;
                
                // Apply velocity with damping
                loot_transform.translation += velocity.0 * dt;
                velocity.0 *= 0.95; // Damping
            }
            
            // Collect when close enough
            if distance < collection_radius {
                // Apply system resource multipliers
                let multiplier = if let Some(multipliers) = resource_multipliers {
                    match loot.resource_type {
                        ResourceType::ScrapMetal => multipliers.scrap_metal,
                        ResourceType::EnergyCores => multipliers.energy_cores,
                        ResourceType::RareMinerals => multipliers.rare_minerals,
                        ResourceType::TechComponents => multipliers.tech_components,
                    }
                } else {
                    1.0
                };
                
                let amount = (loot.amount as f32 * multiplier).round() as u32;
                inventory.add_resource(loot.resource_type, amount);
                
                let resource_name = match loot.resource_type {
                    ResourceType::ScrapMetal => "Scrap Metal",
                    ResourceType::EnergyCores => "Energy Core",
                    ResourceType::RareMinerals => "Rare Mineral",
                    ResourceType::TechComponents => "Tech Component",
                };
                println!("[Resources System] Collected {} x{}", resource_name, loot.amount);
                
                // Spawn collection effect
                spawn_collection_effect(&mut commands, &mut meshes, &mut materials, loot_transform.translation, loot.resource_type);
                
                commands.entity(loot_entity).despawn();
            }
        }
    }
}

/// Spawn a brief visual effect when collecting loot
fn spawn_collection_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    resource_type: ResourceType,
) {
    let color = match resource_type {
        ResourceType::ScrapMetal => Color::srgb(0.7, 0.7, 0.7),
        ResourceType::EnergyCores => Color::srgb(0.2, 0.8, 1.0),
        ResourceType::RareMinerals => Color::srgb(0.8, 0.2, 0.8),
        ResourceType::TechComponents => Color::srgb(1.0, 0.8, 0.2),
    };
    
    // Spawn multiple small particles
    for i in 0..8 {
        let angle = (i as f32 / 8.0) * std::f32::consts::TAU;
        let offset = Vec3::new(angle.cos() * 0.5, 0.0, angle.sin() * 0.5);
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(0.1)),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    emissive: color.into(),
                    ..default()
                }),
                transform: Transform::from_translation(position + offset),
                ..default()
            },
            CollectionParticle { lifetime: 0.5 },
            Velocity(Vec3::new(offset.x * 3.0, 2.0, offset.z * 3.0)),
        ));
    }
}

/// Marker for collection particles
#[derive(Component)]
pub struct CollectionParticle {
    pub lifetime: f32,
}

/// Spawn loot for a specific enemy - called directly when enemy dies
pub fn spawn_loot_for_enemy(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    transform: &Transform,
    enemy: &Enemy,
) {
        // Better drop rates based on enemy type
        let (loot_count, amount_multiplier) = match enemy.enemy_type {
            crate::components::ai::EnemyType::Fighter => (2, 1),      // 2 pieces, 1 each
            crate::components::ai::EnemyType::Corvette => (3, 2),     // 3 pieces, 2 each
            crate::components::ai::EnemyType::Frigate => (4, 3),      // 4 pieces, 3 each
            crate::components::ai::EnemyType::CapitalShip => (6, 5),  // 6 pieces, 5 each
        };
        
        for i in 0..loot_count {
            // Weighted random for resource type (more scrap and energy, less rare stuff)
            let roll = rand::random::<f32>();
            let resource_type = if roll < 0.4 {
                ResourceType::ScrapMetal  // 40% chance
            } else if roll < 0.7 {
                ResourceType::EnergyCores  // 30% chance
            } else if roll < 0.9 {
                ResourceType::RareMinerals  // 20% chance
            } else {
                ResourceType::TechComponents  // 10% chance
            };
            
            // Spread loot in a circle around the ship
            let angle = (i as f32 / loot_count as f32) * std::f32::consts::TAU + rand::random::<f32>() * 0.5;
            let radius = 3.0 + rand::random::<f32>() * 2.0;
            let offset = Vec3::new(
                angle.cos() * radius,
                (rand::random::<f32>() - 0.5) * 2.0,
                angle.sin() * radius,
            );
            
            let (color, emissive_color, mesh_type) = match resource_type {
                ResourceType::ScrapMetal => (
                    Color::srgb(0.7, 0.7, 0.7), 
                    Color::srgb(0.9, 0.9, 0.9), 
                    0
                ), // Gray cube
                ResourceType::EnergyCores => (
                    Color::srgb(0.2, 0.8, 1.0), 
                    Color::srgb(0.4, 1.0, 1.0), 
                    1
                ), // Blue sphere
                ResourceType::RareMinerals => (
                    Color::srgb(0.8, 0.2, 0.8), 
                    Color::srgb(1.0, 0.4, 1.0), 
                    2
                ), // Purple octahedron-ish
                ResourceType::TechComponents => (
                    Color::srgb(1.0, 0.8, 0.2), 
                    Color::srgb(1.0, 1.0, 0.4), 
                    3
                ), // Gold/yellow
            };
            
            let mesh = match mesh_type {
                0 => meshes.add(Cuboid::new(0.5, 0.5, 0.5)),  // Scrap
                1 => meshes.add(Sphere::new(0.4)),            // Energy
                2 => meshes.add(Sphere::new(0.35)),           // Minerals (could be different shape)
                _ => meshes.add(Cuboid::new(0.4, 0.4, 0.4)),  // Tech (could be different shape)
            };
            
            // Random rotation speed for visual effect
            let rotation_speed = 1.0 + rand::random::<f32>() * 2.0;
            
            commands.spawn((
                PbrBundle {
                    mesh,
                    material: materials.add(StandardMaterial {
                        base_color: color,
                        emissive: emissive_color.into(),
                        metallic: 0.3,
                        perceptual_roughness: 0.5,
                        ..default()
                    }),
                    transform: Transform::from_translation(transform.translation + offset),
                    ..default()
                },
                Loot {
                    resource_type,
                    amount: amount_multiplier,
                },
                LootVisual {
                    lifetime: 60.0,  // Loot disappears after 60 seconds if not collected
                    rotation_speed,
                },
                Velocity(Vec3::new(
                    (rand::random::<f32>() - 0.5) * 2.0,
                    rand::random::<f32>() * 1.0,
                    (rand::random::<f32>() - 0.5) * 2.0,
                )),
            ));
        }
}

/// Old spawn loot system - now deprecated, loot spawns directly in death system
pub fn spawn_loot_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Transform, &Enemy), With<ShouldSpawnLoot>>,
) {
    // This system is kept for backwards compatibility but is no longer used
    // Loot is now spawned directly in the ship_death_system
    for (entity, transform, enemy) in query.iter() {
        spawn_loot_for_enemy(&mut commands, &mut meshes, &mut materials, transform, enemy);
        commands.entity(entity).remove::<ShouldSpawnLoot>();
    }
}

/// Animate loot (rotation and bobbing)
pub fn animate_loot_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut LootVisual, &mut Velocity)>,
) {
    let dt = time.delta_seconds();
    
    for (entity, mut transform, mut visual, mut velocity) in query.iter_mut() {
        // Rotate loot
        transform.rotate_y(visual.rotation_speed * dt);
        
        // Apply bobbing motion
        let bob_height = (time.elapsed_seconds() * 2.0).sin() * 0.5;
        
        // Slow down the loot over time
        velocity.0 *= 0.95;
        
        // Add slight upward drift
        velocity.0.y = bob_height * 0.3;
        
        // Update lifetime
        visual.lifetime -= dt;
        
        // Despawn if too old
        if visual.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Update collection particles
pub fn update_collection_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut CollectionParticle, &Velocity)>,
) {
    let dt = time.delta_seconds();
    
    for (entity, mut transform, mut particle, velocity) in query.iter_mut() {
        particle.lifetime -= dt;
        
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            // Move particle
            transform.translation += velocity.0 * dt;
            
            // Fade out (scale down)
            let scale = particle.lifetime * 2.0;
            transform.scale = Vec3::splat(scale.clamp(0.1, 1.0));
        }
    }
}

