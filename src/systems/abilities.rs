use bevy::prelude::*;
use crate::components::{
    ship::{Player, Velocity},
    abilities::{AbilityController, SpecialAbility, ActiveEffect, AbilityEffectType, AbilityVisualEffect},
    combat::{Health, Shield, WeaponMount},
};

/// System to handle ability key presses
pub fn ability_activation_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &Transform, &mut AbilityController), With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok((player_entity, transform, mut ability_controller)) = query.get_single_mut() {
        for ability in [
            SpecialAbility::QuantumDash,
            SpecialAbility::FortressMode,
            SpecialAbility::Devastation,
            SpecialAbility::PhaseShift,
            SpecialAbility::PerfectShot,
            SpecialAbility::MissileStorm,
        ] {
            if keyboard.just_pressed(ability.activation_key()) {
                if ability_controller.activate(ability) {
                    activate_ability(&mut commands, player_entity, transform, &mut ability_controller, ability, &mut meshes, &mut materials);
                } else if !ability_controller.is_unlocked(ability) {
                    println!("[Abilities] {} is not unlocked yet!", ability.name());
                } else {
                    let remaining = ability_controller.get_cooldown_remaining(ability);
                    println!("[Abilities] {} on cooldown ({:.1}s remaining)", ability.name(), remaining);
                }
            }
        }
    }
}

/// Activate a specific ability
fn activate_ability(
    commands: &mut Commands,
    player_entity: Entity,
    transform: &Transform,
    ability_controller: &mut AbilityController,
    ability: SpecialAbility,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    match ability {
        SpecialAbility::QuantumDash => {
            activate_quantum_dash(commands, player_entity, transform, meshes, materials);
        }
        SpecialAbility::FortressMode => {
            activate_fortress_mode(ability_controller);
        }
        SpecialAbility::Devastation => {
            activate_devastation(ability_controller);
        }
        SpecialAbility::PhaseShift => {
            activate_phase_shift(ability_controller);
        }
        SpecialAbility::PerfectShot => {
            activate_perfect_shot(ability_controller);
        }
        SpecialAbility::MissileStorm => {
            activate_missile_storm(commands, player_entity, transform, meshes, materials);
        }
    }
}

/// Quantum Dash - teleport forward
fn activate_quantum_dash(
    commands: &mut Commands,
    player_entity: Entity,
    transform: &Transform,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    println!("[Abilities] ⚡ QUANTUM DASH!");
    
    // Teleport forward 50 units
    let forward = transform.forward();
    let new_pos = transform.translation + forward.as_vec3() * 50.0;
    
    // Update position via component (would need to add teleport component)
    // For now, spawn visual effect at destination
    spawn_ability_visual(commands, new_pos, SpecialAbility::QuantumDash, meshes, materials);
}

/// Fortress Mode - invulnerable but immobile
fn activate_fortress_mode(ability_controller: &mut AbilityController) {
    println!("[Abilities] 🛡️ FORTRESS MODE!");
    ability_controller.add_active_effect(ActiveEffect {
        effect_type: AbilityEffectType::Invulnerable,
        remaining_duration: 3.0,
    });
    ability_controller.add_active_effect(ActiveEffect {
        effect_type: AbilityEffectType::Immobile,
        remaining_duration: 3.0,
    });
}

/// Devastation - all weapons fire
fn activate_devastation(ability_controller: &mut AbilityController) {
    println!("[Abilities] 💥 DEVASTATION!");
    ability_controller.add_active_effect(ActiveEffect {
        effect_type: AbilityEffectType::AllWeaponsFiring,
        remaining_duration: 2.0,
    });
}

/// Phase Shift - invisible and invulnerable
fn activate_phase_shift(ability_controller: &mut AbilityController) {
    println!("[Abilities] 👻 PHASE SHIFT!");
    ability_controller.add_active_effect(ActiveEffect {
        effect_type: AbilityEffectType::Invulnerable,
        remaining_duration: 2.0,
    });
    ability_controller.add_active_effect(ActiveEffect {
        effect_type: AbilityEffectType::Invisible,
        remaining_duration: 2.0,
    });
}

/// Perfect Shot - next shot 1000% damage
fn activate_perfect_shot(ability_controller: &mut AbilityController) {
    println!("[Abilities] 🎯 PERFECT SHOT!");
    ability_controller.add_active_effect(ActiveEffect {
        effect_type: AbilityEffectType::DamageBoost(1000),
        remaining_duration: 999.0, // Until next shot
    });
}

/// Missile Storm - launch 50 missiles
fn activate_missile_storm(
    commands: &mut Commands,
    player_entity: Entity,
    transform: &Transform,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    println!("[Abilities] 🚀 MISSILE STORM!");
    
    // Spawn visual effect
    spawn_ability_visual(commands, transform.translation, SpecialAbility::MissileStorm, meshes, materials);
    
    // Would spawn 50 missiles in all directions
    // Implementation would go in combat system
}

/// Spawn visual effect for ability activation
fn spawn_ability_visual(
    commands: &mut Commands,
    position: Vec3,
    ability: SpecialAbility,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let color = match ability {
        SpecialAbility::QuantumDash => Color::srgb(0.2, 0.8, 1.0),
        SpecialAbility::FortressMode => Color::srgb(0.8, 0.8, 0.0),
        SpecialAbility::Devastation => Color::srgb(1.0, 0.2, 0.2),
        SpecialAbility::PhaseShift => Color::srgb(0.6, 0.2, 0.8),
        SpecialAbility::PerfectShot => Color::srgb(1.0, 1.0, 0.2),
        SpecialAbility::MissileStorm => Color::srgb(1.0, 0.5, 0.0),
    };
    
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(3.0)),
            material: materials.add(StandardMaterial {
                base_color: color,
                emissive: color.into(),
                ..default()
            }),
            transform: Transform::from_translation(position),
            ..default()
        },
        AbilityVisualEffect {
            ability,
            lifetime: 1.0,
        },
    ));
}

/// Update ability cooldowns
pub fn update_ability_cooldowns_system(
    time: Res<Time>,
    mut query: Query<&mut AbilityController, With<Player>>,
) {
    if let Ok(mut ability_controller) = query.get_single_mut() {
        ability_controller.update_cooldowns(time.delta_seconds());
    }
}

/// Apply active ability effects
pub fn apply_ability_effects_system(
    mut query: Query<(&AbilityController, &mut Health, &mut Shield, &mut Velocity), With<Player>>,
) {
    if let Ok((ability_controller, mut health, mut shield, mut velocity)) = query.get_single_mut() {
        // Invulnerable - prevent damage (would need to modify damage system)
        if ability_controller.has_active_effect(AbilityEffectType::Invulnerable) {
            // Mark as invulnerable
            health.current = health.max;
            shield.current = shield.max;
        }
        
        // Immobile - stop movement
        if ability_controller.has_active_effect(AbilityEffectType::Immobile) {
            velocity.0 = Vec3::ZERO;
        }
        
        // Invisible - reduce visibility (would affect rendering)
    }
}

/// Cleanup expired visual effects
pub fn cleanup_ability_visuals_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AbilityVisualEffect)>,
) {
    for (entity, mut visual) in query.iter_mut() {
        visual.lifetime -= time.delta_seconds();
        if visual.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Apply devastation effect to weapons
pub fn devastation_effect_system(
    mut query: Query<(&AbilityController, &mut WeaponMount), With<Player>>,
) {
    if let Ok((ability_controller, mut weapon_mount)) = query.get_single_mut() {
        if ability_controller.has_active_effect(AbilityEffectType::AllWeaponsFiring) {
            // Force all weapons to be ready to fire
            for weapon in weapon_mount.weapons.iter_mut() {
                weapon.cooldown_timer = 0.0;
            }
        }
    }
}

