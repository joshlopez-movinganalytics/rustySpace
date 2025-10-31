use bevy::prelude::*;
use crate::systems::ui_theme::colors;

/// Hit marker component (displayed when hitting an enemy)
#[derive(Component)]
pub struct HitMarker {
    pub lifetime: f32,
}

/// Floating damage number component
#[derive(Component)]
pub struct DamageNumber {
    pub lifetime: f32,
    pub velocity: Vec3,
}

/// Kill confirmation notification
#[derive(Component)]
pub struct KillConfirmation {
    pub lifetime: f32,
}

/// Screen edge damage indicator (shows direction of damage)
#[derive(Component)]
pub struct DamageIndicator {
    pub lifetime: f32,
    pub direction: Vec2, // Direction damage came from
}

/// Event when player hits an enemy
#[derive(Event)]
pub struct HitEvent {
    pub position: Vec3,
    pub damage: f32,
    pub critical: bool,
}

/// Event when player kills an enemy
#[derive(Event)]
pub struct KillEvent;

/// Event when player takes damage
#[derive(Event)]
pub struct PlayerDamagedEvent {
    pub direction: Vec3, // Direction damage came from
}

/// Spawn hit marker at impact position
pub fn spawn_hit_marker_system(
    mut commands: Commands,
    mut hit_events: EventReader<HitEvent>,
    camera_query: Query<(&Camera, &GlobalTransform), With<crate::components::camera::CameraController>>,
) {
    let Ok((camera, camera_transform)) = camera_query.get_single() else {
        return;
    };
    
    for event in hit_events.read() {
        // Project 3D position to screen space
        if let Some(screen_pos) = camera.world_to_viewport(camera_transform, event.position) {
            // Spawn hit marker UI
            commands.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(screen_pos.x - 25.0),
                        top: Val::Px(screen_pos.y - 25.0),
                        width: Val::Px(50.0),
                        height: Val::Px(50.0),
                        ..default()
                    },
                    z_index: ZIndex::Global(50),
                    ..default()
                },
                HitMarker { lifetime: 0.2 },
            )).with_children(|parent| {
                // X-shaped hit marker
                let color = if event.critical {
                    colors::NEON_YELLOW // Critical hit in yellow
                } else {
                    Color::WHITE // Normal hit in white
                };
                
                // Horizontal line
                parent.spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Px(40.0),
                        height: Val::Px(3.0),
                        left: Val::Px(5.0),
                        top: Val::Px(23.5),
                        ..default()
                    },
                    background_color: color.into(),
                    ..default()
                });
                
                // Vertical line
                parent.spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Px(3.0),
                        height: Val::Px(40.0),
                        left: Val::Px(23.5),
                        top: Val::Px(5.0),
                        ..default()
                    },
                    background_color: color.into(),
                    ..default()
                });
            });
            
            // Spawn floating damage number
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        format!("{}", event.damage as i32),
                        TextStyle {
                            font_size: if event.critical { 28.0 } else { 20.0 },
                            color: if event.critical {
                                colors::NEON_YELLOW
                            } else {
                                Color::WHITE
                            },
                            ..default()
                        },
                    ),
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(screen_pos.x + 30.0),
                        top: Val::Px(screen_pos.y - 10.0),
                        ..default()
                    },
                    z_index: ZIndex::Global(51),
                    ..default()
                },
                DamageNumber {
                    lifetime: 1.0,
                    velocity: Vec3::new(0.0, 50.0, 0.0), // Float upward
                },
            ));
        }
    }
}

/// Update and remove hit markers
pub fn update_hit_markers_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut HitMarker, &mut BackgroundColor), With<HitMarker>>,
) {
    for (entity, mut marker, mut color) in query.iter_mut() {
        marker.lifetime -= time.delta_seconds();
        
        if marker.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        } else {
            // Fade out
            let alpha = marker.lifetime / 0.2;
            let current_color = color.0;
            *color = Color::srgba(
                current_color.to_srgba().red,
                current_color.to_srgba().green,
                current_color.to_srgba().blue,
                alpha,
            ).into();
        }
    }
}

/// Update and remove floating damage numbers
pub fn update_damage_numbers_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DamageNumber, &mut Style, &mut Text)>,
) {
    let dt = time.delta_seconds();
    
    for (entity, mut damage_num, mut style, mut text) in query.iter_mut() {
        damage_num.lifetime -= dt;
        
        if damage_num.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        } else {
            // Move upward
            if let Val::Px(ref mut top) = style.top {
                *top -= damage_num.velocity.y * dt;
            }
            
            // Fade out
            let alpha = damage_num.lifetime / 1.0;
            if !text.sections.is_empty() {
                let current_color = text.sections[0].style.color;
                text.sections[0].style.color = Color::srgba(
                    current_color.to_srgba().red,
                    current_color.to_srgba().green,
                    current_color.to_srgba().blue,
                    alpha,
                );
            }
        }
    }
}

/// Spawn kill confirmation notification
pub fn spawn_kill_confirmation_system(
    mut commands: Commands,
    mut kill_events: EventReader<KillEvent>,
) {
    for _ in kill_events.read() {
        commands.spawn((
            TextBundle {
                text: Text::from_section(
                    ">> ELIMINATED <<",
                    TextStyle {
                        font_size: 32.0,
                        color: colors::NEON_GREEN,
                        ..default()
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(65.0),
                    ..default()
                },
                z_index: ZIndex::Global(60),
                ..default()
            },
            KillConfirmation { lifetime: 1.5 },
        ));
    }
}

/// Update and remove kill confirmations
pub fn update_kill_confirmations_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KillConfirmation, &mut Text)>,
) {
    for (entity, mut confirmation, mut text) in query.iter_mut() {
        confirmation.lifetime -= time.delta_seconds();
        
        if confirmation.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        } else {
            // Pulse and fade
            let alpha = (confirmation.lifetime / 1.5).clamp(0.0, 1.0);
            let pulse = (confirmation.lifetime * 10.0).sin().abs();
            
            if !text.sections.is_empty() {
                text.sections[0].style.color = Color::srgba(
                    0.0,
                    1.0,
                    0.3,
                    alpha * (0.7 + pulse * 0.3),
                );
            }
        }
    }
}

/// Spawn damage indicators on screen edges
pub fn spawn_damage_indicator_system(
    mut commands: Commands,
    mut damage_events: EventReader<PlayerDamagedEvent>,
    player_query: Query<&Transform, With<crate::components::ship::Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    
    for event in damage_events.read() {
        // Calculate direction from player to damage source (in player's local space)
        let damage_dir = event.direction - player_transform.translation;
        let local_dir = player_transform.rotation.inverse() * damage_dir;
        
        // Project to 2D screen edge direction
        let screen_dir = Vec2::new(local_dir.x, local_dir.y).normalize_or_zero();
        
        // Calculate position on screen edge
        let _angle = screen_dir.y.atan2(screen_dir.x);
        let (left, top) = if screen_dir.x.abs() > screen_dir.y.abs() {
            // Left or right edge
            if screen_dir.x > 0.0 {
                (Val::Percent(95.0), Val::Percent(50.0 + screen_dir.y * 40.0))
            } else {
                (Val::Percent(5.0), Val::Percent(50.0 - screen_dir.y * 40.0))
            }
        } else {
            // Top or bottom edge
            if screen_dir.y > 0.0 {
                (Val::Percent(50.0 + screen_dir.x * 40.0), Val::Percent(95.0))
            } else {
                (Val::Percent(50.0 - screen_dir.x * 40.0), Val::Percent(5.0))
            }
        };
        
        // Spawn damage indicator
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left,
                    top,
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    ..default()
                },
                background_color: Color::srgba(1.0, 0.0, 0.0, 0.6).into(),
                border_radius: BorderRadius::all(Val::Px(15.0)),
                z_index: ZIndex::Global(45),
                ..default()
            },
            DamageIndicator {
                lifetime: 0.5,
                direction: screen_dir,
            },
        ));
    }
}

/// Update and remove damage indicators
pub fn update_damage_indicators_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DamageIndicator, &mut BackgroundColor)>,
) {
    for (entity, mut indicator, mut color) in query.iter_mut() {
        indicator.lifetime -= time.delta_seconds();
        
        if indicator.lifetime <= 0.0 {
            commands.entity(entity).despawn_recursive();
        } else {
            // Fade out
            let alpha = indicator.lifetime / 0.5;
            *color = Color::srgba(1.0, 0.0, 0.0, alpha * 0.6).into();
        }
    }
}

