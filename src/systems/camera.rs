use bevy::prelude::*;
use crate::components::camera::CameraController;
use crate::components::ship::Player;

/// Camera follow system
pub fn camera_follow_system(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<CameraController>)>,
    mut camera_query: Query<(&mut Transform, &CameraController), With<Camera>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut camera_transform, controller) in camera_query.iter_mut() {
            let player_pos = player_transform.translation;
            let player_forward = player_transform.forward();
            let player_up = player_transform.up();
            
            // Calculate desired camera position
            let desired_pos = player_pos
                - player_forward.as_vec3() * controller.follow_distance
                + player_up.as_vec3() * controller.follow_height;
            
            // Smoothly interpolate to desired position
            let smoothness = controller.smoothness * time.delta_seconds();
            camera_transform.translation = camera_transform.translation.lerp(desired_pos, smoothness);
            
            // Look at a point ahead of and above the player to position ship lower on screen
            let look_offset = player_forward.as_vec3() * 10.0 + Vec3::Y * 5.0;
            camera_transform.look_at(player_pos + look_offset, Vec3::Y);
        }
    }
}

/// Camera free look system (for debugging or when player is destroyed)
pub fn camera_free_look_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if !keyboard.pressed(KeyCode::KeyC) {
        return;
    }
    
    let dt = time.delta_seconds();
    let move_speed = 20.0;
    
    for mut transform in camera_query.iter_mut() {
        let mut movement = Vec3::ZERO;
        
        if keyboard.pressed(KeyCode::KeyI) {
            movement += transform.forward().as_vec3();
        }
        if keyboard.pressed(KeyCode::KeyK) {
            movement -= transform.forward().as_vec3();
        }
        if keyboard.pressed(KeyCode::KeyJ) {
            movement -= transform.right().as_vec3();
        }
        if keyboard.pressed(KeyCode::KeyL) {
            movement += transform.right().as_vec3();
        }
        
        if movement.length() > 0.0 {
            transform.translation += movement.normalize() * move_speed * dt;
        }
    }
}

