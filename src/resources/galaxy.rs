use bevy::prelude::*;
use std::collections::HashMap;
use crate::components::galaxy::{StarSystem, SystemId};

/// Galaxy resource - contains all star systems and galaxy state
#[derive(Resource, Clone)]
pub struct Galaxy {
    pub seed: u64,
    pub systems: HashMap<SystemId, StarSystem>,
    pub current_system_id: SystemId,
    pub starting_system_id: SystemId,
}

impl Galaxy {
    pub fn new(seed: u64) -> Self {
        let mut galaxy = Self {
            seed,
            systems: HashMap::new(),
            current_system_id: 0,
            starting_system_id: 0,
        };
        
        galaxy.generate();
        galaxy
    }
    
    /// Generate the entire galaxy with procedural systems
    fn generate(&mut self) {
        use rand::Rng;
        use rand::SeedableRng;
        
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);
        
        // Generate 15 star systems in 3D space
        let system_count = 15;
        let mut systems = Vec::new();
        
        // First system is always at origin (starting system)
        let start_system = StarSystem::new(0, Vec3::ZERO, self.seed);
        systems.push(start_system);
        self.starting_system_id = 0;
        self.current_system_id = 0;
        
        // Generate remaining systems in spherical distribution
        for id in 1..system_count {
            let position = Self::generate_system_position(id, &mut rng);
            let system = StarSystem::new(id, position, self.seed);
            systems.push(system);
        }
        
        // Connect systems based on proximity
        let connections = Self::generate_connections(&systems, &mut rng);
        
        // Apply connections to systems
        for (from_id, to_id) in connections {
            if let Some(system) = systems.iter_mut().find(|s| s.id == from_id) {
                if !system.connected_systems.contains(&to_id) {
                    system.connected_systems.push(to_id);
                }
            }
            // Make connections bidirectional
            if let Some(system) = systems.iter_mut().find(|s| s.id == to_id) {
                if !system.connected_systems.contains(&from_id) {
                    system.connected_systems.push(from_id);
                }
            }
        }
        
        // Ensure starting system has at least one connection
        if systems[0].connected_systems.is_empty() && systems.len() > 1 {
            systems[0].connected_systems.push(1);
            systems[1].connected_systems.push(0);
        }
        
        // Convert to HashMap
        for system in systems {
            self.systems.insert(system.id, system);
        }
        
        println!("[Galaxy] Generated galaxy with {} systems (seed: {})", self.systems.len(), self.seed);
    }
    
    /// Generate position for a star system in 3D space
    fn generate_system_position(id: SystemId, rng: &mut impl rand::Rng) -> Vec3 {
        // Use spherical coordinates for better 3D distribution
        let theta = rng.gen_range(0.0..std::f32::consts::TAU); // Azimuthal angle
        let phi = rng.gen_range(0.0..std::f32::consts::PI); // Polar angle
        
        // Distance increases with ID for progressive difficulty
        let min_distance = 100.0 + ((id as f32) * 30.0);
        let max_distance = min_distance + 80.0;
        let distance = rng.gen_range(min_distance..max_distance);
        
        // Convert spherical to Cartesian
        let x = distance * phi.sin() * theta.cos();
        let y = distance * phi.cos();
        let z = distance * phi.sin() * theta.sin();
        
        Vec3::new(x, y, z)
    }
    
    /// Generate connections between star systems
    fn generate_connections(systems: &[StarSystem], rng: &mut impl rand::Rng) -> Vec<(SystemId, SystemId)> {
        let mut connections = Vec::new();
        
        // Connect each system to its 2-3 nearest neighbors
        for system in systems {
            let mut distances: Vec<(SystemId, f32)> = systems
                .iter()
                .filter(|s| s.id != system.id)
                .map(|s| (s.id, system.position.distance(s.position)))
                .collect();
            
            // Sort by distance
            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            
            // Connect to 2-3 nearest systems
            let connection_count = rng.gen_range(2..=3).min(distances.len());
            for i in 0..connection_count {
                connections.push((system.id, distances[i].0));
            }
        }
        
        connections
    }
    
    /// Get current star system
    pub fn current_system(&self) -> Option<&StarSystem> {
        self.systems.get(&self.current_system_id)
    }
    
    /// Get a system by ID
    pub fn get_system(&self, id: SystemId) -> Option<&StarSystem> {
        self.systems.get(&id)
    }
    
    /// Jump to a new system
    pub fn jump_to_system(&mut self, system_id: SystemId) -> bool {
        if self.systems.contains_key(&system_id) {
            self.current_system_id = system_id;
            println!("[Galaxy] Jumped to system: {}", system_id);
            true
        } else {
            println!("[Galaxy] Failed to jump to system: {} (doesn't exist)", system_id);
            false
        }
    }
}

impl Default for Galaxy {
    fn default() -> Self {
        Self::new(12345) // Default seed
    }
}

