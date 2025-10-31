use bevy::prelude::*;
use crate::components::ship::{ShipType, ShipPiece, ShipPieceType, ShipVisuals, UpgradeVisuals};

/// Definition of a ship piece for procedural generation
pub struct ShipPieceDefinition {
    pub piece_type: ShipPieceType,
    pub shape: PieceShape,
    pub transform: Transform,
    pub color: Color,
    pub metallic: f32,
    pub emissive: Color,
}

/// Shape types for ship pieces
pub enum PieceShape {
    Cuboid { x: f32, y: f32, z: f32 },
    Cylinder { radius: f32, height: f32 },
    Capsule { radius: f32, depth: f32 },
}

/// Main function to build a ship with all its modular pieces
pub fn build_ship(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    ship_type: ShipType,
    parent_entity: Entity,
    base_color: Color,
) {
    let pieces = match ship_type {
        ShipType::Fighter => generate_fighter_layout(base_color),
        ShipType::Corvette => generate_corvette_layout(base_color),
        ShipType::Frigate => generate_frigate_layout(base_color),
        ShipType::CapitalShip => generate_capital_ship_layout(base_color),
    };

    // Add ShipVisuals and UpgradeVisuals components to parent
    commands.entity(parent_entity).insert((
        ShipVisuals { ship_type },
        UpgradeVisuals::default(),
    ));

    // Spawn all pieces as children of the parent ship entity
    for piece_def in pieces {
        spawn_ship_piece(
            commands,
            meshes,
            materials,
            parent_entity,
            piece_def,
        );
    }
}

/// Spawn a single ship piece as a child entity
fn spawn_ship_piece(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parent_entity: Entity,
    piece_def: ShipPieceDefinition,
) {
    let mesh = match piece_def.shape {
        PieceShape::Cuboid { x, y, z } => meshes.add(Cuboid::new(x, y, z)),
        PieceShape::Cylinder { radius, height } => {
            meshes.add(Cylinder::new(radius, height))
        }
        PieceShape::Capsule { radius, depth } => {
            meshes.add(Capsule3d::new(radius, depth))
        }
    };

    let material = materials.add(StandardMaterial {
        base_color: piece_def.color,
        metallic: piece_def.metallic,
        perceptual_roughness: 0.3,
        emissive: piece_def.emissive.into(),
        ..default()
    });

    let piece_entity = commands
        .spawn((
            PbrBundle {
                mesh,
                material,
                transform: piece_def.transform,
                ..default()
            },
            ShipPiece {
                piece_type: piece_def.piece_type,
                parent_ship: parent_entity,
            },
        ))
        .id();

    // Make it a child of the parent ship
    commands.entity(parent_entity).add_child(piece_entity);
}

/// Generate fighter layout - small, agile design
fn generate_fighter_layout(base_color: Color) -> Vec<ShipPieceDefinition> {
    vec![
        // Main hull
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 1.2, y: 0.4, z: 2.0 },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            color: base_color,
            metallic: 0.8,
            emissive: Color::NONE,
        },
        // Cockpit
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 0.6, y: 0.3, z: 0.8 },
            transform: Transform::from_xyz(0.0, 0.2, 1.0),
            color: Color::srgb(0.3, 0.6, 0.9),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Left wing
        ShipPieceDefinition {
            piece_type: ShipPieceType::Wing,
            shape: PieceShape::Cuboid { x: 1.5, y: 0.1, z: 1.0 },
            transform: Transform::from_xyz(-1.3, 0.0, -0.3),
            color: base_color,
            metallic: 0.7,
            emissive: Color::NONE,
        },
        // Right wing
        ShipPieceDefinition {
            piece_type: ShipPieceType::Wing,
            shape: PieceShape::Cuboid { x: 1.5, y: 0.1, z: 1.0 },
            transform: Transform::from_xyz(1.3, 0.0, -0.3),
            color: base_color,
            metallic: 0.7,
            emissive: Color::NONE,
        },
        // Engine
        ShipPieceDefinition {
            piece_type: ShipPieceType::Engine,
            shape: PieceShape::Cylinder { radius: 0.3, height: 0.8 },
            transform: Transform::from_xyz(0.0, 0.0, -1.2)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            color: Color::srgb(0.3, 0.3, 0.3),
            metallic: 0.9,
            emissive: Color::NONE,
        },
    ]
}

/// Generate corvette layout - medium size
fn generate_corvette_layout(base_color: Color) -> Vec<ShipPieceDefinition> {
    vec![
        // Main hull
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 1.8, y: 0.6, z: 2.8 },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            color: base_color,
            metallic: 0.8,
            emissive: Color::NONE,
        },
        // Forward section
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 1.2, y: 0.5, z: 1.2 },
            transform: Transform::from_xyz(0.0, 0.0, 1.8),
            color: base_color,
            metallic: 0.8,
            emissive: Color::NONE,
        },
        // Cockpit
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 0.8, y: 0.4, z: 0.6 },
            transform: Transform::from_xyz(0.0, 0.3, 2.2),
            color: Color::srgb(0.3, 0.6, 0.9),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Left wing
        ShipPieceDefinition {
            piece_type: ShipPieceType::Wing,
            shape: PieceShape::Cuboid { x: 1.0, y: 0.15, z: 1.5 },
            transform: Transform::from_xyz(-1.4, 0.0, 0.0),
            color: base_color,
            metallic: 0.7,
            emissive: Color::NONE,
        },
        // Right wing
        ShipPieceDefinition {
            piece_type: ShipPieceType::Wing,
            shape: PieceShape::Cuboid { x: 1.0, y: 0.15, z: 1.5 },
            transform: Transform::from_xyz(1.4, 0.0, 0.0),
            color: base_color,
            metallic: 0.7,
            emissive: Color::NONE,
        },
        // Left engine
        ShipPieceDefinition {
            piece_type: ShipPieceType::Engine,
            shape: PieceShape::Cylinder { radius: 0.35, height: 1.0 },
            transform: Transform::from_xyz(-0.6, 0.0, -1.6)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            color: Color::srgb(0.3, 0.3, 0.3),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Right engine
        ShipPieceDefinition {
            piece_type: ShipPieceType::Engine,
            shape: PieceShape::Cylinder { radius: 0.35, height: 1.0 },
            transform: Transform::from_xyz(0.6, 0.0, -1.6)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            color: Color::srgb(0.3, 0.3, 0.3),
            metallic: 0.9,
            emissive: Color::NONE,
        },
    ]
}

/// Generate frigate layout - larger ship
fn generate_frigate_layout(base_color: Color) -> Vec<ShipPieceDefinition> {
    vec![
        // Main hull - center
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 2.5, y: 0.8, z: 3.5 },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            color: base_color,
            metallic: 0.8,
            emissive: Color::NONE,
        },
        // Forward hull
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 2.0, y: 0.7, z: 1.8 },
            transform: Transform::from_xyz(0.0, 0.0, 2.5),
            color: base_color,
            metallic: 0.8,
            emissive: Color::NONE,
        },
        // Command section
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 1.5, y: 0.6, z: 1.2 },
            transform: Transform::from_xyz(0.0, 0.5, 1.5),
            color: base_color,
            metallic: 0.8,
            emissive: Color::NONE,
        },
        // Bridge
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 1.0, y: 0.4, z: 0.8 },
            transform: Transform::from_xyz(0.0, 0.9, 2.8),
            color: Color::srgb(0.3, 0.6, 0.9),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Left wing structure
        ShipPieceDefinition {
            piece_type: ShipPieceType::Wing,
            shape: PieceShape::Cuboid { x: 1.2, y: 0.3, z: 2.5 },
            transform: Transform::from_xyz(-1.8, 0.0, 0.0),
            color: base_color,
            metallic: 0.7,
            emissive: Color::NONE,
        },
        // Right wing structure
        ShipPieceDefinition {
            piece_type: ShipPieceType::Wing,
            shape: PieceShape::Cuboid { x: 1.2, y: 0.3, z: 2.5 },
            transform: Transform::from_xyz(1.8, 0.0, 0.0),
            color: base_color,
            metallic: 0.7,
            emissive: Color::NONE,
        },
        // Left weapon mount
        ShipPieceDefinition {
            piece_type: ShipPieceType::WeaponMount,
            shape: PieceShape::Cuboid { x: 0.3, y: 0.3, z: 0.8 },
            transform: Transform::from_xyz(-1.8, -0.3, 1.5),
            color: Color::srgb(0.4, 0.4, 0.4),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Right weapon mount
        ShipPieceDefinition {
            piece_type: ShipPieceType::WeaponMount,
            shape: PieceShape::Cuboid { x: 0.3, y: 0.3, z: 0.8 },
            transform: Transform::from_xyz(1.8, -0.3, 1.5),
            color: Color::srgb(0.4, 0.4, 0.4),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Left engine
        ShipPieceDefinition {
            piece_type: ShipPieceType::Engine,
            shape: PieceShape::Cylinder { radius: 0.45, height: 1.4 },
            transform: Transform::from_xyz(-1.0, 0.0, -2.2)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            color: Color::srgb(0.3, 0.3, 0.3),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Right engine
        ShipPieceDefinition {
            piece_type: ShipPieceType::Engine,
            shape: PieceShape::Cylinder { radius: 0.45, height: 1.4 },
            transform: Transform::from_xyz(1.0, 0.0, -2.2)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            color: Color::srgb(0.3, 0.3, 0.3),
            metallic: 0.9,
            emissive: Color::NONE,
        },
    ]
}

/// Generate capital ship layout - massive ship
fn generate_capital_ship_layout(base_color: Color) -> Vec<ShipPieceDefinition> {
    vec![
        // Main hull - massive center
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 4.0, y: 1.5, z: 5.0 },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            color: base_color,
            metallic: 0.8,
            emissive: Color::NONE,
        },
        // Forward section
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 3.0, y: 1.2, z: 2.5 },
            transform: Transform::from_xyz(0.0, 0.0, 3.5),
            color: base_color,
            metallic: 0.8,
            emissive: Color::NONE,
        },
        // Aft section
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 3.5, y: 1.3, z: 2.0 },
            transform: Transform::from_xyz(0.0, 0.0, -3.2),
            color: base_color,
            metallic: 0.8,
            emissive: Color::NONE,
        },
        // Command tower
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 2.0, y: 1.0, z: 2.0 },
            transform: Transform::from_xyz(0.0, 1.2, 0.5),
            color: base_color,
            metallic: 0.8,
            emissive: Color::NONE,
        },
        // Bridge
        ShipPieceDefinition {
            piece_type: ShipPieceType::Hull,
            shape: PieceShape::Cuboid { x: 1.5, y: 0.6, z: 1.0 },
            transform: Transform::from_xyz(0.0, 2.0, 1.0),
            color: Color::srgb(0.3, 0.6, 0.9),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Left wing assembly
        ShipPieceDefinition {
            piece_type: ShipPieceType::Wing,
            shape: PieceShape::Cuboid { x: 1.5, y: 0.4, z: 3.5 },
            transform: Transform::from_xyz(-2.7, 0.0, 0.0),
            color: base_color,
            metallic: 0.7,
            emissive: Color::NONE,
        },
        // Right wing assembly
        ShipPieceDefinition {
            piece_type: ShipPieceType::Wing,
            shape: PieceShape::Cuboid { x: 1.5, y: 0.4, z: 3.5 },
            transform: Transform::from_xyz(2.7, 0.0, 0.0),
            color: base_color,
            metallic: 0.7,
            emissive: Color::NONE,
        },
        // Left forward weapon array
        ShipPieceDefinition {
            piece_type: ShipPieceType::WeaponMount,
            shape: PieceShape::Cuboid { x: 0.4, y: 0.4, z: 1.2 },
            transform: Transform::from_xyz(-2.0, -0.5, 2.5),
            color: Color::srgb(0.4, 0.4, 0.4),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Right forward weapon array
        ShipPieceDefinition {
            piece_type: ShipPieceType::WeaponMount,
            shape: PieceShape::Cuboid { x: 0.4, y: 0.4, z: 1.2 },
            transform: Transform::from_xyz(2.0, -0.5, 2.5),
            color: Color::srgb(0.4, 0.4, 0.4),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Left side weapon array
        ShipPieceDefinition {
            piece_type: ShipPieceType::WeaponMount,
            shape: PieceShape::Cuboid { x: 0.4, y: 0.4, z: 1.0 },
            transform: Transform::from_xyz(-3.5, 0.0, 0.0),
            color: Color::srgb(0.4, 0.4, 0.4),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Right side weapon array
        ShipPieceDefinition {
            piece_type: ShipPieceType::WeaponMount,
            shape: PieceShape::Cuboid { x: 0.4, y: 0.4, z: 1.0 },
            transform: Transform::from_xyz(3.5, 0.0, 0.0),
            color: Color::srgb(0.4, 0.4, 0.4),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Left engine cluster - outer
        ShipPieceDefinition {
            piece_type: ShipPieceType::Engine,
            shape: PieceShape::Cylinder { radius: 0.5, height: 1.8 },
            transform: Transform::from_xyz(-1.5, 0.0, -4.2)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            color: Color::srgb(0.3, 0.3, 0.3),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Right engine cluster - outer
        ShipPieceDefinition {
            piece_type: ShipPieceType::Engine,
            shape: PieceShape::Cylinder { radius: 0.5, height: 1.8 },
            transform: Transform::from_xyz(1.5, 0.0, -4.2)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            color: Color::srgb(0.3, 0.3, 0.3),
            metallic: 0.9,
            emissive: Color::NONE,
        },
        // Center engine
        ShipPieceDefinition {
            piece_type: ShipPieceType::Engine,
            shape: PieceShape::Cylinder { radius: 0.6, height: 2.0 },
            transform: Transform::from_xyz(0.0, 0.0, -4.3)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            color: Color::srgb(0.3, 0.3, 0.3),
            metallic: 0.9,
            emissive: Color::NONE,
        },
    ]
}

