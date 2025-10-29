use bevy::prelude::*;

/// Explosion effect marker
#[derive(Component)]
pub struct Explosion {
    pub lifetime: f32,
    pub max_lifetime: f32,
}

/// Shield hit effect marker
#[derive(Component)]
pub struct ShieldHitEffect {
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub impact_point: Vec3,
}

/// Hull spark effect marker
#[derive(Component)]
pub struct HullSparkEffect {
    pub lifetime: f32,
    pub max_lifetime: f32,
}

/// Spawn an explosion effect
pub fn spawn_explosion(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    // Central bright sphere
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(2.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.8, 0.2),
                emissive: Color::srgb(10.0, 5.0, 1.0).into(),
                ..default()
            }),
            transform: Transform::from_translation(position),
            ..default()
        },
        Explosion {
            lifetime: 0.0,
            max_lifetime: 0.5,
        },
    ));
    
    // Spawn debris particles
    for _ in 0..10 {
        let offset = Vec3::new(
            (rand::random::<f32>() - 0.5) * 2.0,
            (rand::random::<f32>() - 0.5) * 2.0,
            (rand::random::<f32>() - 0.5) * 2.0,
        );
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.3, 0.3, 0.3)),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.7, 0.3, 0.1),
                    metallic: 0.8,
                    ..default()
                }),
                transform: Transform::from_translation(position + offset),
                ..default()
            },
            crate::components::ship::Velocity(offset.normalize() * 20.0),
            Explosion {
                lifetime: 0.0,
                max_lifetime: 1.0,
            },
        ));
    }
}

/// Spawn a shield hit effect at impact point
pub fn spawn_shield_hit_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    ship_position: Vec3,
    impact_point: Vec3,
) {
    // Create an oval shield bubble around the ship
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(4.0)), // Large enough to encompass ship
            material: materials.add(StandardMaterial {
                base_color: Color::srgba(0.2, 0.5, 1.0, 0.3),
                emissive: Color::srgb(0.5, 1.0, 2.0).into(),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_translation(ship_position)
                .with_scale(Vec3::new(1.2, 1.0, 1.2)), // Slightly oval
            ..default()
        },
        ShieldHitEffect {
            lifetime: 0.0,
            max_lifetime: 0.2,
            impact_point,
        },
    ));
    
    // Add a bright flash at the impact point
    let direction = (impact_point - ship_position).normalize();
    let flash_pos = ship_position + direction * 3.5;
    
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.5)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.5, 1.0, 2.0),
                emissive: Color::srgb(2.0, 4.0, 8.0).into(),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(flash_pos),
            ..default()
        },
        ShieldHitEffect {
            lifetime: 0.0,
            max_lifetime: 0.15,
            impact_point,
        },
    ));
}

/// Spawn hull spark effect when hull is hit
pub fn spawn_hull_spark_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    impact_point: Vec3,
    projectile_direction: Vec3,
) {
    // Calculate reflection direction (sparks bounce off at impact angle)
    let reflection_dir = projectile_direction.normalize();
    
    // Spawn multiple sparks flying away from impact
    let spark_count = rand::random::<usize>() % 5 + 5; // 5-9 sparks
    
    for _ in 0..spark_count {
        // Randomize spark direction around the reflection with some spread
        let spread_x = (rand::random::<f32>() - 0.5) * 2.0;
        let spread_y = (rand::random::<f32>() - 0.5) * 2.0;
        let spread_z = (rand::random::<f32>() - 0.5) * 2.0;
        let spread = Vec3::new(spread_x, spread_y, spread_z);
        let spark_dir = (reflection_dir + spread).normalize();
        
        // Vary spark colors (orange, yellow, red-orange)
        let color_var = rand::random::<f32>();
        let (base_color, emissive_color) = if color_var < 0.33 {
            (Color::srgb(1.0, 0.6, 0.1), Color::srgb(3.0, 1.8, 0.3)) // Orange
        } else if color_var < 0.66 {
            (Color::srgb(1.0, 0.9, 0.2), Color::srgb(3.0, 2.7, 0.6)) // Yellow
        } else {
            (Color::srgb(1.0, 0.4, 0.1), Color::srgb(3.0, 1.2, 0.3)) // Red-orange
        };
        
        let spark_speed = 8.0 + rand::random::<f32>() * 8.0; // Vary speed
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(0.12)),
                material: materials.add(StandardMaterial {
                    base_color,
                    emissive: emissive_color.into(),
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_translation(impact_point),
                ..default()
            },
            crate::components::ship::Velocity(spark_dir * spark_speed),
            HullSparkEffect {
                lifetime: 0.0,
                max_lifetime: 0.3 + rand::random::<f32>() * 0.3, // 0.3-0.6 seconds
            },
        ));
    }
    
    // Add a small flash at impact point
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.4)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.8, 0.3),
                emissive: Color::srgb(3.0, 2.0, 0.5).into(),
                unlit: true,
                ..default()
            }),
            transform: Transform::from_translation(impact_point),
            ..default()
        },
        HullSparkEffect {
            lifetime: 0.0,
            max_lifetime: 0.1,
        },
    ));
}

/// Spawn a shield break effect when shields go to zero
pub fn spawn_shield_break_effect(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    // Large collapsing shield sphere
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(5.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgba(0.2, 0.5, 1.0, 0.5),
                emissive: Color::srgb(0.5, 1.5, 3.0).into(),
                alpha_mode: AlphaMode::Blend,
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::new(1.2, 1.0, 1.2)),
            ..default()
        },
        ShieldHitEffect {
            lifetime: 0.0,
            max_lifetime: 0.4,
            impact_point: position,
        },
    ));
    
    // Spawn sparks flying outward
    for i in 0..12 {
        let angle = (i as f32 / 12.0) * std::f32::consts::TAU;
        let direction = Vec3::new(angle.cos(), (rand::random::<f32>() - 0.5) * 0.5, angle.sin()).normalize();
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(0.2)),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.3, 0.8, 1.5),
                    emissive: Color::srgb(1.0, 2.0, 4.0).into(),
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_translation(position + direction * 2.0),
                ..default()
            },
            crate::components::ship::Velocity(direction * 15.0),
            ShieldHitEffect {
                lifetime: 0.0,
                max_lifetime: 0.6,
                impact_point: position,
            },
        ));
    }
}

/// Update explosion effects
pub fn update_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Explosion, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    
    for (entity, mut explosion, mut transform) in query.iter_mut() {
        explosion.lifetime += dt;
        
        if explosion.lifetime >= explosion.max_lifetime {
            commands.entity(entity).despawn();
        } else {
            // Fade out and expand
            let progress = explosion.lifetime / explosion.max_lifetime;
            let scale = 1.0 + progress * 3.0;
            transform.scale = Vec3::splat(scale);
        }
    }
}

/// Update hull spark effects
pub fn update_hull_spark_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut HullSparkEffect, &mut Transform, Option<&Handle<StandardMaterial>>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dt = time.delta_seconds();
    
    for (entity, mut spark, mut transform, material_handle) in query.iter_mut() {
        spark.lifetime += dt;
        
        if spark.lifetime >= spark.max_lifetime {
            commands.entity(entity).despawn();
        } else {
            let progress = spark.lifetime / spark.max_lifetime;
            
            // Fade out sparks
            if let Some(material_handle) = material_handle {
                if let Some(material) = materials.get_mut(material_handle) {
                    // Fade out
                    let fade = 1.0 - progress;
                    material.base_color.set_alpha(fade);
                    
                    // Reduce emissive
                    let current = material.emissive;
                    material.emissive = (current * fade).into();
                }
            }
            
            // Shrink sparks as they fade
            let scale = 1.0 - progress * 0.5;
            transform.scale = Vec3::splat(scale);
        }
    }
}

/// Update shield hit effects
pub fn update_shield_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ShieldHitEffect, &mut Transform, Option<&mut Handle<StandardMaterial>>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dt = time.delta_seconds();
    
    for (entity, mut effect, mut transform, material_handle) in query.iter_mut() {
        effect.lifetime += dt;
        
        if effect.lifetime >= effect.max_lifetime {
            commands.entity(entity).despawn();
        } else {
            let progress = effect.lifetime / effect.max_lifetime;
            
            // Fade out and scale based on effect type
            if let Some(material_handle) = material_handle {
                if let Some(material) = materials.get_mut(&*material_handle) {
                    // Fade alpha
                    let alpha = 1.0 - progress;
                    material.base_color.set_alpha(alpha * 0.3);
                    
                    // Reduce emissive
                    let emissive_strength = 1.0 - progress;
                    if progress < 0.5 {
                        // Shield bubble expands then fades
                        let scale = 1.0 + progress * 0.3;
                        transform.scale = Vec3::new(scale * 1.2, scale * 1.0, scale * 1.2);
                    } else {
                        // Collapse effect for shield break
                        let collapse = 1.0 - (progress - 0.5) * 2.0;
                        transform.scale *= collapse.max(0.5);
                    }
                    
                    // Update emissive based on remaining strength
                    let current = material.emissive;
                    material.emissive = (current * emissive_strength).into();
                }
            }
        }
    }
}

