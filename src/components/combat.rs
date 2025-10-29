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
    pub shield_damage_multiplier: f32, // Multiplier for shield damage
    pub hull_damage_multiplier: f32, // Multiplier for hull damage
    
    // Weapon state tracking
    pub heat: f32,              // Current heat level (0.0 - max_heat)
    pub max_heat: f32,          // Maximum heat before overheat
    pub heat_per_shot: f32,     // Heat generated per shot
    pub cooling_rate: f32,      // Heat dissipation per second
    
    pub current_ammo: u32,      // Current ammo in magazine
    pub max_ammo: u32,          // Magazine capacity (0 = infinite)
    pub reserve_ammo: u32,      // Reserve ammo pool
    pub reload_time: f32,       // Time to reload in seconds
    pub reload_timer: f32,      // Current reload progress
    pub is_reloading: bool,     // Currently reloading flag
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
    pub hull_damage_multiplier: f32,
    pub piercing: bool,
    pub area_damage: f32, // Radius for area of effect damage
    pub homing_strength: f32, // 0.0 = no homing, higher = stronger homing
    pub homing_target: Option<Entity>,
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
    /// Laser - Anti-Shield weapon (2.5x shield, 0.3x hull)
    /// Alt-fire: 3-shot burst
    /// Heat-based: Generates heat per shot, must cool down when overheated
    pub fn laser() -> Self {
        Self {
            weapon_type: WeaponType::Laser,
            damage: 12.0,
            fire_rate: 6.0,
            projectile_speed: 150.0,
            energy_cost: 4.0,
            cooldown_timer: 0.0,
            spread: 0.008,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 2.5,  // 30 damage to shields
            hull_damage_multiplier: 0.3,     // 3.6 damage to hull
            
            // Heat mechanics
            heat: 0.0,
            max_heat: 100.0,
            heat_per_shot: 8.0,              // ~12 shots before overheat
            cooling_rate: 25.0,              // Cools in ~4 seconds
            
            // No ammo limit
            current_ammo: 0,
            max_ammo: 0,                     // 0 = infinite
            reserve_ammo: 0,
            reload_time: 0.0,
            reload_timer: 0.0,
            is_reloading: false,
        }
    }

    /// Autocannon - Anti-Hull weapon (0.2x shield, 2.0x hull)
    /// Alt-fire: Shotgun spread
    /// Ammo-based: Must reload when magazine is empty
    pub fn autocannon() -> Self {
        Self {
            weapon_type: WeaponType::Autocannon,
            damage: 14.0,
            fire_rate: 8.0,
            projectile_speed: 140.0,
            energy_cost: 3.0,
            cooldown_timer: 0.0,
            spread: 0.015,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 0.2,  // 2.8 damage to shields (50% less than before)
            hull_damage_multiplier: 2.0,     // 28 damage to hull
            
            // No heat
            heat: 0.0,
            max_heat: 0.0,
            heat_per_shot: 0.0,
            cooling_rate: 0.0,
            
            // Ammo mechanics
            current_ammo: 60,                // Start with full mag
            max_ammo: 60,                    // 60 rounds per magazine
            reserve_ammo: 300,               // 5 additional magazines
            reload_time: 2.0,                // 2 seconds to reload
            reload_timer: 0.0,
            is_reloading: false,
        }
    }

    /// Missile - Homing with area damage (1.5x shield, 1.0x hull)
    /// Alt-fire: Missile swarm
    /// Ammo-based: Limited missiles, must resupply
    pub fn missile() -> Self {
        Self {
            weapon_type: WeaponType::Missile,
            damage: 40.0,
            fire_rate: 1.2,
            projectile_speed: 60.0,
            energy_cost: 20.0,
            cooldown_timer: 0.0,
            spread: 0.0,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 1.5,  // 60 damage to shields (60% of total)
            hull_damage_multiplier: 1.0,     // 40 damage to hull (40% of total)
            
            // No heat
            heat: 0.0,
            max_heat: 0.0,
            heat_per_shot: 0.0,
            cooling_rate: 0.0,
            
            // Limited ammo (no reload, just total count)
            current_ammo: 20,
            max_ammo: 20,
            reserve_ammo: 0,
            reload_time: 0.0,
            reload_timer: 0.0,
            is_reloading: false,
        }
    }

    /// Plasma - Balanced heavy weapon (1.2x shield, 1.3x hull)
    /// Alt-fire: Charged shot (charge bar shown in HUD)
    /// Energy-based: No special limits, just energy cost
    pub fn plasma() -> Self {
        Self {
            weapon_type: WeaponType::Plasma,
            damage: 22.0,
            fire_rate: 2.5,
            projectile_speed: 90.0,
            energy_cost: 12.0,
            cooldown_timer: 0.0,
            spread: 0.02,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 1.2,
            hull_damage_multiplier: 1.3,
            
            // No special mechanics
            heat: 0.0,
            max_heat: 0.0,
            heat_per_shot: 0.0,
            cooling_rate: 0.0,
            current_ammo: 0,
            max_ammo: 0,
            reserve_ammo: 0,
            reload_time: 0.0,
            reload_timer: 0.0,
            is_reloading: false,
        }
    }

    /// Railgun - Armor piercing (0.6x shield, 2.5x hull, piercing)
    /// Alt-fire: Overcharged piercing shot
    pub fn railgun() -> Self {
        Self {
            weapon_type: WeaponType::Railgun,
            damage: 60.0,
            fire_rate: 0.8,
            projectile_speed: 300.0,
            energy_cost: 35.0,
            cooldown_timer: 0.0,
            spread: 0.0,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 0.6,  // 36 damage to shields
            hull_damage_multiplier: 2.5,     // 150 damage to hull
            heat: 0.0, max_heat: 0.0, heat_per_shot: 0.0, cooling_rate: 0.0,
            current_ammo: 0, max_ammo: 0, reserve_ammo: 0,
            reload_time: 0.0, reload_timer: 0.0, is_reloading: false,
        }
    }

    /// Ion Cannon - Pure shield killer (5.0x shield, 0.1x hull)
    /// Alt-fire: Shield disruptor pulse
    pub fn ion_cannon() -> Self {
        Self {
            weapon_type: WeaponType::IonCannon,
            damage: 8.0,
            fire_rate: 3.0,
            projectile_speed: 120.0,
            energy_cost: 15.0,
            cooldown_timer: 0.0,
            spread: 0.0,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 5.0,  // 40 damage to shields
            hull_damage_multiplier: 0.1,     // 0.8 damage to hull
            heat: 0.0, max_heat: 0.0, heat_per_shot: 0.0, cooling_rate: 0.0,
            current_ammo: 0, max_ammo: 0, reserve_ammo: 0,
            reload_time: 0.0, reload_timer: 0.0, is_reloading: false,
        }
    }

    /// Flak Cannon - Area denial (1.0x shield, 1.5x hull, area damage)
    /// Alt-fire: Wide spread barrage
    pub fn flak_cannon() -> Self {
        Self {
            weapon_type: WeaponType::FlakCannon,
            damage: 18.0,
            fire_rate: 2.0,
            projectile_speed: 100.0,
            energy_cost: 14.0,
            cooldown_timer: 0.0,
            spread: 0.04,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 1.0,
            hull_damage_multiplier: 1.5,
            heat: 0.0, max_heat: 0.0, heat_per_shot: 0.0, cooling_rate: 0.0,
            current_ammo: 0, max_ammo: 0, reserve_ammo: 0,
            reload_time: 0.0, reload_timer: 0.0, is_reloading: false,
        }
    }

    /// Beam Laser - Continuous damage (2.0x shield, 0.8x hull)
    /// Alt-fire: Focused beam (higher damage, narrower)
    pub fn beam_laser() -> Self {
        Self {
            weapon_type: WeaponType::BeamLaser,
            damage: 7.0,
            fire_rate: 15.0,
            projectile_speed: 250.0,
            energy_cost: 2.5,
            cooldown_timer: 0.0,
            spread: 0.0,
            alt_fire_charge: 0.0,
            shield_damage_multiplier: 2.0,
            hull_damage_multiplier: 0.8,
            heat: 0.0, max_heat: 0.0, heat_per_shot: 0.0, cooling_rate: 0.0,
            current_ammo: 0, max_ammo: 0, reserve_ammo: 0,
            reload_time: 0.0, reload_timer: 0.0, is_reloading: false,
        }
    }
}


