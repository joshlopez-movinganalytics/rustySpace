use bevy::prelude::*;

/// Calculate lead for targeting moving objects
pub fn calculate_lead_point(
    shooter_pos: Vec3,
    target_pos: Vec3,
    target_velocity: Vec3,
    projectile_speed: f32,
) -> Vec3 {
    let to_target = target_pos - shooter_pos;
    let distance = to_target.length();
    let time_to_impact = distance / projectile_speed;
    
    target_pos + target_velocity * time_to_impact
}

/// Clamp a vector to a maximum length
pub fn clamp_length(v: Vec3, max_length: f32) -> Vec3 {
    if v.length() > max_length {
        v.normalize() * max_length
    } else {
        v
    }
}

/// Smooth damp for smooth interpolation
pub fn smooth_damp(current: f32, target: f32, velocity: &mut f32, smooth_time: f32, delta_time: f32) -> f32 {
    let omega = 2.0 / smooth_time;
    let x = omega * delta_time;
    let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
    let change = current - target;
    let temp = (*velocity + omega * change) * delta_time;
    *velocity = (*velocity - omega * temp) * exp;
    target + (change + temp) * exp
}

