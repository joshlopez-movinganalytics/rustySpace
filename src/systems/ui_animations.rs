use bevy::prelude::*;
use super::ui_theme::colors;
use std::f32::consts::TAU;

/// Component for pulsing glow animation
#[derive(Component)]
pub struct PulseAnimation {
    pub timer: f32,
    pub speed: f32,
    pub min_intensity: f32,
    pub max_intensity: f32,
    pub color: Color,
}

impl Default for PulseAnimation {
    fn default() -> Self {
        Self {
            timer: 0.0,
            speed: 2.0,
            min_intensity: 0.5,
            max_intensity: 1.0,
            color: colors::NEON_CYAN,
        }
    }
}

impl PulseAnimation {
    pub fn new(speed: f32, color: Color) -> Self {
        Self {
            speed,
            color,
            ..default()
        }
    }
    
    pub fn with_range(mut self, min: f32, max: f32) -> Self {
        self.min_intensity = min;
        self.max_intensity = max;
        self
    }
}

/// Component for scanline effect moving across UI
#[derive(Component)]
pub struct ScanlineEffect {
    pub timer: f32,
    pub speed: f32,
    pub height: f32,
}

impl Default for ScanlineEffect {
    fn default() -> Self {
        Self {
            timer: 0.0,
            speed: 100.0,
            height: 2.0,
        }
    }
}

/// Component for glitch distortion effect
#[derive(Component)]
pub struct GlitchEffect {
    pub timer: f32,
    pub interval: f32,
    pub duration: f32,
    pub intensity: f32,
    pub active: bool,
}

impl Default for GlitchEffect {
    fn default() -> Self {
        Self {
            timer: 0.0,
            interval: 3.0,
            duration: 0.1,
            intensity: 1.0,
            active: false,
        }
    }
}

/// Component for slide-in animation
#[derive(Component)]
pub struct SlideInAnimation {
    pub timer: f32,
    pub duration: f32,
    pub start_offset: Vec2,
    pub completed: bool,
}

impl Default for SlideInAnimation {
    fn default() -> Self {
        Self {
            timer: 0.0,
            duration: 0.4,
            start_offset: Vec2::new(-100.0, 0.0),
            completed: false,
        }
    }
}

impl SlideInAnimation {
    pub fn from_left(duration: f32) -> Self {
        Self {
            duration,
            start_offset: Vec2::new(-100.0, 0.0),
            ..default()
        }
    }
    
    pub fn from_right(duration: f32) -> Self {
        Self {
            duration,
            start_offset: Vec2::new(100.0, 0.0),
            ..default()
        }
    }
    
    pub fn from_top(duration: f32) -> Self {
        Self {
            duration,
            start_offset: Vec2::new(0.0, -100.0),
            ..default()
        }
    }
    
    pub fn from_bottom(duration: f32) -> Self {
        Self {
            duration,
            start_offset: Vec2::new(0.0, 100.0),
            ..default()
        }
    }
}

/// Component for fade animation
#[derive(Component)]
pub struct FadeAnimation {
    pub timer: f32,
    pub duration: f32,
    pub fade_in: bool,
    pub completed: bool,
}

impl Default for FadeAnimation {
    fn default() -> Self {
        Self {
            timer: 0.0,
            duration: 0.3,
            fade_in: true,
            completed: false,
        }
    }
}

impl FadeAnimation {
    pub fn fade_in(duration: f32) -> Self {
        Self {
            duration,
            fade_in: true,
            ..default()
        }
    }
    
    pub fn fade_out(duration: f32) -> Self {
        Self {
            duration,
            fade_in: false,
            ..default()
        }
    }
}

/// Component for floating/hovering effect
#[derive(Component)]
pub struct FloatAnimation {
    pub timer: f32,
    pub speed: f32,
    pub amplitude: f32,
}

impl Default for FloatAnimation {
    fn default() -> Self {
        Self {
            timer: 0.0,
            speed: 1.0,
            amplitude: 5.0,
        }
    }
}

impl FloatAnimation {
    pub fn new(speed: f32, amplitude: f32) -> Self {
        Self {
            speed,
            amplitude,
            ..default()
        }
    }
}

/// Component for rotating animation (for reticule elements)
#[derive(Component)]
pub struct RotateAnimation {
    pub speed: f32, // Degrees per second
}

impl Default for RotateAnimation {
    fn default() -> Self {
        Self { speed: 30.0 }
    }
}

/// Update pulse animations
pub fn update_pulse_animations(
    time: Res<Time>,
    mut query: Query<(&mut PulseAnimation, &mut BorderColor)>,
) {
    for (mut pulse, mut border_color) in query.iter_mut() {
        pulse.timer += time.delta_seconds() * pulse.speed;
        
        // Calculate pulse intensity using sine wave
        let sine_value = (pulse.timer * TAU).sin();
        let intensity = pulse.min_intensity + 
            (sine_value + 1.0) * 0.5 * (pulse.max_intensity - pulse.min_intensity);
        
        // Apply intensity to color
        let base_color = pulse.color;
        let pulsed_color = Color::srgb(
            base_color.to_srgba().red * intensity,
            base_color.to_srgba().green * intensity,
            base_color.to_srgba().blue * intensity,
        );
        
        *border_color = pulsed_color.into();
    }
}

/// Update pulse animations for background colors
pub fn update_pulse_backgrounds(
    time: Res<Time>,
    mut query: Query<(&mut PulseAnimation, &mut BackgroundColor), Without<BorderColor>>,
) {
    for (mut pulse, mut bg_color) in query.iter_mut() {
        pulse.timer += time.delta_seconds() * pulse.speed;
        
        let sine_value = (pulse.timer * TAU).sin();
        let intensity = pulse.min_intensity + 
            (sine_value + 1.0) * 0.5 * (pulse.max_intensity - pulse.min_intensity);
        
        let base_color = pulse.color;
        let pulsed_color = Color::srgba(
            base_color.to_srgba().red * intensity,
            base_color.to_srgba().green * intensity,
            base_color.to_srgba().blue * intensity,
            base_color.to_srgba().alpha,
        );
        
        *bg_color = pulsed_color.into();
    }
}

/// Update glitch effects
pub fn update_glitch_effects(
    time: Res<Time>,
    mut query: Query<(&mut GlitchEffect, &mut Style)>,
) {
    for (mut glitch, mut style) in query.iter_mut() {
        glitch.timer += time.delta_seconds();
        
        if !glitch.active && glitch.timer >= glitch.interval {
            // Start glitch
            glitch.active = true;
            glitch.timer = 0.0;
        } else if glitch.active && glitch.timer >= glitch.duration {
            // End glitch
            glitch.active = false;
            glitch.timer = 0.0;
            // Reset position
            style.left = Val::Auto;
        }
        
        if glitch.active {
            // Apply random offset
            let offset = (rand::random::<f32>() - 0.5) * 10.0 * glitch.intensity;
            style.left = Val::Px(offset);
        }
    }
}

/// Update glitch effects on text
pub fn update_glitch_text_effects(
    time: Res<Time>,
    mut query: Query<(&mut GlitchEffect, &mut Text)>,
) {
    for (mut glitch, mut text) in query.iter_mut() {
        glitch.timer += time.delta_seconds();
        
        if !glitch.active && glitch.timer >= glitch.interval {
            glitch.active = true;
            glitch.timer = 0.0;
        } else if glitch.active && glitch.timer >= glitch.duration {
            glitch.active = false;
            glitch.timer = 0.0;
        }
        
        if glitch.active && !text.sections.is_empty() {
            // Add color shift for glitch effect
            let r = rand::random::<f32>() * glitch.intensity;
            text.sections[0].style.color = Color::srgb(1.0, 1.0 - r, 1.0 - r);
        } else if !text.sections.is_empty() {
            // Reset to normal color
            text.sections[0].style.color = Color::WHITE;
        }
    }
}

/// Update slide-in animations
pub fn update_slide_animations(
    time: Res<Time>,
    mut query: Query<(&mut SlideInAnimation, &mut Style)>,
) {
    for (mut slide, mut style) in query.iter_mut() {
        if slide.completed {
            continue;
        }
        
        slide.timer += time.delta_seconds();
        let progress = (slide.timer / slide.duration).clamp(0.0, 1.0);
        
        // Ease-out cubic for smooth deceleration
        let eased_progress = 1.0 - (1.0 - progress).powi(3);
        
        // Interpolate from start_offset to 0
        let current_offset = slide.start_offset * (1.0 - eased_progress);
        
        // Apply offset based on direction
        if slide.start_offset.x != 0.0 {
            style.left = Val::Px(current_offset.x);
        }
        if slide.start_offset.y != 0.0 {
            style.top = Val::Px(current_offset.y);
        }
        
        if progress >= 1.0 {
            slide.completed = true;
            style.left = Val::Auto;
            style.top = Val::Auto;
        }
    }
}

/// Update fade animations
pub fn update_fade_animations(
    time: Res<Time>,
    mut query: Query<(&mut FadeAnimation, &mut BackgroundColor)>,
) {
    for (mut fade, mut bg_color) in query.iter_mut() {
        if fade.completed {
            continue;
        }
        
        fade.timer += time.delta_seconds();
        let progress = (fade.timer / fade.duration).clamp(0.0, 1.0);
        
        let alpha = if fade.fade_in {
            progress
        } else {
            1.0 - progress
        };
        
        let current_color = bg_color.0;
        *bg_color = Color::srgba(
            current_color.to_srgba().red,
            current_color.to_srgba().green,
            current_color.to_srgba().blue,
            alpha,
        ).into();
        
        if progress >= 1.0 {
            fade.completed = true;
        }
    }
}

/// Update float animations (for holographic hovering effect)
pub fn update_float_animations(
    time: Res<Time>,
    mut query: Query<(&mut FloatAnimation, &mut Style)>,
) {
    for (mut float, mut style) in query.iter_mut() {
        float.timer += time.delta_seconds() * float.speed;
        
        // Sine wave for smooth up/down motion
        let offset = (float.timer * TAU).sin() * float.amplitude;
        
        // Get current top value or default to 0
        let base_top = match style.top {
            Val::Px(px) => px,
            _ => 0.0,
        };
        
        style.top = Val::Px(base_top + offset);
    }
}

/// Update rotation animations (for reticule elements)
pub fn update_rotate_animations(
    time: Res<Time>,
    mut query: Query<(&RotateAnimation, &mut Transform)>,
) {
    for (rotate, mut transform) in query.iter_mut() {
        let rotation_delta = rotate.speed * time.delta_seconds();
        transform.rotate_z(rotation_delta.to_radians());
    }
}

/// Marker component for warning state (pulsing red when low health)
#[derive(Component)]
pub struct WarningPulse {
    pub active: bool,
}

/// Update warning pulse effects
pub fn update_warning_pulse(
    time: Res<Time>,
    mut query: Query<(&WarningPulse, &mut BackgroundColor)>,
) {
    let pulse_value = ((time.elapsed_seconds() * 3.0).sin() + 1.0) * 0.5;
    
    for (warning, mut bg_color) in query.iter_mut() {
        if warning.active {
            let intensity = 0.3 + pulse_value * 0.7;
            *bg_color = Color::srgba(1.0, 0.0, 0.0, intensity).into();
        }
    }
}

