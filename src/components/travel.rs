use bevy::prelude::*;
use super::galaxy::SystemId;

/// Jump gate component - portal to another star system
#[derive(Component)]
pub struct JumpGate {
    pub target_system_id: SystemId,
    pub activation_range: f32,
}

/// Hyperspace effect during jump animation
#[derive(Component)]
pub struct HyperspaceEffect {
    pub timer: Timer,
    pub target_system_id: SystemId,
}

impl HyperspaceEffect {
    pub fn new(target_system_id: SystemId) -> Self {
        Self {
            timer: Timer::from_seconds(3.5, TimerMode::Once),
            target_system_id,
        }
    }
    
    pub fn progress(&self) -> f32 {
        self.timer.fraction()
    }
}

/// UI marker for jump gate interaction prompt
#[derive(Component)]
pub struct JumpPrompt;

/// Marker for hyperspace visual effect entities
#[derive(Component)]
pub struct HyperspaceVisual;

/// Component for the ring part of jump gate
#[derive(Component)]
pub struct JumpGateRing {
    pub rotation_speed: f32,
}

/// Component for the gate glow effect
#[derive(Component)]
pub struct JumpGateGlow {
    pub pulse_speed: f32,
    pub pulse_offset: f32,
}

