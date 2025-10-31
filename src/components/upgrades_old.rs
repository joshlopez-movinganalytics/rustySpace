use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::resources::UpgradeCost;

/// Upgrade category
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpgradeCategory {
    Hull,
    Shields,
    Engines,
    PowerPlant,
    Weapons,
}

/// Specific upgrade types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpgradeType {
    // Hull upgrades
    HullIntegrity1,
    HullIntegrity2,
    HullIntegrity3,
    ArmorPlating1,
    ArmorPlating2,
    
    // Shield upgrades
    ShieldCapacity1,
    ShieldCapacity2,
    ShieldCapacity3,
    ShieldRecharge1,
    ShieldRecharge2,
    
    // Engine upgrades
    EngineSpeed1,
    EngineSpeed2,
    EngineSpeed3,
    Maneuverability1,
    Maneuverability2,
    
    // Power plant upgrades
    PowerCapacity1,
    PowerCapacity2,
    PowerRecharge1,
    PowerRecharge2,
    
    // Weapon upgrades
    WeaponDamage1,
    WeaponDamage2,
    WeaponFireRate1,
    WeaponFireRate2,
    UnlockPlasma,
    UnlockMissile,
    UnlockRailgun,
}

impl UpgradeType {
    pub fn category(&self) -> UpgradeCategory {
        match self {
            Self::HullIntegrity1 | Self::HullIntegrity2 | Self::HullIntegrity3 
            | Self::ArmorPlating1 | Self::ArmorPlating2 => UpgradeCategory::Hull,
            
            Self::ShieldCapacity1 | Self::ShieldCapacity2 | Self::ShieldCapacity3
            | Self::ShieldRecharge1 | Self::ShieldRecharge2 => UpgradeCategory::Shields,
            
            Self::EngineSpeed1 | Self::EngineSpeed2 | Self::EngineSpeed3
            | Self::Maneuverability1 | Self::Maneuverability2 => UpgradeCategory::Engines,
            
            Self::PowerCapacity1 | Self::PowerCapacity2
            | Self::PowerRecharge1 | Self::PowerRecharge2 => UpgradeCategory::PowerPlant,
            
            Self::WeaponDamage1 | Self::WeaponDamage2
            | Self::WeaponFireRate1 | Self::WeaponFireRate2
            | Self::UnlockPlasma | Self::UnlockMissile | Self::UnlockRailgun => UpgradeCategory::Weapons,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Self::HullIntegrity1 => "Hull Integrity I",
            Self::HullIntegrity2 => "Hull Integrity II",
            Self::HullIntegrity3 => "Hull Integrity III",
            Self::ArmorPlating1 => "Armor Plating I",
            Self::ArmorPlating2 => "Armor Plating II",
            
            Self::ShieldCapacity1 => "Shield Capacity I",
            Self::ShieldCapacity2 => "Shield Capacity II",
            Self::ShieldCapacity3 => "Shield Capacity III",
            Self::ShieldRecharge1 => "Shield Recharge I",
            Self::ShieldRecharge2 => "Shield Recharge II",
            
            Self::EngineSpeed1 => "Engine Speed I",
            Self::EngineSpeed2 => "Engine Speed II",
            Self::EngineSpeed3 => "Engine Speed III",
            Self::Maneuverability1 => "Maneuverability I",
            Self::Maneuverability2 => "Maneuverability II",
            
            Self::PowerCapacity1 => "Power Capacity I",
            Self::PowerCapacity2 => "Power Capacity II",
            Self::PowerRecharge1 => "Power Recharge I",
            Self::PowerRecharge2 => "Power Recharge II",
            
            Self::WeaponDamage1 => "Weapon Damage I",
            Self::WeaponDamage2 => "Weapon Damage II",
            Self::WeaponFireRate1 => "Fire Rate I",
            Self::WeaponFireRate2 => "Fire Rate II",
            Self::UnlockPlasma => "Unlock Plasma Cannon",
            Self::UnlockMissile => "Unlock Missile Launcher",
            Self::UnlockRailgun => "Unlock Railgun",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            Self::HullIntegrity1 => "Increase max health by 25%",
            Self::HullIntegrity2 => "Increase max health by 50%",
            Self::HullIntegrity3 => "Increase max health by 100%",
            Self::ArmorPlating1 => "Reduce damage taken by 10%",
            Self::ArmorPlating2 => "Reduce damage taken by 20%",
            
            Self::ShieldCapacity1 => "Increase max shields by 25%",
            Self::ShieldCapacity2 => "Increase max shields by 50%",
            Self::ShieldCapacity3 => "Increase max shields by 100%",
            Self::ShieldRecharge1 => "Increase shield recharge rate by 50%",
            Self::ShieldRecharge2 => "Increase shield recharge rate by 100%",
            
            Self::EngineSpeed1 => "Increase max speed by 20%",
            Self::EngineSpeed2 => "Increase max speed by 40%",
            Self::EngineSpeed3 => "Increase max speed by 60%",
            Self::Maneuverability1 => "Increase turn rate by 30%",
            Self::Maneuverability2 => "Increase turn rate by 60%",
            
            Self::PowerCapacity1 => "Increase max energy by 50%",
            Self::PowerCapacity2 => "Increase max energy by 100%",
            Self::PowerRecharge1 => "Increase energy recharge by 50%",
            Self::PowerRecharge2 => "Increase energy recharge by 100%",
            
            Self::WeaponDamage1 => "Increase weapon damage by 25%",
            Self::WeaponDamage2 => "Increase weapon damage by 50%",
            Self::WeaponFireRate1 => "Increase fire rate by 25%",
            Self::WeaponFireRate2 => "Increase fire rate by 50%",
            Self::UnlockPlasma => "Unlock the Plasma Cannon weapon",
            Self::UnlockMissile => "Unlock the Missile Launcher",
            Self::UnlockRailgun => "Unlock the powerful Railgun",
        }
    }
    
    pub fn cost(&self) -> UpgradeCost {
        match self {
            Self::HullIntegrity1 => UpgradeCost { scrap_metal: 10, energy_cores: 5, rare_minerals: 0, tech_components: 0 },
            Self::HullIntegrity2 => UpgradeCost { scrap_metal: 25, energy_cores: 15, rare_minerals: 5, tech_components: 0 },
            Self::HullIntegrity3 => UpgradeCost { scrap_metal: 50, energy_cores: 30, rare_minerals: 15, tech_components: 10 },
            Self::ArmorPlating1 => UpgradeCost { scrap_metal: 15, energy_cores: 0, rare_minerals: 10, tech_components: 5 },
            Self::ArmorPlating2 => UpgradeCost { scrap_metal: 35, energy_cores: 10, rare_minerals: 20, tech_components: 10 },
            
            Self::ShieldCapacity1 => UpgradeCost { scrap_metal: 5, energy_cores: 15, rare_minerals: 5, tech_components: 0 },
            Self::ShieldCapacity2 => UpgradeCost { scrap_metal: 15, energy_cores: 30, rare_minerals: 15, tech_components: 5 },
            Self::ShieldCapacity3 => UpgradeCost { scrap_metal: 30, energy_cores: 60, rare_minerals: 30, tech_components: 15 },
            Self::ShieldRecharge1 => UpgradeCost { scrap_metal: 5, energy_cores: 20, rare_minerals: 10, tech_components: 5 },
            Self::ShieldRecharge2 => UpgradeCost { scrap_metal: 15, energy_cores: 40, rare_minerals: 20, tech_components: 10 },
            
            Self::EngineSpeed1 => UpgradeCost { scrap_metal: 10, energy_cores: 10, rare_minerals: 5, tech_components: 5 },
            Self::EngineSpeed2 => UpgradeCost { scrap_metal: 25, energy_cores: 25, rare_minerals: 15, tech_components: 10 },
            Self::EngineSpeed3 => UpgradeCost { scrap_metal: 45, energy_cores: 45, rare_minerals: 30, tech_components: 20 },
            Self::Maneuverability1 => UpgradeCost { scrap_metal: 10, energy_cores: 15, rare_minerals: 10, tech_components: 5 },
            Self::Maneuverability2 => UpgradeCost { scrap_metal: 25, energy_cores: 35, rare_minerals: 25, tech_components: 15 },
            
            Self::PowerCapacity1 => UpgradeCost { scrap_metal: 5, energy_cores: 25, rare_minerals: 5, tech_components: 10 },
            Self::PowerCapacity2 => UpgradeCost { scrap_metal: 15, energy_cores: 50, rare_minerals: 15, tech_components: 20 },
            Self::PowerRecharge1 => UpgradeCost { scrap_metal: 5, energy_cores: 30, rare_minerals: 10, tech_components: 10 },
            Self::PowerRecharge2 => UpgradeCost { scrap_metal: 15, energy_cores: 60, rare_minerals: 20, tech_components: 20 },
            
            Self::WeaponDamage1 => UpgradeCost { scrap_metal: 15, energy_cores: 10, rare_minerals: 10, tech_components: 10 },
            Self::WeaponDamage2 => UpgradeCost { scrap_metal: 35, energy_cores: 25, rare_minerals: 25, tech_components: 25 },
            Self::WeaponFireRate1 => UpgradeCost { scrap_metal: 10, energy_cores: 15, rare_minerals: 10, tech_components: 15 },
            Self::WeaponFireRate2 => UpgradeCost { scrap_metal: 25, energy_cores: 35, rare_minerals: 25, tech_components: 35 },
            Self::UnlockPlasma => UpgradeCost { scrap_metal: 20, energy_cores: 20, rare_minerals: 10, tech_components: 20 },
            Self::UnlockMissile => UpgradeCost { scrap_metal: 30, energy_cores: 25, rare_minerals: 20, tech_components: 30 },
            Self::UnlockRailgun => UpgradeCost { scrap_metal: 50, energy_cores: 40, rare_minerals: 40, tech_components: 50 },
        }
    }
    
    pub fn prerequisites(&self) -> Vec<UpgradeType> {
        match self {
            Self::HullIntegrity2 => vec![Self::HullIntegrity1],
            Self::HullIntegrity3 => vec![Self::HullIntegrity2],
            Self::ArmorPlating2 => vec![Self::ArmorPlating1],
            
            Self::ShieldCapacity2 => vec![Self::ShieldCapacity1],
            Self::ShieldCapacity3 => vec![Self::ShieldCapacity2],
            Self::ShieldRecharge2 => vec![Self::ShieldRecharge1],
            
            Self::EngineSpeed2 => vec![Self::EngineSpeed1],
            Self::EngineSpeed3 => vec![Self::EngineSpeed2],
            Self::Maneuverability2 => vec![Self::Maneuverability1],
            
            Self::PowerCapacity2 => vec![Self::PowerCapacity1],
            Self::PowerRecharge2 => vec![Self::PowerRecharge1],
            
            Self::WeaponDamage2 => vec![Self::WeaponDamage1],
            Self::WeaponFireRate2 => vec![Self::WeaponFireRate1],
            Self::UnlockMissile => vec![Self::UnlockPlasma],
            Self::UnlockRailgun => vec![Self::UnlockMissile],
            
            _ => vec![],
        }
    }
}

/// Player upgrades resource
#[derive(Resource, Clone, Default, Serialize, Deserialize)]
pub struct PlayerUpgrades {
    pub purchased: Vec<UpgradeType>,
}

impl PlayerUpgrades {
    pub fn has_upgrade(&self, upgrade: UpgradeType) -> bool {
        self.purchased.contains(&upgrade)
    }
    
    pub fn can_purchase(&self, upgrade: UpgradeType) -> bool {
        if self.has_upgrade(upgrade) {
            return false;
        }
        
        // Check prerequisites
        for prereq in upgrade.prerequisites() {
            if !self.has_upgrade(prereq) {
                return false;
            }
        }
        
        true
    }
    
    pub fn purchase(&mut self, upgrade: UpgradeType) {
        if !self.purchased.contains(&upgrade) {
            self.purchased.push(upgrade);
        }
    }
    
    pub fn get_damage_multiplier(&self) -> f32 {
        let mut multiplier = 1.0;
        if self.has_upgrade(UpgradeType::WeaponDamage1) {
            multiplier *= 1.25;
        }
        if self.has_upgrade(UpgradeType::WeaponDamage2) {
            multiplier *= 1.5;
        }
        multiplier
    }
    
    pub fn get_fire_rate_multiplier(&self) -> f32 {
        let mut multiplier = 1.0;
        if self.has_upgrade(UpgradeType::WeaponFireRate1) {
            multiplier *= 1.25;
        }
        if self.has_upgrade(UpgradeType::WeaponFireRate2) {
            multiplier *= 1.5;
        }
        multiplier
    }
}

