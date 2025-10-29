use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Unique identifier for a star system
pub type SystemId = u32;

/// Star system data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StarSystem {
    pub id: SystemId,
    pub name: String,
    pub position: Vec3,
    pub difficulty: u32,
    pub enemy_preference: Vec<crate::components::ai::EnemyType>,
    pub resource_multipliers: ResourceMultipliers,
    pub planets: Vec<PlanetData>,
    pub connected_systems: Vec<SystemId>,
}

/// Resource multipliers for a system
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceMultipliers {
    pub scrap_metal: f32,
    pub energy_cores: f32,
    pub rare_minerals: f32,
    pub tech_components: f32,
}

impl Default for ResourceMultipliers {
    fn default() -> Self {
        Self {
            scrap_metal: 1.0,
            energy_cores: 1.0,
            rare_minerals: 1.0,
            tech_components: 1.0,
        }
    }
}

/// Planet configuration data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanetData {
    pub size: f32,
    pub color: [f32; 3], // RGB
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub orbit_offset: f32, // Starting angle in radians
}

/// Component marker for planets in the scene
#[derive(Component)]
pub struct Planet {
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub orbit_offset: f32,
    pub center: Vec3,
}

/// Component for system node visuals in galaxy map
#[derive(Component)]
pub struct SystemNode {
    pub system_id: SystemId,
}

/// Component for connection lines between systems
#[derive(Component)]
pub struct SystemConnection {
    pub from_system: SystemId,
    pub to_system: SystemId,
}

/// Component marking the current system in galaxy map
#[derive(Component)]
pub struct CurrentSystemMarker;

/// Component for galaxy map camera
#[derive(Component)]
pub struct GalaxyMapCamera;

impl StarSystem {
    pub fn new(id: SystemId, position: Vec3, seed: u64) -> Self {
        use rand::Rng;
        use rand::SeedableRng;
        
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed.wrapping_add(id as u64));
        
        // Calculate difficulty based on distance from origin
        let distance_from_start = position.length();
        let difficulty = ((distance_from_start / 50.0) as u32).clamp(1, 10);
        
        // Generate enemy preferences based on difficulty
        let enemy_preference = Self::generate_enemy_types(difficulty, &mut rng);
        
        // Generate resource multipliers
        let resource_multipliers = Self::generate_resource_multipliers(difficulty, &mut rng);
        
        // Generate planets (1-4 per system)
        let planet_count = rng.gen_range(1..=4);
        let planets = (0..planet_count)
            .map(|i| Self::generate_planet(i, &mut rng))
            .collect();
        
        // Generate system name
        let name = Self::generate_name(id, &mut rng);
        
        Self {
            id,
            name,
            position,
            difficulty,
            enemy_preference,
            resource_multipliers,
            planets,
            connected_systems: Vec::new(),
        }
    }
    
    fn generate_name(id: SystemId, rng: &mut impl rand::Rng) -> String {
        let prefixes = ["Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta", "Eta", "Theta"];
        let suffixes = ["Centauri", "Draconis", "Orionis", "Lyrae", "Cygni", "Aquarii", "Phoenicis", "Scorpii"];
        
        let prefix = prefixes[rng.gen_range(0..prefixes.len())];
        let suffix = suffixes[rng.gen_range(0..suffixes.len())];
        
        format!("{} {}-{}", prefix, suffix, id)
    }
    
    fn generate_enemy_types(difficulty: u32, rng: &mut impl rand::Rng) -> Vec<crate::components::ai::EnemyType> {
        use crate::components::ai::EnemyType;
        
        let mut types = vec![EnemyType::Fighter];
        
        if difficulty >= 3 {
            types.push(EnemyType::Corvette);
        }
        if difficulty >= 5 {
            types.push(EnemyType::Frigate);
        }
        if difficulty >= 8 {
            types.push(EnemyType::CapitalShip);
        }
        
        // Shuffle to randomize preference order
        for i in (1..types.len()).rev() {
            let j = rng.gen_range(0..=i);
            types.swap(i, j);
        }
        
        types
    }
    
    fn generate_resource_multipliers(difficulty: u32, rng: &mut impl rand::Rng) -> ResourceMultipliers {
        // Higher difficulty = better rewards
        let base_multiplier = 1.0 + (difficulty as f32 * 0.15);
        
        // Randomly emphasize one resource type
        let emphasis = rng.gen_range(0..4);
        let mut multipliers = ResourceMultipliers::default();
        
        match emphasis {
            0 => multipliers.scrap_metal = base_multiplier * rng.gen_range(1.2..1.8),
            1 => multipliers.energy_cores = base_multiplier * rng.gen_range(1.2..1.8),
            2 => multipliers.rare_minerals = base_multiplier * rng.gen_range(1.2..1.8),
            3 => multipliers.tech_components = base_multiplier * rng.gen_range(1.2..1.8),
            _ => {}
        }
        
        // Apply base multiplier to all
        multipliers.scrap_metal *= base_multiplier.max(1.0);
        multipliers.energy_cores *= base_multiplier.max(1.0);
        multipliers.rare_minerals *= base_multiplier.max(1.0);
        multipliers.tech_components *= base_multiplier.max(1.0);
        
        multipliers
    }
    
    fn generate_planet(index: usize, rng: &mut impl rand::Rng) -> PlanetData {
        let size = rng.gen_range(15.0..40.0);
        let color = [
            rng.gen_range(0.3..1.0),
            rng.gen_range(0.3..1.0),
            rng.gen_range(0.3..1.0),
        ];
        let orbit_radius = 100.0 + (index as f32 * 80.0) + rng.gen_range(-20.0..20.0);
        let orbit_speed = rng.gen_range(0.05..0.15) / (index as f32 + 1.0);
        let orbit_offset = rng.gen_range(0.0..std::f32::consts::TAU);
        
        PlanetData {
            size,
            color,
            orbit_radius,
            orbit_speed,
            orbit_offset,
        }
    }
}

