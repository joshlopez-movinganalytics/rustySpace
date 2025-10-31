use bevy::prelude::*;

/// Explosion effect marker
#[derive(Component)]
pub struct Explosion {
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub explosion_type: ExplosionType,
    pub start_delay: f32, // Delay before this explosion appears
    pub main_progress: f32, // Independent progress for main explosion (only increments when visible)
}

/// Type of explosion for different stages
#[derive(Clone, Copy)]
pub enum ExplosionType {
    SmallFire,    // Small initial explosions (fire colors)
    MainFire,     // Large main explosion (fire colors)
    Smoke,        // Smoke clouds (gray/black)
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

/// Spawn an explosion effect with multiple stages
pub fn spawn_explosion(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    // Stage 1: Multiple smaller explosions around the ship - last 1-3 seconds
    let small_explosion_count = 8;
    for i in 0..small_explosion_count {
        let angle = (i as f32 / small_explosion_count as f32) * std::f32::consts::TAU;
        let radius = 2.0 + rand::random::<f32>() * 3.0; // Spread further around ship
        let offset = Vec3::new(
            angle.cos() * radius,
            (rand::random::<f32>() - 0.5) * 2.0,
            angle.sin() * radius,
        );
        
        let small_pos = position + offset;
        let size = 0.3 + rand::random::<f32>() * 1.0; // Larger small explosions
        
        // Fire gradient: bright yellow/orange at center, red at edges
        let fire_intensity = rand::random::<f32>();
        let base_color = if fire_intensity < 0.5 {
            Color::srgb(1.0, 0.9, 0.3) // Bright yellow
        } else {
            Color::srgb(1.0, 0.5, 0.1) // Orange-red
        };
        
        // Random lifetime between 1-3 seconds
        let lifetime = 1.0 + rand::random::<f32>() * 2.0;
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(size)),
                material: materials.add(StandardMaterial {
                    base_color,
                    emissive: LinearRgba::from(base_color) * 15.0, // Bright enough for bloom
                    ..default()
                }),
                transform: Transform::from_translation(small_pos),
                ..default()
            },
            Explosion {
                lifetime: 0.0,
                max_lifetime: lifetime, // 1-3 seconds
                explosion_type: ExplosionType::SmallFire,
                start_delay: 0.0,
                main_progress: 0.0,
            },
        ));
    }
    
    // Stage 2: Massive main explosion that consumes the ship - appears quickly after small explosions start
    let main_delay = 0.3; // Short delay - appears almost immediately with small explosions
    let main_size = 3.0; // Much larger to consume the ship
    
    // Main explosion - starts pure white, transitions to orange, then fades
    // White phase has maximum glow/bloom effect
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(main_size)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 1.0), // Pure white at start
                emissive: LinearRgba::rgb(200.0, 200.0, 200.0), // Extremely bright for intense glow/bloom
                alpha_mode: AlphaMode::Opaque, // Ensure fully opaque for bloom
                ..default()
            }),
            transform: Transform::from_translation(position),
            ..default()
        },
        Explosion {
            lifetime: 0.0,
            max_lifetime: 2.0, // Lasts 2 seconds
            explosion_type: ExplosionType::MainFire,
            start_delay: main_delay,
            main_progress: 0.0, // Starts at 0, increments only when visible
        },
    ));
    
    // Stage 3: Smoke clouds - appear after main explosion starts
    let smoke_delay = 0.5; // Smoke appears shortly after main explosion starts
    let smoke_count = 8;
    
    for i in 0..smoke_count {
        let angle = (i as f32 / smoke_count as f32) * std::f32::consts::TAU;
        let radius = 2.0 + rand::random::<f32>() * 3.0;
        let height = rand::random::<f32>() * 2.0;
        let offset = Vec3::new(
            angle.cos() * radius,
            height,
            angle.sin() * radius,
        );
        
        let smoke_pos = position + offset;
        let smoke_size = 1.5 + rand::random::<f32>() * 1.5;
        
        // Smoke gradient: dark gray/black, less emissive
        let smoke_darkness = 0.15 + rand::random::<f32>() * 0.2; // 0.15-0.35
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(smoke_size)),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(smoke_darkness, smoke_darkness, smoke_darkness),
                    emissive: LinearRgba::rgb(0.5, 0.5, 0.5), // Slight glow
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                }),
                transform: Transform::from_translation(smoke_pos),
                ..default()
            },
            crate::components::ship::Velocity(Vec3::new(
                (rand::random::<f32>() - 0.5) * 5.0,
                rand::random::<f32>() * 3.0, // Smoke rises
                (rand::random::<f32>() - 0.5) * 5.0,
            )),
            Explosion {
                lifetime: 0.0,
                max_lifetime: 1.5,
                explosion_type: ExplosionType::Smoke,
                start_delay: smoke_delay,
                main_progress: 0.0,
            },
        ));
    }
    
    // Spawn debris particles
    for _ in 0..15 {
        let offset = Vec3::new(
            (rand::random::<f32>() - 0.5) * 3.0,
            (rand::random::<f32>() - 0.5) * 3.0,
            (rand::random::<f32>() - 0.5) * 3.0,
        );
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.2, 0.2, 0.2)),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.5, 0.3, 0.2), // Dark brown/metal
                    metallic: 0.8,
                    ..default()
                }),
                transform: Transform::from_translation(position + offset),
                ..default()
            },
            crate::components::ship::Velocity(offset.normalize() * (15.0 + rand::random::<f32>() * 15.0)),
            Explosion {
                lifetime: 0.0,
                max_lifetime: 2.0,
                explosion_type: ExplosionType::Smoke, // Reuse smoke type for debris
                start_delay: 0.0,
                main_progress: 0.0,
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

/// Update explosion effects with color gradients and timing
pub fn update_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Explosion, &mut Transform, Option<&Handle<StandardMaterial>>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let dt = time.delta_seconds();
    
    for (entity, mut explosion, mut transform, material_handle) in query.iter_mut() {
        // Don't start until delay has passed - hide explosion during delay
        if explosion.lifetime < explosion.start_delay {
            explosion.lifetime += dt;
            // Make explosion invisible during delay
            transform.scale = Vec3::ZERO;
            if let Some(material_handle) = material_handle {
                if let Some(material) = materials.get_mut(material_handle) {
                    material.base_color.set_alpha(0.0);
                }
            }
            continue;
        }
        
        // For main explosion, use its own independent progress counter that only increments when visible
        // For other explosions, use standard progress calculation
        let progress = if matches!(explosion.explosion_type, ExplosionType::MainFire) {
            // Main explosion: increment its own progress counter (starts at 0 when visible)
            explosion.main_progress += dt;
            if explosion.main_progress >= explosion.max_lifetime {
                commands.entity(entity).despawn();
                continue;
            }
            explosion.main_progress / explosion.max_lifetime
        } else {
            // Other explosions: use standard progress calculation
            let active_lifetime = explosion.lifetime - explosion.start_delay;
            explosion.lifetime += dt;
            
            if active_lifetime >= explosion.max_lifetime {
                commands.entity(entity).despawn();
                continue;
            }
            active_lifetime / explosion.max_lifetime
        };
        
        // Update material color based on explosion type and progress
        if let Some(material_handle) = material_handle {
            if let Some(material) = materials.get_mut(material_handle) {
                // Make sure it's visible now that delay has passed
                material.base_color.set_alpha(1.0);
                match explosion.explosion_type {
                ExplosionType::SmallFire => {
                    // Small fire: bright yellow -> orange -> red -> fade (lasts 1-3 seconds)
                    let fade = 1.0 - progress;
                    let color_progress = progress.min(0.6); // Slower color transition for longer duration
                    
                    let r = 1.0;
                    let g = if color_progress < 0.4 {
                        0.9 - color_progress * 0.5 // 0.9 -> 0.7
                    } else {
                        0.7 - (color_progress - 0.4) * 1.75 // 0.7 -> 0.0
                    };
                    let b = if color_progress < 0.25 {
                        0.3 - color_progress * 1.2 // 0.3 -> 0.0
                    } else {
                        0.0
                    };
                    
                    material.base_color = Color::srgb(r * fade, g * fade, b * fade);
                    // Maintain brightness longer for 1-3 second duration
                    let emissive_strength = if progress < 0.5 {
                        15.0 // Bright for first half
                    } else {
                        15.0 - (progress - 0.5) * 20.0 // Gradual fade
                    };
                    material.emissive = LinearRgba::rgb(
                        r * fade * emissive_strength,
                        g * fade * emissive_strength,
                        b * fade * emissive_strength * 0.3,
                    );
                    
                    // Expand more gradually over longer duration
                    let scale = 1.0 + progress * 3.0;
                    transform.scale = Vec3::splat(scale);
                }
                ExplosionType::MainFire => {
                    // Main fire: white -> orange -> fade
                    // White phase has maximum glow/bloom effect
                    // Note: progress is from main_progress, so it starts at 0 when explosion becomes visible
                    let fade = 1.0 - progress;
                    
                    // Color gradient: pure white -> orange -> fade
                    let r = 1.0;
                    let g = if progress < 0.4 {
                        // White phase: maintain high green/blue for white
                        1.0 - progress * 0.625 // 1.0 -> 0.75 (white transitioning to yellow-white)
                    } else {
                        // Orange phase: transition to orange
                        0.75 - (progress - 0.4) * 1.25 // 0.75 -> 0.25 (yellow-white to orange)
                    };
                    let b = if progress < 0.3 {
                        // White phase: maintain blue component for pure white
                        1.0 - progress * 2.0 // 1.0 -> 0.4 (pure white to yellow-white)
                    } else if progress < 0.5 {
                        // Transitioning to orange: remove blue
                        0.4 - (progress - 0.3) * 2.0 // 0.4 -> 0.0 (yellow-white to orange)
                    } else {
                        0.0 // Pure orange
                    };
                    
                    // Base color transitions from white to orange with fade
                    material.base_color = Color::srgb(r * fade, g * fade, b * fade);
                    material.base_color.set_alpha(1.0); // Ensure fully visible
                    
                    // Emissive values: maximum during white phase, then decrease
                    // Calculate emissive from the non-faded color for maximum brightness
                    // Then apply fade separately to emissive for better control
                    let emissive_base_strength = if progress < 0.3 {
                        // White phase: maximum glow/bloom
                        200.0 * fade
                    } else if progress < 0.5 {
                        // Transition from white to orange: still very bright
                        (200.0 - (progress - 0.3) * 50.0) * fade
                    } else if progress < 0.7 {
                        // Orange phase: bright but decreasing
                        (190.0 - (progress - 0.5) * 100.0) * fade
                    } else {
                        // Fade phase: rapid decrease
                        (170.0 - (progress - 0.7) * 566.0) * fade.max(0.0)
                    };
                    
                    // Emissive uses the unfaded RGB values for color, then applies strength
                    // This keeps the glow color consistent with maximum bloom trigger
                    material.emissive = LinearRgba::rgb(
                        r * emissive_base_strength,
                        g * emissive_base_strength,
                        b * emissive_base_strength
                    );
                    
                    // Debug: Print emissive values for first few frames
                    if progress < 0.1 {
                        println!("[Effects] MainFire explosion - progress: {:.3}, base_color: {:?}, emissive: {:?}, strength: {:.1}", 
                            progress, material.base_color, material.emissive, emissive_base_strength);
                    }
                    
                    // Expand dramatically - massive explosion that consumes everything
                    let scale = 1.0 + progress * 3.0; // Grows to 8x size
                    transform.scale = Vec3::splat(scale);
                }
                ExplosionType::Smoke => {
                    // Smoke: dark gray -> lighter gray -> fade out
                    let fade = 1.0 - progress;
                    let smoke_color = 0.2 + progress * 0.3; // Gets lighter as it expands
                    
                    material.base_color = Color::srgba(
                        smoke_color * fade,
                        smoke_color * fade,
                        smoke_color * fade,
                        fade * 0.8, // Alpha fades
                    );
                    material.emissive = LinearRgba::rgb(0.5 * fade, 0.5 * fade, 0.5 * fade);
                    
                    // Smoke expands and rises
                    let scale = 1.0 + progress * 2.0;
                    transform.scale = Vec3::splat(scale);
                }
                }
            }
        } else {
            // For explosions without materials (shouldn't happen, but handle gracefully)
            // Just expand based on type
            let scale = match explosion.explosion_type {
                ExplosionType::SmallFire => 1.0 + progress * 2.5,
                ExplosionType::MainFire => 1.0 + progress * 4.0,
                ExplosionType::Smoke => 1.0 + progress * 2.0,
            };
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

