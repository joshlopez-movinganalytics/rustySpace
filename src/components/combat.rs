use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Health component
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

/// Shield component with recharge mechanics
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Shield {
    pub current: f32,
    pub max: f32,
    pub recharge_rate: f32,
    pub recharge_delay: f32,
    pub time_since_last_hit: f32,
}

/// Energy for weapons and abilities
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Energy {
    pub current: f32,
    pub max: f32,
    pub recharge_rate: f32,
}

/// Weapon types
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum WeaponType {
    Laser,
    Plasma,
    Missile,
    Railgun,
    Autocannon,
    IonCannon,
    FlakCannon,
    BeamLaser,
}

/// Individual weapon
#[derive(Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage: f32,
    pub fire_rate: f32,
    pub projectile_speed: f32,
    pub energy_cost: f32,
    pub cooldown_timer: f32,
    pub spread: f32,
    pub alt_fire_charge: f32, // For charged weapons
    pub shield_damage_multiplier: f32, // For ion weapons
}

/// Weapon mount component
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct WeaponMount {
    pub weapons: Vec<Weapon>,
    pub current_weapon: usize,
}

/// Projectile component
#[derive(Component)]
pub struct Projectile {
    pub damage: f32,
    pub lifetime: f32,
    pub owner: Entity,
    pub weapon_type: WeaponType,
    pub shield_damage_multiplier: f32,
    pub piercing: bool,
}

/// Damage type for resistances
#[derive(Clone, Copy, Debug)]
pub enum DamageType {
    Kinetic,
    Energy,
    Explosive,
}

/// Faction for friend/foe identification
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Faction {
    Player,
    Enemy,
    Neutral,
}

impl Weapon {
    pub fn laser() -> Self {
        Self {
            weapon_type: WeaponType::Laser,
            damage: 10.0,
            fire_rate: 5.0,
            projectile_speed: 130.0, // Increased from 100
            energy_cost: 5.0,
            cooldown_timer: 0.0,
            spread: 0.01,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 1.0,
        }
    }

    pub fn plasma() -> Self {
        Self {
            weapon_type: WeaponType::Plasma,
            damage: 25.0,
            fire_rate: 2.0,
            projectile_speed: 78.0, // Increased from 60
            energy_cost: 15.0,
            cooldown_timer: 0.0,
            spread: 0.03,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 1.0,
        }
    }

    pub fn missile() -> Self {
        Self {
            weapon_type: WeaponType::Missile,
            damage: 50.0,
            fire_rate: 1.0,
            projectile_speed: 52.0, // Increased from 40
            energy_cost: 25.0,
            cooldown_timer: 0.0,
            spread: 0.0,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 1.0,
        }
    }

    pub fn railgun() -> Self {
        Self {
            weapon_type: WeaponType::Railgun,
            damage: 75.0,
            fire_rate: 0.5,
            projectile_speed: 260.0, // Increased from 200
            energy_cost: 40.0,
            cooldown_timer: 0.0,
            spread: 0.0,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 1.0,
        }
    }

    pub fn autocannon() -> Self {
        Self {
            weapon_type: WeaponType::Autocannon,
            damage: 15.0,
            fire_rate: 10.0,
            projectile_speed: 150.0,
            energy_cost: 3.0,
            cooldown_timer: 0.0,
            spread: 0.02,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 0.8,
        }
    }

    pub fn ion_cannon() -> Self {
        Self {
            weapon_type: WeaponType::IonCannon,
            damage: 5.0,
            fire_rate: 2.0,
            projectile_speed: 100.0,
            energy_cost: 20.0,
            cooldown_timer: 0.0,
            spread: 0.0,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 6.0, // Does 30 damage to shields
        }
    }

    pub fn flak_cannon() -> Self {
        Self {
            weapon_type: WeaponType::FlakCannon,
            damage: 20.0,
            fire_rate: 1.5,
            projectile_speed: 90.0,
            energy_cost: 18.0,
            cooldown_timer: 0.0,
            spread: 0.05,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 1.2,
        }
    }

    pub fn beam_laser() -> Self {
        Self {
            weapon_type: WeaponType::BeamLaser,
            damage: 8.0, // DPS-based, continuous
            fire_rate: 20.0, // Very high fire rate for beam effect
            projectile_speed: 200.0,
            energy_cost: 2.0,
            cooldown_timer: 0.0,
            spread: 0.0,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 0.9,
        }
    }
}

