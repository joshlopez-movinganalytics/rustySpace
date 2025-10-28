use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Resource types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResourceType {
    ScrapMetal,
    EnergyCores,
    RareMinerals,
    TechComponents,
}

/// Loot component for collectible resources
#[derive(Component)]
pub struct Loot {
    pub resource_type: ResourceType,
    pub amount: u32,
}

/// Player inventory resource
#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub scrap_metal: u32,
    pub energy_cores: u32,
    pub rare_minerals: u32,
    pub tech_components: u32,
}

impl Inventory {
    pub fn add_resource(&mut self, resource_type: ResourceType, amount: u32) {
        match resource_type {
            ResourceType::ScrapMetal => self.scrap_metal += amount,
            ResourceType::EnergyCores => self.energy_cores += amount,
            ResourceType::RareMinerals => self.rare_minerals += amount,
            ResourceType::TechComponents => self.tech_components += amount,
        }
    }

    pub fn can_afford(&self, cost: &UpgradeCost) -> bool {
        self.scrap_metal >= cost.scrap_metal
            && self.energy_cores >= cost.energy_cores
            && self.rare_minerals >= cost.rare_minerals
            && self.tech_components >= cost.tech_components
    }

    pub fn deduct(&mut self, cost: &UpgradeCost) {
        self.scrap_metal -= cost.scrap_metal;
        self.energy_cores -= cost.energy_cores;
        self.rare_minerals -= cost.rare_minerals;
        self.tech_components -= cost.tech_components;
    }
}

/// Upgrade cost
#[derive(Clone, Serialize, Deserialize)]
pub struct UpgradeCost {
    pub scrap_metal: u32,
    pub energy_cores: u32,
    pub rare_minerals: u32,
    pub tech_components: u32,
}

