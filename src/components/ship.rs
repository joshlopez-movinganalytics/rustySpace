use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Player marker component
#[derive(Component)]
pub struct Player;

/// Ship type for visual generation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipType {
    Fighter,
    Corvette,
    Frigate,
    CapitalShip,
}

/// Ship visuals component - stores ship type and visual configuration
#[derive(Component, Clone)]
pub struct ShipVisuals {
    pub ship_type: ShipType,
}

/// Ship piece type for modular construction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShipPieceType {
    Hull,
    Engine,
    Wing,
    WeaponMount,
    ArmorPlating,
    ShieldEmitter,
}

/// Marker component for individual ship pieces
#[derive(Component, Clone)]
pub struct ShipPiece {
    pub piece_type: ShipPieceType,
    pub parent_ship: Entity,
}

/// Tracks visual pieces that represent upgrades
#[derive(Component, Default, Clone)]
pub struct UpgradeVisuals {
    pub armor_pieces: Vec<Entity>,
    pub engine_pieces: Vec<Entity>,
    pub weapon_pieces: Vec<Entity>,
    pub shield_pieces: Vec<Entity>,
}

/// Ship component with flight characteristics
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub max_speed: f32,
    pub acceleration: f32,
    pub turn_rate: f32,
    pub mass: f32,
    pub boost_multiplier: f32,
}

/// Velocity component for physics-based movement
#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

/// Angular velocity for rotation
#[derive(Component, Default)]
pub struct AngularVelocity(pub Vec3);

impl Ship {
    pub fn fighter() -> Self {
        Self {
            max_speed: 60.0,
            acceleration: 25.0,
            turn_rate: 3.0,
            mass: 800.0,
            boost_multiplier: 2.5,
        }
    }

    pub fn corvette() -> Self {
        Self {
            max_speed: 45.0,
            acceleration: 18.0,
            turn_rate: 2.0,
            mass: 1500.0,
            boost_multiplier: 1.8,
        }
    }

    pub fn frigate() -> Self {
        Self {
            max_speed: 30.0,
            acceleration: 12.0,
            turn_rate: 1.2,
            mass: 3000.0,
            boost_multiplier: 1.5,
        }
    }

    pub fn capital_ship() -> Self {
        Self {
            max_speed: 20.0,
            acceleration: 8.0,
            turn_rate: 0.8,
            mass: 8000.0,
            boost_multiplier: 1.2,
        }
    }
}

