use bevy::prelude::*;
use crate::components::ship::*;
use crate::components::combat::*;
use crate::components::ai::*;

/// AI controller system - manages behavior state machine
pub fn ai_controller_system(
    time: Res<Time>,
    mut query: Query<(&mut AIController, &Health, &Shield)>,
) {
    let dt = time.delta_seconds();
    
    for (mut ai, health, shield) in query.iter_mut() {
        ai.state_timer += dt;
        
        // Check health for retreat
        let health_percent = health.current / health.max;
        let shield_percent = shield.current / shield.max;
        
        if health_percent < 0.2 || (health_percent < 0.5 && shield_percent < 0.1) {
            ai.state = AIBehaviorState::Retreat;
        }
        
        // State transitions based on timer
        if ai.state_timer > 5.0 {
            ai.state_timer = 0.0;
            
            match ai.state {
                AIBehaviorState::Patrol => {
                    if ai.target.is_some() {
                        ai.state = AIBehaviorState::Pursue;
                    }
                }
                AIBehaviorState::Pursue => {
                    if ai.target.is_some() {
                        ai.state = AIBehaviorState::Attack;
                    }
                }
                AIBehaviorState::Attack => {
                    if rand::random::<f32>() < ai.evasion_threshold {
                        ai.state = AIBehaviorState::Evade;
                    }
                }
                AIBehaviorState::Evade => {
                    ai.state = AIBehaviorState::Attack;
                }
                AIBehaviorState::Retreat => {
                    if health_percent > 0.6 {
                        ai.state = AIBehaviorState::Patrol;
                    }
                }
            }
        }
    }
}

/// AI target acquisition system
pub fn ai_target_acquisition_system(
    mut ai_query: Query<(&mut AIController, &Transform, &Faction), Without<Player>>,
    target_query: Query<(Entity, &Transform, &Faction), With<Player>>,
) {
    for (mut ai, ai_transform, ai_faction) in ai_query.iter_mut() {
        // Find closest target of opposing faction
        let mut closest_distance = 200.0;
        let mut closest_target = None;
        
        for (target_entity, target_transform, target_faction) in target_query.iter() {
            if ai_faction != target_faction {
                let distance = ai_transform.translation.distance(target_transform.translation);
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_target = Some(target_entity);
                }
            }
        }
        
        ai.target = closest_target;
    }
}

/// AI weapon selection system - chooses optimal weapon based on target's shield/hull status
pub fn ai_weapon_selection_system(
    mut ai_query: Query<(&AIController, &mut WeaponMount), Without<Player>>,
    target_query: Query<(&Shield, &Health), With<Player>>,
) {
    for (ai, mut weapon_mount) in ai_query.iter_mut() {
        // Only switch weapons if AI has multiple weapons
        if weapon_mount.weapons.len() <= 1 {
            continue;
        }
        
        if let Some(target_entity) = ai.target {
            if let Ok((shield, health)) = target_query.get(target_entity) {
                let shield_percent = shield.current / shield.max;
                let health_percent = health.current / health.max;
                
                // Tactical weapon switching based on weapon type rules:
                // - Use shield-breaking weapons (Laser, IonCannon, BeamLaser) when shields > 25%
                // - Use hull-damaging weapons (Autocannon, Railgun) when shields low
                // - Use balanced/special weapons (Plasma, Missile) situationally
                
                let best_weapon_idx = if shield_percent > 0.25 {
                    // Target has shields - prioritize anti-shield weapons
                    weapon_mount.weapons.iter().enumerate()
                        .max_by(|(_, a), (_, b)| {
                            a.shield_damage_multiplier.partial_cmp(&b.shield_damage_multiplier).unwrap()
                        })
                        .map(|(idx, _)| idx)
                        .unwrap_or(0)
                } else if health_percent < 0.5 {
                    // Target low on health - use missiles for finishing blow if available
                    weapon_mount.weapons.iter().enumerate()
                        .find(|(_, w)| w.weapon_type == WeaponType::Missile)
                        .map(|(idx, _)| idx)
                        .unwrap_or_else(|| {
                            // Otherwise use best hull weapon
                            weapon_mount.weapons.iter().enumerate()
                                .max_by(|(_, a), (_, b)| {
                                    a.hull_damage_multiplier.partial_cmp(&b.hull_damage_multiplier).unwrap()
                                })
                                .map(|(idx, _)| idx)
                                .unwrap_or(0)
                        })
                } else {
                    // Shields down, target healthy - prioritize anti-hull weapons
                    weapon_mount.weapons.iter().enumerate()
                        .max_by(|(_, a), (_, b)| {
                            a.hull_damage_multiplier.partial_cmp(&b.hull_damage_multiplier).unwrap()
                        })
                        .map(|(idx, _)| idx)
                        .unwrap_or(0)
                };
                
                weapon_mount.current_weapon = best_weapon_idx;
            }
        }
    }
}

/// AI combat system - handles movement and firing
pub fn ai_combat_system(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ai_query: Query<(
        Entity,
        &AIController,
        &Ship,
        &mut Transform,
        &mut Velocity,
        &mut AngularVelocity,
        &mut WeaponMount,
        &mut Energy,
        &Faction,
        &Enemy,
    ), Without<Player>>,
    target_query: Query<(&Transform, &Velocity), With<Player>>,
) {
    let dt = time.delta_seconds();
    
    for (entity, ai, ship, mut transform, mut velocity, mut angular_velocity, mut weapon_mount, mut energy, faction, enemy) in ai_query.iter_mut() {
        // Update weapon cooldowns
        for weapon in weapon_mount.weapons.iter_mut() {
            weapon.cooldown_timer = (weapon.cooldown_timer - dt).max(0.0);
        }
        
        // Determine if this is an elite AI (Frigate or Capital Ship)
        let is_elite = matches!(enemy.enemy_type, EnemyType::Frigate | EnemyType::CapitalShip);
        
        if let Some(target_entity) = ai.target {
            if let Ok((target_transform, target_velocity)) = target_query.get(target_entity) {
                let to_target = target_transform.translation - transform.translation;
                let distance = to_target.length();
                
                // Lead targeting for elite AI
                let target_pos = if is_elite {
                    // Predict where target will be
                    let time_to_hit = distance / 100.0; // Assume projectile speed
                    target_transform.translation + target_velocity.0 * time_to_hit
                } else {
                    target_transform.translation
                };
                
                let to_target_predicted = target_pos - transform.translation;
                let desired_direction = to_target_predicted.normalize();
                
                // AI Movement and rotation
                match ai.state {
                    AIBehaviorState::Pursue | AIBehaviorState::Attack => {
                        // Rotate to face target
                        let current_forward = transform.forward().as_vec3();
                        
                        // Smooth rotation using cross product
                        let rotation_axis = current_forward.cross(desired_direction);
                        let rotation_angle = current_forward.dot(desired_direction).acos();
                        
                        if rotation_axis.length() > 0.001 && !rotation_angle.is_nan() {
                            let rotation_speed = ship.turn_rate * dt * 3.0; // Increased rotation speed
                            let actual_rotation_angle = rotation_angle.min(rotation_speed);
                            let rotation = Quat::from_axis_angle(
                                rotation_axis.normalize(),
                                actual_rotation_angle
                            );
                            transform.rotation = rotation * transform.rotation;
                        }
                        
                        // Elite AI: Tactical positioning (flanking)
                        if is_elite {
                            let right = transform.right();
                            let flank_offset = right.as_vec3() * 10.0 * (ai.state_timer * 0.5).sin();
                            
                            if distance > 40.0 {
                                let thrust = (current_forward + flank_offset.normalize() * 0.3) * ship.acceleration * dt;
                                velocity.0 += thrust;
                            } else if distance < 25.0 {
                                // Strafe while maintaining distance
                                velocity.0 += flank_offset.normalize() * ship.acceleration * dt;
                            }
                        } else {
                            // Basic AI: Simple aggressive pursuit
                            if distance > 35.0 {
                                let thrust = current_forward * ship.acceleration * 1.5 * dt;
                                velocity.0 += thrust;
                            } else if distance < 15.0 {
                                // Back away if too close
                                velocity.0 -= current_forward * ship.acceleration * 0.7 * dt;
                            }
                        }
                        
                        // Speed limit
                        if velocity.0.length() > ship.max_speed {
                            velocity.0 = velocity.0.normalize() * ship.max_speed;
                        }
                        
                        // Apply drag
                        velocity.0 *= 0.97;
                    }
                    AIBehaviorState::Evade => {
                        // Elite AI: Strategic evasion
                        if is_elite {
                            let perpendicular = Vec3::new(
                                -to_target.z,
                                to_target.y * 0.5,
                                to_target.x,
                            ).normalize();
                            
                            velocity.0 += perpendicular * ship.acceleration * dt * 1.5;
                            
                            // Barrel roll
                            angular_velocity.0.z = ship.turn_rate * 2.0 * dt;
                        } else {
                            // Basic AI: Random evasion
                            let evasion_direction = Vec3::new(
                                (rand::random::<f32>() - 0.5) * 2.0,
                                (rand::random::<f32>() - 0.5) * 2.0,
                                (rand::random::<f32>() - 0.5) * 2.0,
                            ).normalize();
                            
                            velocity.0 += evasion_direction * ship.acceleration * dt * 1.2;
                            
                            // Random roll
                            angular_velocity.0.z = ship.turn_rate * (rand::random::<f32>() - 0.5) * dt;
                        }
                        
                        if velocity.0.length() > ship.max_speed * ship.boost_multiplier {
                            velocity.0 = velocity.0.normalize() * ship.max_speed * ship.boost_multiplier;
                        }
                        
                        velocity.0 *= 0.95;
                    }
                    AIBehaviorState::Retreat => {
                        // Run away from target
                        let escape_direction = -to_target.normalize();
                        velocity.0 += escape_direction * ship.acceleration * ship.boost_multiplier * dt;
                        
                        if velocity.0.length() > ship.max_speed * ship.boost_multiplier {
                            velocity.0 = velocity.0.normalize() * ship.max_speed * ship.boost_multiplier;
                        }
                        
                        velocity.0 *= 0.99;
                    }
                    AIBehaviorState::Patrol => {
                        // Slow down and drift
                        velocity.0 *= 0.95;
                    }
                }
                
                match ai.state {
                    AIBehaviorState::Attack => {
                        // Fire at target if in range and facing them
                        // Elite AI: Longer range and better accuracy threshold
                        let max_range = if is_elite { 120.0 } else { 100.0 };
                        let min_range = if is_elite { 25.0 } else { 20.0 };
                        let accuracy_threshold = if is_elite { 0.85 } else { 0.9 };
                        
                        if distance < max_range && distance > min_range {
                            let forward = transform.forward();
                            let angle_to_target = forward.as_vec3().dot(to_target_predicted.normalize());
                            
                            if angle_to_target > accuracy_threshold {
                                // Fire weapon
                                let current_weapon_idx = weapon_mount.current_weapon;
                                if let Some(weapon) = weapon_mount.weapons.get_mut(current_weapon_idx) {
                                    if weapon.cooldown_timer <= 0.0 && energy.current >= weapon.energy_cost {
                                        weapon.cooldown_timer = 1.0 / weapon.fire_rate;
                                        energy.current -= weapon.energy_cost;
                                        
                                        let forward = transform.forward();
                                        let projectile_pos = transform.translation + forward.as_vec3() * 3.0;
                                        let projectile_direction = forward.as_vec3().normalize();
                                        let projectile_velocity = forward.as_vec3() * weapon.projectile_speed + velocity.0;
                                        
                                        // Calculate laser color based on damage (enemies use base damage)
                                        let base_damage = weapon.damage;
                                        let final_damage = weapon.damage; // Enemies don't have bonuses
                                        let laser_piercing = false; // Enemies don't have piercing upgrades
                                        
                                        let (mesh, base_color) = match weapon.weapon_type {
                                            WeaponType::Laser => (
                                                meshes.add(Capsule3d::new(0.05, 1.5)), // Smaller and sharper like player lasers
                                                Color::srgb(0.0, 1.0, 0.0), // Will be overridden
                                            ),
                                            WeaponType::Plasma => (
                                                meshes.add(Sphere::new(0.3)),
                                                Color::srgb(1.0, 0.2, 0.0),
                                            ),
                                            WeaponType::Missile => (
                                                meshes.add(Capsule3d::new(0.15, 0.8)),
                                                Color::srgb(0.8, 0.4, 0.0),
                                            ),
                                            WeaponType::Railgun => (
                                                meshes.add(Capsule3d::new(0.08, 2.0)),
                                                Color::srgb(1.0, 0.3, 0.0),
                                            ),
                                            WeaponType::Autocannon => (
                                                meshes.add(Capsule3d::new(0.1, 0.5)),
                                                Color::srgb(1.0, 0.8, 0.0),
                                            ),
                                            WeaponType::IonCannon => (
                                                meshes.add(Sphere::new(0.25)),
                                                Color::srgb(0.0, 0.5, 1.0),
                                            ),
                                            WeaponType::FlakCannon => (
                                                meshes.add(Sphere::new(0.4)),
                                                Color::srgb(0.7, 0.7, 0.7),
                                            ),
                                            WeaponType::BeamLaser => (
                                                meshes.add(Capsule3d::new(0.05, 1.5)),
                                                Color::srgb(1.0, 0.6, 0.2),
                                            ),
                                        };
                                        
                                        // Apply weapon type-specific properties
                                        let (homing_strength, area_damage, piercing) = match weapon.weapon_type {
                                            WeaponType::Missile => (15.0, 8.0, false),      // Homing missiles with area damage
                                            WeaponType::Railgun => (0.0, 0.0, true),        // Piercing rounds
                                            WeaponType::FlakCannon => (0.0, 5.0, false),    // Area damage
                                            _ => (0.0, 0.0, false),                         // Standard projectile
                                        };
                                        
                                        // Use the calculated color (for lasers) or base color (for other weapons)
                                        let final_color = if weapon.weapon_type == WeaponType::Laser {
                                            crate::systems::combat::calculate_laser_color(base_damage, final_damage, laser_piercing)
                                        } else {
                                            base_color
                                        };
                                        
                                        // Calculate rotation based on projectile direction instead of ship rotation
                                        let projectile_rotation = if projectile_direction.length() > 0.1 {
                                            Transform::from_translation(projectile_pos)
                                                .looking_to(projectile_direction, Vec3::Y)
                                        } else {
                                            Transform::from_translation(projectile_pos)
                                                .with_rotation(transform.rotation)
                                        };
                                        
                                        // Set emissive for bloom glow effect - keep color consistent, bloom will add the glow
                                        let emissive_intensity = if weapon.weapon_type == WeaponType::Laser || weapon.weapon_type == WeaponType::BeamLaser {
                                            LinearRgba::from(final_color) * 8.0 // Bright enough for bloom, but color stays consistent
                                        } else {
                                            LinearRgba::from(final_color) * 4.0
                                        };
                                        
                                        commands.spawn((
                                            PbrBundle {
                                                mesh,
                                                material: materials.add(StandardMaterial {
                                                    base_color: final_color,
                                                    emissive: emissive_intensity,
                                                    ..default()
                                                }),
                                                transform: projectile_rotation,
                                                ..default()
                                            },
                                            Projectile {
                                                damage: weapon.damage,
                                                lifetime: 5.0,
                                                owner: entity,
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
                                            *faction,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

