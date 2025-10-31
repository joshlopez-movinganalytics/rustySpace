use bevy::prelude::*;
use crate::components::galaxy::*;
use crate::components::travel::*;
use crate::resources::Galaxy;

/// Resource flag to trigger system content spawning
#[derive(Resource)]
pub struct SpawnSystemContentFlag;

/// Spawn planets and jump gates for the current star system
pub fn spawn_system_content(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    galaxy: Res<Galaxy>,
    existing_planets: Query<Entity, With<Planet>>,
    existing_gates: Query<Entity, With<JumpGate>>,
    spawn_flag: Option<Res<SpawnSystemContentFlag>>,
) {
    // Only spawn if flag is present
    if spawn_flag.is_none() {
        return;
    }
    
    // Remove the flag
    commands.remove_resource::<SpawnSystemContentFlag>();
    
    // Despawn existing planets and gates
    for entity in existing_planets.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in existing_gates.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    let Some(current_system) = galaxy.current_system() else {
        println!("[Galaxy System] No current system found!");
        return;
    };
    
    println!("[Galaxy System] Spawning content for system: {} (difficulty: {})", 
             current_system.name, current_system.difficulty);
    
    // Spawn planets
    for planet_data in &current_system.planets {
        spawn_planet(
            &mut commands,
            &mut meshes,
            &mut materials,
            planet_data,
            Vec3::ZERO,
        );
    }
    
    // Spawn jump gates for each connected system
    for (idx, &connected_id) in current_system.connected_systems.iter().enumerate() {
        spawn_jump_gate(
            &mut commands,
            &mut meshes,
            &mut materials,
            connected_id,
            idx,
            current_system.connected_systems.len(),
        );
    }
}

/// Spawn a single planet
fn spawn_planet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    planet_data: &PlanetData,
    center: Vec3,
) {
    let planet_mesh = meshes.add(Sphere::new(planet_data.size));
    let planet_material = materials.add(StandardMaterial {
        base_color: Color::srgb(
            planet_data.color[0],
            planet_data.color[1],
            planet_data.color[2],
        ),
        ..default()
    });
    
    // Calculate initial position
    let angle = planet_data.orbit_offset;
    let x = center.x + planet_data.orbit_radius * angle.cos();
    let z = center.z + planet_data.orbit_radius * angle.sin();
    
    commands.spawn((
        PbrBundle {
            mesh: planet_mesh,
            material: planet_material,
            transform: Transform::from_xyz(x, center.y, z),
            ..default()
        },
        Planet {
            orbit_radius: planet_data.orbit_radius,
            orbit_speed: planet_data.orbit_speed,
            orbit_offset: planet_data.orbit_offset,
            center,
        },
    ));
}

/// Spawn a jump gate
fn spawn_jump_gate(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    target_system_id: SystemId,
    index: usize,
    total_gates: usize,
) {
    // Position gates in a circle around the origin
    let angle = (index as f32 / total_gates as f32) * std::f32::consts::TAU;
    let distance = 200.0;
    let position = Vec3::new(
        angle.cos() * distance,
        0.0,
        angle.sin() * distance,
    );
    
    // Create gate parent entity
    let gate_entity = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(position)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        JumpGate {
            target_system_id,
            activation_range: 30.0,
        },
    )).id();
    
    // Create gate ring (torus)
    let ring_mesh = meshes.add(Torus {
        minor_radius: 1.5,
        major_radius: 15.0,
    });
    let ring_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.6, 0.9),
        metallic: 0.8,
        perceptual_roughness: 0.2,
        emissive: Color::srgb(0.2, 0.4, 0.6).into(),
        ..default()
    });
    
    commands.entity(gate_entity).with_children(|parent| {
        parent.spawn((
            PbrBundle {
                mesh: ring_mesh,
                material: ring_material,
                transform: Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                ..default()
            },
            JumpGateRing {
                rotation_speed: 0.5,
            },
        ));
    });
    
    // Create gate glow effect (inner sphere)
    let glow_mesh = meshes.add(Sphere::new(12.0));
    let glow_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.4, 0.7, 1.0, 0.3),
        alpha_mode: AlphaMode::Blend,
        emissive: Color::srgb(0.3, 0.5, 0.8).into(),
        ..default()
    });
    
    commands.entity(gate_entity).with_children(|parent| {
        parent.spawn((
            PbrBundle {
                mesh: glow_mesh,
                material: glow_material,
                ..default()
            },
            JumpGateGlow {
                pulse_speed: 2.0,
                pulse_offset: index as f32,
            },
        ));
    });
    
    println!("[Galaxy System] Spawned jump gate to system {} at {:?}", target_system_id, position);
}

/// Update planet orbits
pub fn update_planet_orbits(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Planet)>,
) {
    for (mut transform, planet) in query.iter_mut() {
        let elapsed = time.elapsed_seconds();
        let angle = planet.orbit_offset + (elapsed * planet.orbit_speed);
        
        let x = planet.center.x + planet.orbit_radius * angle.cos();
        let z = planet.center.z + planet.orbit_radius * angle.sin();
        
        transform.translation = Vec3::new(x, planet.center.y, z);
    }
}

/// Animate jump gate rings
pub fn animate_jump_gate_rings(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &JumpGateRing)>,
) {
    for (mut transform, ring) in query.iter_mut() {
        transform.rotate_local_z(ring.rotation_speed * time.delta_seconds());
    }
}

/// Animate jump gate glow effect
pub fn animate_jump_gate_glow(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &JumpGateGlow)>,
) {
    for (mut transform, glow) in query.iter_mut() {
        let pulse = ((time.elapsed_seconds() * glow.pulse_speed + glow.pulse_offset).sin() + 1.0) / 2.0;
        let scale = 1.0 + (pulse * 0.1);
        transform.scale = Vec3::splat(scale);
    }
}

