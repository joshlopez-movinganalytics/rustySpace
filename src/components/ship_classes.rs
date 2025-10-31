use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ship class types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShipClass {
    Fighter,
    Tank,
    Gunner,
    Stealth,
    Sniper,
    MissileTanker,
}

impl ShipClass {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Fighter => "Fighter",
            Self::Tank => "Tank",
            Self::Gunner => "Gunner",
            Self::Stealth => "Stealth",
            Self::Sniper => "Sniper",
            Self::MissileTanker => "Missile Tanker",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            Self::Fighter => "Speed, agility, and evasion focused",
            Self::Tank => "Armor, shields, and survivability focused",
            Self::Gunner => "Weapon damage, fire rate, and offense focused",
            Self::Stealth => "Cloak, reduced detection, and hit-and-run tactics",
            Self::Sniper => "Long-range weapons, precision, and charged shots",
            Self::MissileTanker => "Heavy missile payloads, tracking, and AOE damage",
        }
    }
    
    pub fn primary_color(&self) -> Color {
        match self {
            Self::Fighter => Color::srgb(0.2, 0.8, 1.0),  // Cyan
            Self::Tank => Color::srgb(0.6, 0.6, 0.6),     // Gray
            Self::Gunner => Color::srgb(1.0, 0.3, 0.2),   // Red
            Self::Stealth => Color::srgb(0.2, 0.2, 0.4),  // Dark blue
            Self::Sniper => Color::srgb(0.8, 0.8, 0.2),   // Yellow
            Self::MissileTanker => Color::srgb(0.8, 0.4, 0.0), // Orange
        }
    }
    
    pub fn secondary_color(&self) -> Color {
        match self {
            Self::Fighter => Color::srgb(0.1, 0.4, 0.6),
            Self::Tank => Color::srgb(0.4, 0.4, 0.4),
            Self::Gunner => Color::srgb(0.6, 0.1, 0.1),
            Self::Stealth => Color::srgb(0.1, 0.1, 0.2),
            Self::Sniper => Color::srgb(0.4, 0.4, 0.1),
            Self::MissileTanker => Color::srgb(0.4, 0.2, 0.0),
        }
    }
    
    pub fn accent_color(&self) -> Color {
        match self {
            Self::Fighter => Color::srgb(0.4, 1.0, 1.0),
            Self::Tank => Color::srgb(0.8, 0.8, 0.8),
            Self::Gunner => Color::srgb(1.0, 0.6, 0.4),
            Self::Stealth => Color::srgb(0.4, 0.4, 0.6),
            Self::Sniper => Color::srgb(1.0, 1.0, 0.6),
            Self::MissileTanker => Color::srgb(1.0, 0.6, 0.2),
        }
    }
}

/// Track points invested in each class
#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct ClassProgression {
    pub points: HashMap<ShipClass, u32>,
    pub total_skill_points: u32,
    pub spent_skill_points: u32,
}

impl ClassProgression {
    pub fn new() -> Self {
        let mut points = HashMap::new();
        points.insert(ShipClass::Fighter, 0);
        points.insert(ShipClass::Tank, 0);
        points.insert(ShipClass::Gunner, 0);
        points.insert(ShipClass::Stealth, 0);
        points.insert(ShipClass::Sniper, 0);
        points.insert(ShipClass::MissileTanker, 0);
        
        Self {
            points,
            total_skill_points: 0,
            spent_skill_points: 0,
        }
    }
    
    pub fn get_points(&self, class: ShipClass) -> u32 {
        *self.points.get(&class).unwrap_or(&0)
    }
    
    pub fn add_point(&mut self, class: ShipClass) {
        *self.points.entry(class).or_insert(0) += 1;
        self.spent_skill_points += 1;
    }
    
    pub fn available_points(&self) -> u32 {
        self.total_skill_points.saturating_sub(self.spent_skill_points)
    }
    
    pub fn award_skill_points(&mut self, amount: u32) {
        self.total_skill_points += amount;
        println!("[Class Progression] Awarded {} skill points. Total: {}, Available: {}", 
            amount, self.total_skill_points, self.available_points());
    }
}

/// Ship mesh variant types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShipMeshVariant {
    Base,
    Fighter,
    Tank,
    Gunner,
    Stealth,
    Sniper,
    MissileTanker,
}

/// Attachment types for visual customization
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttachmentType {
    // Fighter attachments
    ThrusterWings,
    SpeedBoosters,
    
    // Tank attachments
    ArmorPlating,
    ShieldGenerators,
    
    // Gunner attachments
    WeaponPods,
    AmmoFeeders,
    
    // Stealth attachments
    StealthPanels,
    CloakEmitters,
    
    // Sniper attachments
    AntennaArrays,
    TargetingScopes,
    
    // Missile Tanker attachments
    MissileRacks,
    LauncherPods,
}

/// Visual configuration for a ship
#[derive(Component, Clone, Serialize, Deserialize)]
pub struct ShipVisualConfig {
    pub mesh_variant: ShipMeshVariant,
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
    pub attachments: Vec<AttachmentType>,
    pub scale_modifier: f32,
}

impl Default for ShipVisualConfig {
    fn default() -> Self {
        Self {
            mesh_variant: ShipMeshVariant::Base,
            primary_color: Color::srgb(0.5, 0.5, 0.5),
            secondary_color: Color::srgb(0.3, 0.3, 0.3),
            accent_color: Color::srgb(0.7, 0.7, 0.7),
            attachments: Vec::new(),
            scale_modifier: 1.0,
        }
    }
}

/// Marker for ship attachments
#[derive(Component)]
pub struct ShipAttachment {
    pub attachment_type: AttachmentType,
    pub parent_ship: Entity,
}

/// Stat bonuses from class progression
#[derive(Component, Clone, Default, Serialize, Deserialize)]
pub struct ClassBonuses {
    pub health_multiplier: f32,
    pub shield_multiplier: f32,
    pub speed_multiplier: f32,
    pub turn_rate_multiplier: f32,
    pub damage_multiplier: f32,
    pub fire_rate_multiplier: f32,
    pub energy_multiplier: f32,
    pub shield_recharge_multiplier: f32,
    pub energy_recharge_multiplier: f32,
    pub damage_reduction: f32,
    pub evasion_chance: f32,
    pub critical_chance: f32,
    pub critical_multiplier: f32,
    pub stealth_level: f32,
    pub detection_range_multiplier: f32,
    pub projectile_speed_multiplier: f32,
    pub missile_count_multiplier: f32,
}

impl ClassBonuses {
    pub fn new() -> Self {
        Self {
            health_multiplier: 1.0,
            shield_multiplier: 1.0,
            speed_multiplier: 1.0,
            turn_rate_multiplier: 1.0,
            damage_multiplier: 1.0,
            fire_rate_multiplier: 1.0,
            energy_multiplier: 1.0,
            shield_recharge_multiplier: 1.0,
            energy_recharge_multiplier: 1.0,
            damage_reduction: 0.0,
            evasion_chance: 0.0,
            critical_chance: 0.0,
            critical_multiplier: 1.5,
            stealth_level: 0.0,
            detection_range_multiplier: 1.0,
            projectile_speed_multiplier: 1.0,
            missile_count_multiplier: 1.0,
        }
    }
}

