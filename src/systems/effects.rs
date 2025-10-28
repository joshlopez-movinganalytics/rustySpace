use bevy::prelude::*;

/// Explosion effect marker
#[derive(Component)]
pub struct Explosion {
    pub lifetime: f32,
    pub max_lifetime: f32,
}

/// Spawn an explosion effect
pub fn spawn_explosion(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    // Central bright sphere
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(2.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.8, 0.2),
                emissive: Color::srgb(10.0, 5.0, 1.0).into(),
                ..default()
            }),
            transform: Transform::from_translation(position),
            ..default()
        },
        Explosion {
            lifetime: 0.0,
            max_lifetime: 0.5,
        },
    ));
    
    // Spawn debris particles
    for _ in 0..10 {
        let offset = Vec3::new(
            (rand::random::<f32>() - 0.5) * 2.0,
            (rand::random::<f32>() - 0.5) * 2.0,
            (rand::random::<f32>() - 0.5) * 2.0,
        );
        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(0.3, 0.3, 0.3)),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.7, 0.3, 0.1),
                    metallic: 0.8,
                    ..default()
                }),
                transform: Transform::from_translation(position + offset),
                ..default()
            },
            crate::components::ship::Velocity(offset.normalize() * 20.0),
            Explosion {
                lifetime: 0.0,
                max_lifetime: 1.0,
            },
        ));
    }
}

/// Update explosion effects
pub fn update_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Explosion, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    
    for (entity, mut explosion, mut transform) in query.iter_mut() {
        explosion.lifetime += dt;
        
        if explosion.lifetime >= explosion.max_lifetime {
            commands.entity(entity).despawn();
        } else {
            // Fade out and expand
            let progress = explosion.lifetime / explosion.max_lifetime;
            let scale = 1.0 + progress * 3.0;
            transform.scale = Vec3::splat(scale);
        }
    }
}

