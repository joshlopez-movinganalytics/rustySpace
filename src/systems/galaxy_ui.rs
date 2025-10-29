use bevy::prelude::*;
use crate::components::galaxy::*;
use crate::resources::{Galaxy, GameState};

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

/// Spawn a system node in the galaxy map
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
    
    // Color based on difficulty and current status
    let color = if is_current {
        Color::srgb(0.3, 0.6, 1.0) // Bright blue for current
    } else {
        // Gradient from green (easy) to red (hard)
        let difficulty_ratio = (system.difficulty as f32 / 10.0).clamp(0.0, 1.0);
        Color::srgb(
            0.2 + difficulty_ratio * 0.6,
            0.8 - difficulty_ratio * 0.6,
            0.2,
        )
    };
    
    let node_mesh = meshes.add(Sphere::new(size));
    let node_material = materials.add(StandardMaterial {
        base_color: color,
        emissive: Color::srgb(color.to_srgba().red * 0.5, color.to_srgba().green * 0.5, color.to_srgba().blue * 0.5).into(),
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
    
    // Mark current system
    if is_current {
        entity_commands.insert(CurrentSystemMarker);
    }
}

/// Spawn a connection line between two systems
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
    
    // Create a thin cylinder as the connection line
    let line_mesh = meshes.add(Cylinder {
        radius: 0.5,
        half_height: distance / 2.0,
    });
    
    let line_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.4, 0.4, 0.6, 0.6),
        emissive: Color::srgb(0.1, 0.1, 0.2).into(),
        alpha_mode: AlphaMode::Blend,
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

/// Spawn UI overlay for galaxy map
fn spawn_galaxy_map_overlay(commands: &mut Commands, galaxy: &Galaxy) {
    // Title and instructions
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        GalaxyMapOverlay,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "GALAXY MAP",
            TextStyle {
                font_size: 36.0,
                color: Color::srgb(0.8, 0.9, 1.0),
                ..default()
            },
        ));
        
        parent.spawn(TextBundle::from_section(
            format!("Current System: {}", 
                    galaxy.current_system()
                        .map(|s| s.name.as_str())
                        .unwrap_or("Unknown")),
            TextStyle {
                font_size: 20.0,
                color: Color::srgb(0.6, 0.7, 0.8),
                ..default()
            },
        ));
        
        parent.spawn(TextBundle::from_section(
            "\nControls:",
            TextStyle {
                font_size: 18.0,
                color: Color::srgb(0.7, 0.8, 0.9),
                ..default()
            },
        ));
        
        parent.spawn(TextBundle::from_section(
            "Mouse Drag - Rotate View\nScroll - Zoom\nWASD - Pan\nESC/M - Close Map",
            TextStyle {
                font_size: 16.0,
                color: Color::srgb(0.5, 0.6, 0.7),
                ..default()
            },
        ));
    });
    
    // Legend
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                left: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        GalaxyMapOverlay,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "Legend:",
            TextStyle {
                font_size: 18.0,
                color: Color::srgb(0.7, 0.8, 0.9),
                ..default()
            },
        ));
        
        parent.spawn(TextBundle::from_section(
            "Blue - Current System\nGreen - Low Difficulty\nYellow - Medium Difficulty\nRed - High Difficulty",
            TextStyle {
                font_size: 14.0,
                color: Color::srgb(0.5, 0.6, 0.7),
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
    
    // Despawn overlay UI
    for entity in overlay_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

