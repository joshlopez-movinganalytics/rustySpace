use bevy::prelude::*;
use crate::components::ship::Player;
use crate::components::travel::*;
use crate::components::galaxy::SystemId;
use crate::components::ai::Enemy;
use crate::components::combat::Projectile;
use crate::components::resources::Loot;
use crate::resources::Galaxy;

/// Check if player is near a jump gate and show prompt
pub fn check_jump_gate_proximity(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    gate_query: Query<(&Transform, &JumpGate), Without<Player>>,
    existing_prompts: Query<Entity, With<JumpPrompt>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    hyperspace_query: Query<&HyperspaceEffect>,
) {
    // Don't show prompts during hyperspace jump
    if hyperspace_query.iter().next().is_some() {
        return;
    }
    
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    
    let player_pos = player_transform.translation;
    
    // Check proximity to any gate
    let mut near_gate: Option<(SystemId, f32)> = None;
    
    for (gate_transform, gate) in gate_query.iter() {
        let distance = player_pos.distance(gate_transform.translation);
        if distance < gate.activation_range {
            near_gate = Some((gate.target_system_id, distance));
            break;
        }
    }
    
    if let Some((target_system, _distance)) = near_gate {
        // Show prompt if not already shown
        if existing_prompts.is_empty() {
            spawn_jump_prompt(&mut commands, target_system);
        }
        
        // Check for activation (J key)
        if keyboard.just_pressed(KeyCode::KeyJ) {
            initiate_hyperspace_jump(&mut commands, target_system, &existing_prompts);
        }
    } else {
        // Remove prompt if player moved away
        for entity in existing_prompts.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Spawn UI prompt for jump gate
fn spawn_jump_prompt(commands: &mut Commands, target_system: SystemId) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(150.0),
                left: Val::Percent(50.0),
                margin: UiRect::left(Val::Px(-150.0)),
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.2, 0.9).into(),
            ..default()
        },
        JumpPrompt,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            format!("Press J to jump to system {}", target_system),
            TextStyle {
                font_size: 24.0,
                color: Color::srgb(0.8, 0.9, 1.0),
                ..default()
            },
        ));
    });
}

/// Initiate hyperspace jump animation
fn initiate_hyperspace_jump(
    commands: &mut Commands,
    target_system: SystemId,
    existing_prompts: &Query<Entity, With<JumpPrompt>>,
) {
    println!("[Travel System] Initiating hyperspace jump to system {}", target_system);
    
    // Remove jump prompt
    for entity in existing_prompts.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Spawn hyperspace effect entity
    commands.spawn(HyperspaceEffect::new(target_system));
}

/// Update hyperspace animation
pub fn hyperspace_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut HyperspaceEffect)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    existing_visuals: Query<Entity, With<HyperspaceVisual>>,
    camera_query: Query<&Transform, With<Camera3d>>,
) {
    for (effect_entity, mut effect) in query.iter_mut() {
        effect.timer.tick(time.delta());
        
        let progress = effect.progress();
        
        // Spawn visual effects on first frame
        if progress < 0.05 && existing_visuals.is_empty() {
            spawn_hyperspace_visuals(&mut commands, &mut meshes, &mut materials, &camera_query);
        }
        
        // Update existing visuals
        update_hyperspace_visuals(&time, progress);
        
        // Complete jump when timer finishes
        if effect.timer.finished() {
            complete_hyperspace_jump(
                &mut commands,
                effect.target_system_id,
                effect_entity,
                &existing_visuals,
            );
        }
    }
}

/// Spawn hyperspace tunnel visual effects
fn spawn_hyperspace_visuals(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    camera_query: &Query<&Transform, With<Camera3d>>,
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };
    
    // Create tunnel effect with multiple rings
    for i in 0..20 {
        let distance = (i as f32) * 10.0;
        let position = camera_transform.translation + camera_transform.forward() * distance;
        let radius = 5.0 + (i as f32) * 0.5;
        
        let ring_mesh = meshes.add(Torus {
            minor_radius: 0.3,
            major_radius: radius,
        });
        
        let intensity = 1.0 - (i as f32 / 20.0);
        let ring_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.3 * intensity, 0.5 * intensity, 1.0 * intensity),
            emissive: Color::srgb(0.5, 0.7, 1.5).into(),
            alpha_mode: AlphaMode::Blend,
            ..default()
        });
        
        commands.spawn((
            PbrBundle {
                mesh: ring_mesh,
                material: ring_material,
                transform: Transform::from_translation(position)
                    .looking_at(camera_transform.translation, Vec3::Y),
                ..default()
            },
            HyperspaceVisual,
        ));
    }
    
    println!("[Travel System] Spawned hyperspace visuals");
}

/// Update hyperspace visual effects during animation
fn update_hyperspace_visuals(_time: &Res<Time>, _progress: f32) {
    // Visuals are currently static
    // Could add movement/pulsing effects here if desired
}

/// Complete the hyperspace jump and transition to new system
fn complete_hyperspace_jump(
    commands: &mut Commands,
    target_system: SystemId,
    effect_entity: Entity,
    existing_visuals: &Query<Entity, With<HyperspaceVisual>>,
) {
    println!("[Travel System] Completing hyperspace jump to system {}", target_system);
    
    // Despawn hyperspace effect
    commands.entity(effect_entity).despawn();
    
    // Despawn hyperspace visuals
    for entity in existing_visuals.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Insert flag to trigger system transition
    commands.insert_resource(SystemTransitionFlag {
        target_system_id: target_system,
    });
}

/// Resource flag for system transition
#[derive(Resource)]
pub struct SystemTransitionFlag {
    pub target_system_id: SystemId,
}

/// Handle system transition after hyperspace jump
pub fn handle_system_transition(
    mut commands: Commands,
    transition_flag: Option<Res<SystemTransitionFlag>>,
    mut galaxy: ResMut<Galaxy>,
    mut player_query: Query<&mut Transform, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    projectiles: Query<Entity, With<Projectile>>,
    loot_query: Query<Entity, With<Loot>>,
) {
    let Some(transition) = transition_flag else {
        return;
    };
    
    let target_system_id = transition.target_system_id;
    
    println!("[Travel System] Transitioning to system {}", target_system_id);
    
    // Update galaxy current system
    if !galaxy.jump_to_system(target_system_id) {
        println!("[Travel System] Failed to jump to system {}", target_system_id);
        commands.remove_resource::<SystemTransitionFlag>();
        return;
    }
    
    // Reset player position to center of new system
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        player_transform.translation = Vec3::new(0.0, 0.0, 0.0);
    }
    
    // Despawn all enemies from previous system
    for entity in enemy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Despawn all projectiles
    for entity in projectiles.iter() {
        commands.entity(entity).despawn();
    }
    
    // Despawn all loot
    for entity in loot_query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Insert flag to respawn system content
    commands.insert_resource(RespawnSystemContentFlag);
    
    // Remove transition flag
    commands.remove_resource::<SystemTransitionFlag>();
    
    println!("[Travel System] System transition complete");
}

/// Resource flag to trigger system content respawn
#[derive(Resource)]
pub struct RespawnSystemContentFlag;

/// Freeze player controls during hyperspace jump
pub fn freeze_controls_during_hyperspace(
    hyperspace_query: Query<&HyperspaceEffect>,
) -> bool {
    hyperspace_query.is_empty()
}

