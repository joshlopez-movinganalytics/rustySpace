use bevy::prelude::*;
use crate::components::galaxy::*;
use crate::resources::{Galaxy, GameState};
use crate::systems::ui_theme::colors;
use crate::systems::ui_animations::PulseAnimation;

/// Setup galaxy map UI with 3D visualization
pub fn setup_galaxy_map_ui(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    galaxy: Res<Galaxy>,
) {
    println!("[Galaxy UI] Setting up galaxy map");
    
    // Spawn galaxy map camera
    let camera_distance = 500.0;
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, camera_distance * 0.7, camera_distance)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        GalaxyMapCamera,
    ));
    
    // Spawn system nodes
    for system in galaxy.systems.values() {
        spawn_system_node(&mut commands, &mut meshes, &mut materials, system, &galaxy);
    }
    
    // Spawn connection lines
    for system in galaxy.systems.values() {
        for &connected_id in &system.connected_systems {
            // Only draw each connection once (from lower ID to higher ID)
            if system.id < connected_id {
                if let Some(connected_system) = galaxy.get_system(connected_id) {
                    spawn_connection_line(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        system,
                        connected_system,
                    );
                }
            }
        }
    }
    
    // Spawn UI overlay
    spawn_galaxy_map_overlay(&mut commands, &galaxy);
}

/// Spawn a system node in the galaxy map - CYBERPUNK WITH GLOW
fn spawn_system_node(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    system: &StarSystem,
    galaxy: &Galaxy,
) {
    let is_current = system.id == galaxy.current_system_id;
    
    // Node size based on difficulty
    let size = 5.0 + (system.difficulty as f32 * 0.5);
    
    // Cyberpunk color scheme based on difficulty and current status
    let color = if is_current {
        colors::NEON_CYAN // Bright cyan for current system
    } else {
        // Neon gradient from green (easy) to magenta (hard)
        let difficulty_ratio = (system.difficulty as f32 / 10.0).clamp(0.0, 1.0);
        Color::srgb(
            difficulty_ratio * 1.0,        // More red for harder
            1.0 - difficulty_ratio * 0.5,   // Less green for harder
            0.3 + difficulty_ratio * 0.5,   // Purple tint
        )
    };
    
    let node_mesh = meshes.add(Sphere::new(size));
    let node_material = materials.add(StandardMaterial {
        base_color: color,
        emissive: (color.to_linear() * 3.0).into(), // Increased emissive for neon glow
        unlit: true,
        ..default()
    });
    
    let mut entity_commands = commands.spawn((
        PbrBundle {
            mesh: node_mesh,
            material: node_material,
            transform: Transform::from_translation(system.position),
            ..default()
        },
        SystemNode {
            system_id: system.id,
        },
    ));
    
    // Mark current system and add pulsing effect
    if is_current {
        entity_commands.insert(CurrentSystemMarker);
        
        // Add outer glow ring for current system
        let ring_mesh = meshes.add(Torus::new(size * 1.5, size * 0.3));
        let ring_material = materials.add(StandardMaterial {
            base_color: colors::NEON_CYAN,
            emissive: (colors::NEON_CYAN.to_linear() * 4.0).into(),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        });
        
        commands.spawn((
            PbrBundle {
                mesh: ring_mesh,
                material: ring_material,
                transform: Transform::from_translation(system.position),
                ..default()
            },
            CurrentSystemRing,
        ));
    }
}

/// Marker for current system ring
#[derive(Component)]
pub struct CurrentSystemRing;

/// Spawn a connection line between two systems - NEON ENERGY LINES
fn spawn_connection_line(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    from_system: &StarSystem,
    to_system: &StarSystem,
) {
    let from = from_system.position;
    let to = to_system.position;
    let midpoint = (from + to) / 2.0;
    let direction = (to - from).normalize();
    let distance = from.distance(to);
    
    // Create a thin cylinder as the connection line with neon glow
    let line_mesh = meshes.add(Cylinder {
        radius: 0.6,
        half_height: distance / 2.0,
    });
    
    let line_material = materials.add(StandardMaterial {
        base_color: colors::ELECTRIC_PURPLE,
        emissive: (colors::ELECTRIC_PURPLE.to_linear() * 2.0).into(),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    
    // Calculate rotation to align with connection
    let up = Vec3::Y;
    let rotation = if direction.abs_diff_eq(up, 0.001) || direction.abs_diff_eq(-up, 0.001) {
        Quat::IDENTITY
    } else {
        Quat::from_rotation_arc(up, direction)
    };
    
    commands.spawn((
        PbrBundle {
            mesh: line_mesh,
            material: line_material,
            transform: Transform::from_translation(midpoint)
                .with_rotation(rotation),
            ..default()
        },
        SystemConnection {
            from_system: from_system.id,
            to_system: to_system.id,
        },
    ));
}

/// Spawn UI overlay for galaxy map - CYBERPUNK HOLOGRAPHIC
fn spawn_galaxy_map_overlay(commands: &mut Commands, galaxy: &Galaxy) {
    // Title and instructions panel
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::srgba(0.02, 0.0, 0.08, 0.85).into(),
            border_color: colors::NEON_CYAN.into(),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            ..default()
        },
        GalaxyMapOverlay,
        PulseAnimation::new(1.0, colors::NEON_CYAN).with_range(0.75, 1.0),
    )).with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(
                "// GALAXY NAV-SYSTEM",
                TextStyle {
                    font_size: 32.0,
                    color: colors::NEON_CYAN,
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            })
        );
        
        parent.spawn((
            TextBundle::from_section(
                format!(">> CURRENT: {}", 
                        galaxy.current_system()
                            .map(|s| s.name.as_str())
                            .unwrap_or("UNKNOWN")),
                TextStyle {
                    font_size: 18.0,
                    color: colors::NEON_GREEN,
                    ..default()
                },
            ),
            PulseAnimation::new(2.0, colors::NEON_GREEN).with_range(0.7, 1.0),
        ));
        
        parent.spawn(
            TextBundle::from_section(
                "\n// CONTROLS",
                TextStyle {
                    font_size: 16.0,
                    color: colors::ELECTRIC_PURPLE,
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::top(Val::Px(15.0)),
                ..default()
            })
        );
        
        parent.spawn(TextBundle::from_section(
            "ROTATE > Mouse Drag\nZOOM    > Scroll\nPAN     > WASD\nEXIT    > ESC / M",
            TextStyle {
                font_size: 14.0,
                color: Color::srgb(0.7, 0.8, 0.9),
                ..default()
            },
        ));
    });
    
    // Legend panel
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                left: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(15.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::srgba(0.02, 0.0, 0.08, 0.85).into(),
            border_color: colors::NEON_MAGENTA.into(),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            ..default()
        },
        GalaxyMapOverlay,
        PulseAnimation::new(1.2, colors::NEON_MAGENTA).with_range(0.75, 1.0),
    )).with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(
                "// THREAT ASSESSMENT",
                TextStyle {
                    font_size: 16.0,
                    color: colors::NEON_MAGENTA,
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::bottom(Val::Px(8.0)),
                ..default()
            })
        );
        
        parent.spawn(TextBundle::from_section(
            "[●] CYAN    > Current Location\n[●] GREEN   > Low Threat\n[●] YELLOW  > Medium Threat\n[●] MAGENTA > High Threat",
            TextStyle {
                font_size: 13.0,
                color: Color::srgb(0.7, 0.8, 0.9),
                ..default()
            },
        ));
    });
}

/// Component marker for galaxy map overlay UI
#[derive(Component)]
pub struct GalaxyMapOverlay;

/// Galaxy map camera controls
pub fn galaxy_map_camera_controls(
    mut camera_query: Query<&mut Transform, With<GalaxyMapCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut mouse_wheel: EventReader<bevy::input::mouse::MouseWheel>,
    time: Res<Time>,
) {
    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };
    
    let dt = time.delta_seconds();
    
    // Mouse drag to rotate
    if mouse_button.pressed(MouseButton::Left) {
        for event in mouse_motion.read() {
            let sensitivity = 0.003;
            let delta = event.delta * sensitivity;
            
            // Rotate around Y axis (horizontal movement)
            let rotation_y = Quat::from_rotation_y(-delta.x);
            
            // Rotate around local X axis (vertical movement)
            let right = camera_transform.right();
            let rotation_x = Quat::from_axis_angle(*right, -delta.y);
            
            // Apply rotations
            let current_pos = camera_transform.translation;
            let distance = current_pos.length();
            
            camera_transform.translation = rotation_y * rotation_x * current_pos;
            camera_transform.look_at(Vec3::ZERO, Vec3::Y);
            
            // Maintain distance
            camera_transform.translation = camera_transform.translation.normalize() * distance;
        }
    } else {
        // Clear mouse motion events if not dragging
        mouse_motion.clear();
    }
    
    // Mouse wheel to zoom
    for event in mouse_wheel.read() {
        let zoom_speed = 20.0;
        let zoom_delta = -event.y * zoom_speed;
        
        let direction = camera_transform.translation.normalize();
        camera_transform.translation += direction * zoom_delta;
        
        // Clamp distance
        let distance = camera_transform.translation.length();
        let min_distance = 200.0;
        let max_distance = 1000.0;
        if distance < min_distance || distance > max_distance {
            camera_transform.translation = direction * distance.clamp(min_distance, max_distance);
        }
    }
    
    // WASD to pan
    let pan_speed = 100.0;
    let mut pan = Vec3::ZERO;
    
    if keyboard.pressed(KeyCode::KeyW) {
        pan += *camera_transform.up();
    }
    if keyboard.pressed(KeyCode::KeyS) {
        pan -= *camera_transform.up();
    }
    if keyboard.pressed(KeyCode::KeyA) {
        pan -= *camera_transform.right();
    }
    if keyboard.pressed(KeyCode::KeyD) {
        pan += *camera_transform.right();
    }
    
    if pan.length() > 0.0 {
        camera_transform.translation += pan.normalize() * pan_speed * dt;
    }
}

/// Handle closing the galaxy map
pub fn galaxy_map_close_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::KeyM) {
        println!("[Galaxy UI] Closing galaxy map");
        next_state.set(GameState::InGame);
    }
}

/// Cleanup galaxy map UI
pub fn cleanup_galaxy_map_ui(
    mut commands: Commands,
    camera_query: Query<Entity, With<GalaxyMapCamera>>,
    node_query: Query<Entity, With<SystemNode>>,
    connection_query: Query<Entity, With<SystemConnection>>,
    overlay_query: Query<Entity, With<GalaxyMapOverlay>>,
    ring_query: Query<Entity, With<CurrentSystemRing>>,
) {
    println!("[Galaxy UI] Cleaning up galaxy map");
    
    // Despawn camera
    for entity in camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Despawn nodes
    for entity in node_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Despawn connections
    for entity in connection_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Despawn rings
    for entity in ring_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Despawn overlay UI
    for entity in overlay_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Animate current system ring (rotation)
pub fn animate_current_system_ring(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<CurrentSystemRing>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}

