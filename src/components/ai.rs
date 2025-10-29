use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// AI behavior state
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AIBehaviorState {
    Patrol,
    Pursue,
    Attack,
    Evade,
    Retreat,
}

/// AI controller component
#[derive(Component)]
pub struct AIController {
    pub state: AIBehaviorState,
    pub target: Option<Entity>,
    pub patrol_point: Vec3,
    pub aggression: f32,
    pub evasion_threshold: f32,
    pub state_timer: f32,
}

/// Enemy ship type
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum EnemyType {
    Fighter,
    Corvette,
    Frigate,
    CapitalShip,
}

/// Enemy marker component
#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
}

impl AIController {
    pub fn new() -> Self {
        Self {
            state: AIBehaviorState::Patrol,
            target: None,
            patrol_point: Vec3::ZERO,
            aggression: 0.7,
            evasion_threshold: 0.3,
            state_timer: 0.0,
        }
    }

    pub fn fighter() -> Self {
        Self {
            state: AIBehaviorState::Patrol,
            target: None,
            patrol_point: Vec3::ZERO,
            aggression: 0.9,
            evasion_threshold: 0.4,
            state_timer: 0.0,
        }
    }

    pub fn corvette() -> Self {
        Self {
            state: AIBehaviorState::Patrol,
            target: None,
            patrol_point: Vec3::ZERO,
            aggression: 0.7,
            evasion_threshold: 0.35,
            state_timer: 0.0,
        }
    }

    pub fn frigate() -> Self {
        Self {
            state: AIBehaviorState::Patrol,
            target: None,
            patrol_point: Vec3::ZERO,
            aggression: 0.6,
            evasion_threshold: 0.25,
            state_timer: 0.0,
        }
    }

    pub fn capital_ship() -> Self {
        Self {
            state: AIBehaviorState::Attack,
            target: None,
            patrol_point: Vec3::ZERO,
            aggression: 0.5,
            evasion_threshold: 0.15,
            state_timer: 0.0,
        }
    }
}

