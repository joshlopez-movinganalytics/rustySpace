use bevy::prelude::*;

/// Scanline overlay component
#[derive(Component)]
pub struct ScanlineOverlay {
    pub timer: f32,
    pub speed: f32,
}

impl Default for ScanlineOverlay {
    fn default() -> Self {
        Self {
            timer: 0.0,
            speed: 200.0, // Pixels per second
        }
    }
}

/// Vignette overlay component
#[derive(Component)]
pub struct VignetteOverlay;

/// Background particle system
#[derive(Component)]
pub struct BackgroundParticle {
    pub lifetime: f32,
    pub velocity: Vec2,
}

/// Screen shake effect
#[derive(Resource)]
pub struct ScreenShake {
    pub intensity: f32,
    pub duration: f32,
    pub timer: f32,
}

impl Default for ScreenShake {
    fn default() -> Self {
        Self {
            intensity: 0.0,
            duration: 0.0,
            timer: 0.0,
        }
    }
}

impl ScreenShake {
    pub fn trigger(&mut self, intensity: f32, duration: f32) {
        self.intensity = intensity;
        self.duration = duration;
        self.timer = 0.0;
    }
    
    pub fn is_active(&self) -> bool {
        self.timer < self.duration
    }
}

/// Setup screen effects overlay
pub fn setup_screen_effects(mut commands: Commands) {
    println!("[UI Effects] Setting up screen effects overlay");
    
    // Vignette overlay (darkens edges)
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::NONE.into(),
            z_index: ZIndex::Global(5),
            ..default()
        },
        VignetteOverlay,
    ));
}

/// Spawn background particles for menus
pub fn spawn_background_particles(
    mut commands: Commands,
    _time: Res<Time>,
    particle_query: Query<Entity, With<BackgroundParticle>>,
) {
    let particle_count = particle_query.iter().count();
    
    // Maintain ~20 particles at a time
    if particle_count < 20 && rand::random::<f32>() < 0.3 {
        let x = rand::random::<f32>() * 100.0;
        let y = rand::random::<f32>() * 100.0;
        
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(x),
                    top: Val::Percent(y),
                    width: Val::Px(2.0),
                    height: Val::Px(2.0),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.9, 1.0, 0.4).into(),
                z_index: ZIndex::Global(-5),
                ..default()
            },
            BackgroundParticle {
                lifetime: 5.0,
                velocity: Vec2::new(
                    (rand::random::<f32>() - 0.5) * 2.0,
                    (rand::random::<f32>() - 0.5) * 2.0,
                ),
            },
        ));
    }
}

/// Update background particles
pub fn update_background_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut BackgroundParticle, &mut Style, &mut BackgroundColor)>,
) {
    let dt = time.delta_seconds();
    
    for (entity, mut particle, mut style, mut color) in query.iter_mut() {
        particle.lifetime -= dt;
        
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            // Move particle
            if let Val::Percent(ref mut left) = style.left {
                *left += particle.velocity.x * dt;
            }
            if let Val::Percent(ref mut top) = style.top {
                *top += particle.velocity.y * dt;
            }
            
            // Fade based on lifetime
            let alpha = (particle.lifetime / 5.0).clamp(0.0, 1.0) * 0.4;
            *color = Color::srgba(0.0, 0.9, 1.0, alpha).into();
        }
    }
}

/// Update screen shake
pub fn update_screen_shake(
    time: Res<Time>,
    mut shake: ResMut<ScreenShake>,
    mut camera_query: Query<&mut Transform, With<crate::components::camera::CameraController>>,
) {
    if !shake.is_active() {
        return;
    }
    
    shake.timer += time.delta_seconds();
    
    if shake.timer >= shake.duration {
        shake.intensity = 0.0;
        return;
    }
    
    // Apply shake offset to camera
    let progress = shake.timer / shake.duration;
    let intensity = shake.intensity * (1.0 - progress); // Fade out
    
    for mut transform in camera_query.iter_mut() {
        let offset_x = (rand::random::<f32>() - 0.5) * intensity;
        let offset_y = (rand::random::<f32>() - 0.5) * intensity;
        
        transform.translation.x += offset_x;
        transform.translation.y += offset_y;
    }
}

/// Create chromatic aberration effect on text (simulated with color shift)
#[derive(Component)]
pub struct ChromaticAberration {
    pub intensity: f32,
}

/// Update chromatic aberration effect
pub fn update_chromatic_aberration(
    time: Res<Time>,
    mut query: Query<(&ChromaticAberration, &mut Text)>,
) {
    for (aberration, mut text) in query.iter_mut() {
        if text.sections.is_empty() {
            continue;
        }
        
        // Subtle RGB split effect using time-based offset
        let offset = (time.elapsed_seconds() * 2.0).sin() * aberration.intensity;
        let color = text.sections[0].style.color;
        
        // Add slight color shift for RGB split illusion
        text.sections[0].style.color = Color::srgb(
            color.to_srgba().red + offset * 0.1,
            color.to_srgba().green,
            color.to_srgba().blue - offset * 0.1,
        );
    }
}

