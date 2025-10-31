use bevy::prelude::*;
use crate::components::{
    ship::{Player, ShipPiece, ShipPieceType},
    ship_classes::{
        ClassProgression, ShipClass, ShipVisualConfig, ShipMeshVariant, 
        ShipAttachment, AttachmentType
    },
    upgrades::PlayerUpgrades,
};

/// System to apply visual changes based on class progression
pub fn apply_class_visuals_system(
    mut commands: Commands,
    class_progression: Res<ClassProgression>,
    upgrades: Res<PlayerUpgrades>,
    mut query: Query<(Entity, &mut ShipVisualConfig), With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !class_progression.is_changed() && !upgrades.is_changed() {
        return;
    }
    
    if let Ok((player_entity, mut visual_config)) = query.get_single_mut() {
        // Determine dominant class
        let mut max_points = 0;
        let mut dominant_class = ShipClass::Fighter;
        
        for class in [
            ShipClass::Fighter,
            ShipClass::Tank,
            ShipClass::Gunner,
            ShipClass::Stealth,
            ShipClass::Sniper,
            ShipClass::MissileTanker,
        ] {
            let points = class_progression.get_points(class);
            if points > max_points {
                max_points = points;
                dominant_class = class;
            }
        }
        
        // Apply mesh variant based on dominant class and points invested
        if max_points >= 5 {
            visual_config.mesh_variant = match dominant_class {
                ShipClass::Fighter => ShipMeshVariant::Fighter,
                ShipClass::Tank => ShipMeshVariant::Tank,
                ShipClass::Gunner => ShipMeshVariant::Gunner,
                ShipClass::Stealth => ShipMeshVariant::Stealth,
                ShipClass::Sniper => ShipMeshVariant::Sniper,
                ShipClass::MissileTanker => ShipMeshVariant::MissileTanker,
            };
            
            println!("[Ship Visuals] Switched to {} mesh variant", dominant_class.name());
        }
        
        // Apply color scheme based on dominant class
        if max_points >= 3 {
            visual_config.primary_color = dominant_class.primary_color();
            visual_config.secondary_color = dominant_class.secondary_color();
            visual_config.accent_color = dominant_class.accent_color();
        }
        
        // Apply scale modifier
        visual_config.scale_modifier = match dominant_class {
            ShipClass::Fighter => 0.9,  // Smaller, sleeker
            ShipClass::Tank => 1.3,     // Larger, bulkier
            ShipClass::Gunner => 1.1,   // Slightly larger
            ShipClass::Stealth => 0.85, // Compact
            ShipClass::Sniper => 1.0,   // Normal size, elongated
            ShipClass::MissileTanker => 1.2, // Bulky with racks
        };
        
        // Update attachments based on class progression
        update_attachments(&mut commands, player_entity, &class_progression, &mut visual_config, &mut meshes, &mut materials);
    }
}

/// Update ship attachments based on class points
fn update_attachments(
    commands: &mut Commands,
    player_entity: Entity,
    class_progression: &ClassProgression,
    visual_config: &mut ShipVisualConfig,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mut desired_attachments = Vec::new();
    
    // Fighter attachments
    let fighter_points = class_progression.get_points(ShipClass::Fighter);
    if fighter_points >= 5 {
        desired_attachments.push(AttachmentType::ThrusterWings);
    }
    if fighter_points >= 15 {
        desired_attachments.push(AttachmentType::SpeedBoosters);
    }
    
    // Tank attachments
    let tank_points = class_progression.get_points(ShipClass::Tank);
    if tank_points >= 5 {
        desired_attachments.push(AttachmentType::ArmorPlating);
    }
    if tank_points >= 15 {
        desired_attachments.push(AttachmentType::ShieldGenerators);
    }
    
    // Gunner attachments
    let gunner_points = class_progression.get_points(ShipClass::Gunner);
    if gunner_points >= 5 {
        desired_attachments.push(AttachmentType::WeaponPods);
    }
    if gunner_points >= 15 {
        desired_attachments.push(AttachmentType::AmmoFeeders);
    }
    
    // Stealth attachments
    let stealth_points = class_progression.get_points(ShipClass::Stealth);
    if stealth_points >= 5 {
        desired_attachments.push(AttachmentType::StealthPanels);
    }
    if stealth_points >= 15 {
        desired_attachments.push(AttachmentType::CloakEmitters);
    }
    
    // Sniper attachments
    let sniper_points = class_progression.get_points(ShipClass::Sniper);
    if sniper_points >= 5 {
        desired_attachments.push(AttachmentType::AntennaArrays);
    }
    if sniper_points >= 15 {
        desired_attachments.push(AttachmentType::TargetingScopes);
    }
    
    // Missile attachments
    let missile_points = class_progression.get_points(ShipClass::MissileTanker);
    if missile_points >= 5 {
        desired_attachments.push(AttachmentType::MissileRacks);
    }
    if missile_points >= 15 {
        desired_attachments.push(AttachmentType::LauncherPods);
    }
    
    // Update visual config
    visual_config.attachments = desired_attachments;
}

/// Spawn or update physical attachment entities
pub fn spawn_ship_attachments_system(
    mut commands: Commands,
    visual_config_query: Query<(Entity, &ShipVisualConfig, &Transform), (With<Player>, Changed<ShipVisualConfig>)>,
    attachment_query: Query<(Entity, &ShipAttachment)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok((player_entity, visual_config, player_transform)) = visual_config_query.get_single() {
        // Remove old attachments
        for (entity, attachment) in attachment_query.iter() {
            if attachment.parent_ship == player_entity {
                commands.entity(entity).despawn();
            }
        }
        
        // Spawn new attachments
        for attachment_type in &visual_config.attachments {
            spawn_attachment(&mut commands, player_entity, player_transform, *attachment_type, &mut meshes, &mut materials);
        }
        
        println!("[Ship Visuals] Updated {} attachments", visual_config.attachments.len());
    }
}

/// Spawn a single attachment
fn spawn_attachment(
    commands: &mut Commands,
    parent_entity: Entity,
    parent_transform: &Transform,
    attachment_type: AttachmentType,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let (mesh, color, offset) = match attachment_type {
        AttachmentType::ThrusterWings => {
            (
                meshes.add(Cuboid::new(0.5, 0.3, 2.0)),
                Color::srgb(0.2, 0.6, 1.0),
                Vec3::new(2.0, 0.0, -1.0),
            )
        }
        AttachmentType::SpeedBoosters => {
            (
                meshes.add(Sphere::new(0.4)),
                Color::srgb(0.4, 1.0, 1.0),
                Vec3::new(0.0, 0.0, -2.0),
            )
        }
        AttachmentType::ArmorPlating => {
            (
                meshes.add(Cuboid::new(3.0, 2.5, 0.3)),
                Color::srgb(0.5, 0.5, 0.5),
                Vec3::new(0.0, 0.0, 1.5),
            )
        }
        AttachmentType::ShieldGenerators => {
            (
                meshes.add(Sphere::new(0.6)),
                Color::srgb(0.2, 0.5, 1.0),
                Vec3::new(1.5, 1.0, 0.0),
            )
        }
        AttachmentType::WeaponPods => {
            (
                meshes.add(Cuboid::new(0.4, 0.4, 1.5)),
                Color::srgb(0.8, 0.2, 0.2),
                Vec3::new(1.5, 0.0, 1.0),
            )
        }
        AttachmentType::AmmoFeeders => {
            (
                meshes.add(Cuboid::new(0.6, 0.4, 0.8)),
                Color::srgb(0.6, 0.3, 0.1),
                Vec3::new(-1.5, 0.0, 0.0),
            )
        }
        AttachmentType::StealthPanels => {
            (
                meshes.add(Cuboid::new(2.5, 0.1, 2.5)),
                Color::srgb(0.1, 0.1, 0.2),
                Vec3::new(0.0, 1.2, 0.0),
            )
        }
        AttachmentType::CloakEmitters => {
            (
                meshes.add(Sphere::new(0.3)),
                Color::srgb(0.3, 0.3, 0.5),
                Vec3::new(0.0, 1.5, -1.0),
            )
        }
        AttachmentType::AntennaArrays => {
            (
                meshes.add(Cuboid::new(0.2, 2.0, 0.2)),
                Color::srgb(0.7, 0.7, 0.2),
                Vec3::new(0.0, 2.0, 0.0),
            )
        }
        AttachmentType::TargetingScopes => {
            (
                meshes.add(Cuboid::new(0.3, 0.3, 3.0)),
                Color::srgb(1.0, 1.0, 0.4),
                Vec3::new(0.0, 0.5, 3.0),
            )
        }
        AttachmentType::MissileRacks => {
            (
                meshes.add(Cuboid::new(0.8, 0.6, 1.2)),
                Color::srgb(0.6, 0.3, 0.0),
                Vec3::new(1.5, -0.5, 0.0),
            )
        }
        AttachmentType::LauncherPods => {
            (
                meshes.add(Cuboid::new(0.6, 0.8, 1.5)),
                Color::srgb(0.8, 0.4, 0.1),
                Vec3::new(-1.5, -0.5, -0.5),
            )
        }
    };
    
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color: color,
                metallic: 0.7,
                perceptual_roughness: 0.3,
                ..default()
            }),
            transform: Transform::from_translation(parent_transform.translation + offset),
            ..default()
        },
        ShipAttachment {
            attachment_type,
            parent_ship: parent_entity,
        },
    ));
}

/// Update attachment positions to follow parent ship
pub fn update_attachment_positions_system(
    player_query: Query<&Transform, With<Player>>,
    mut attachment_query: Query<(&mut Transform, &ShipAttachment), Without<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut attachment_transform, ship_attachment) in attachment_query.iter_mut() {
            // Keep attachments relative to parent ship
            // Simplified: just keep them at same rotation
            attachment_transform.rotation = player_transform.rotation;
        }
    }
}

/// Apply material color changes based on visual config
pub fn apply_ship_colors_system(
    visual_config_query: Query<&ShipVisualConfig, (With<Player>, Changed<ShipVisualConfig>)>,
    player_query: Query<&Handle<StandardMaterial>, With<Player>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(visual_config) = visual_config_query.get_single() {
        if let Ok(material_handle) = player_query.get_single() {
            if let Some(material) = materials.get_mut(material_handle) {
                material.base_color = visual_config.primary_color;
                println!("[Ship Visuals] Applied color scheme");
            }
        }
    }
}

/// Apply scale changes based on class
pub fn apply_ship_scale_system(
    mut query: Query<(&ShipVisualConfig, &mut Transform), (With<Player>, Changed<ShipVisualConfig>)>,
) {
    if let Ok((visual_config, mut transform)) = query.get_single_mut() {
        transform.scale = Vec3::splat(visual_config.scale_modifier);
        println!("[Ship Visuals] Applied scale: {}", visual_config.scale_modifier);
    }
}

