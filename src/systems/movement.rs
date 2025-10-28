use bevy::prelude::*;
use bevy::window::{PrimaryWindow, CursorGrabMode};
use bevy::input::mouse::MouseMotion;
use crate::components::ship::*;

/// Mouse sensitivity resource
#[derive(Resource)]
pub struct MouseFlightSettings {
    pub sensitivity: f32,
    pub smoothing: f32,
}

impl Default for MouseFlightSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.002,  // Radians per pixel
            smoothing: 0.15,     // How much to smooth rotation
        }
    }
}

/// Stores accumulated pitch and yaw for smooth mouse flight
#[derive(Resource, Default)]
pub struct MouseFlightState {
    pub pitch: f32,  // Rotation around X axis (up/down)
    pub yaw: f32,    // Rotation around Y axis (left/right)
}

/// Ship movement system - handles player input and applies forces
pub fn ship_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Ship, &mut Velocity, &mut AngularVelocity, &Transform), With<Player>>,
) {
    for (ship, mut velocity, mut angular_velocity, transform) in query.iter_mut() {
        let dt = time.delta_seconds();
        
        // Get ship's local axes
        let forward = transform.forward();
        let right = transform.right();
        let up = transform.up();
        
        // Linear thrust
        let mut thrust = Vec3::ZERO;
        
        // Forward/backward
        if keyboard.pressed(KeyCode::KeyW) {
            thrust += forward.as_vec3();
        }
        if keyboard.pressed(KeyCode::KeyS) {
            thrust -= forward.as_vec3();
        }
        
        // Strafe left/right
        if keyboard.pressed(KeyCode::KeyD) {
            thrust += right.as_vec3();
        }
        if keyboard.pressed(KeyCode::KeyA) {
            thrust -= right.as_vec3();
        }
        
        // Vertical thrust
        if keyboard.pressed(KeyCode::Space) {
            thrust += up.as_vec3();
        }
        if keyboard.pressed(KeyCode::ControlLeft) {
            thrust -= up.as_vec3();
        }
        
        // Apply boost
        let speed_multiplier = if keyboard.pressed(KeyCode::ShiftLeft) {
            ship.boost_multiplier
        } else {
            1.0
        };
        
        // Normalize thrust and apply acceleration
        if thrust.length() > 0.0 {
            thrust = thrust.normalize() * ship.acceleration * speed_multiplier * dt;
            velocity.0 += thrust;
        }
        
        // Apply drag
        velocity.0 *= 0.98;
        
        // Speed limit
        let max_speed = ship.max_speed * speed_multiplier;
        if velocity.0.length() > max_speed {
            velocity.0 = velocity.0.normalize() * max_speed;
        }
        
        // Roll (still keyboard-controlled)
        let mut roll = 0.0;
        if keyboard.pressed(KeyCode::KeyQ) {
            roll -= ship.turn_rate;
        }
        if keyboard.pressed(KeyCode::KeyE) {
            roll += ship.turn_rate;
        }
        
        angular_velocity.0.z = roll * dt;
        
        // Angular drag
        angular_velocity.0 *= 0.9;
    }
}

/// Mouse flight system - uses mouse delta movement for flight control
pub fn mouse_flight_system(
    mut mouse_motion: EventReader<MouseMotion>,
    settings: Res<MouseFlightSettings>,
    mut flight_state: ResMut<MouseFlightState>,
    time: Res<Time>,
    mut query: Query<(&Ship, &mut Transform), With<Player>>,
) {
    // Accumulate mouse delta
    let mut delta = Vec2::ZERO;
    for motion in mouse_motion.read() {
        delta += motion.delta;
    }
    
    if delta.length_squared() < 0.001 {
        return;
    }
    
    // Update pitch and yaw based on mouse movement
    // Negative delta.y because moving mouse up should pitch up (positive rotation)
    flight_state.pitch += -delta.y * settings.sensitivity;
    flight_state.yaw += delta.x * settings.sensitivity;
    
    // Clamp pitch to prevent flipping (about ±85 degrees)
    flight_state.pitch = flight_state.pitch.clamp(-1.48, 1.48);
    
    for (_ship, mut transform) in query.iter_mut() {
        let dt = time.delta_seconds();
        
        // Create target rotation from pitch and yaw
        // Apply yaw (Y-axis) first, then pitch (X-axis)
        let target_rotation = Quat::from_rotation_y(flight_state.yaw) 
            * Quat::from_rotation_x(flight_state.pitch);
        
        // Smoothly interpolate to target rotation
        let smoothing = (settings.smoothing * 60.0 * dt).min(1.0);
        transform.rotation = transform.rotation.slerp(target_rotation, smoothing);
    }
}

/// Apply velocity to transform
pub fn apply_velocity_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0 * dt;
    }
}

/// Apply angular velocity to transform
pub fn apply_angular_velocity_system(
    time: Res<Time>,
    mut query: Query<(&AngularVelocity, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    
    for (angular_velocity, mut transform) in query.iter_mut() {
        if angular_velocity.0.length() > 0.001 {
            let rotation = Quat::from_euler(
                EulerRot::XYZ,
                angular_velocity.0.x * dt,
                angular_velocity.0.y * dt,
                angular_velocity.0.z * dt,
            );
            transform.rotation = transform.rotation * rotation;
        }
    }
}

/// Manage cursor locking for flight controls
pub fn manage_cursor_lock(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        // Lock and hide cursor for flight controls
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}

/// Release cursor lock (for menus)
pub fn release_cursor_lock(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        // Release and show cursor
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

