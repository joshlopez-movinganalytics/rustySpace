use bevy::prelude::*;

/// Camera controller component
#[derive(Component)]
pub struct CameraController {
    pub follow_distance: f32,
    pub follow_height: f32,
    pub smoothness: f32,
}

