use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use crate::components::ship::*;
use crate::components::combat::*;
use crate::components::resources::Inventory;
use crate::components::upgrades::PlayerUpgrades;
use crate::resources::Galaxy;

/// Save data structure
#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub player_position: Vec3Serializable,
    pub player_rotation: QuatSerializable,
    pub health: f32,
    pub max_health: f32,
    pub shield: f32,
    pub max_shield: f32,
    pub energy: f32,
    pub max_energy: f32,
    pub inventory: Inventory,
    pub upgrades: PlayerUpgrades,
    pub galaxy_seed: u64,
    pub current_system_id: u32,
}

/// Serializable Vec3
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Vec3Serializable {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vec3> for Vec3Serializable {
    fn from(v: Vec3) -> Self {
        Self { x: v.x, y: v.y, z: v.z }
    }
}

impl From<Vec3Serializable> for Vec3 {
    fn from(v: Vec3Serializable) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

/// Serializable Quat
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct QuatSerializable {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<Quat> for QuatSerializable {
    fn from(q: Quat) -> Self {
        Self { x: q.x, y: q.y, z: q.z, w: q.w }
    }
}

impl From<QuatSerializable> for Quat {
    fn from(q: QuatSerializable) -> Self {
        Self::from_xyzw(q.x, q.y, q.z, q.w)
    }
}

/// Get save file path
fn get_save_path() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".space_combat_game");
    fs::create_dir_all(&path).ok();
    path.push("save.json");
    path
}

/// Save game state
pub fn save_game(
    player_query: &Query<(&Transform, &Health, &Shield, &Energy), With<Player>>,
    inventory: &Inventory,
    upgrades: &PlayerUpgrades,
    galaxy: Option<&Galaxy>,
) -> Result<(), String> {
    if let Ok((transform, health, shield, energy)) = player_query.get_single() {
        let (galaxy_seed, current_system_id) = if let Some(galaxy) = galaxy {
            (galaxy.seed, galaxy.current_system_id)
        } else {
            (12345, 0) // Default values if no galaxy
        };
        
        let save_data = SaveData {
            player_position: transform.translation.into(),
            player_rotation: transform.rotation.into(),
            health: health.current,
            max_health: health.max,
            shield: shield.current,
            max_shield: shield.max,
            energy: energy.current,
            max_energy: energy.max,
            inventory: inventory.clone(),
            upgrades: upgrades.clone(),
            galaxy_seed,
            current_system_id,
        };
        
        let json = serde_json::to_string_pretty(&save_data)
            .map_err(|e| format!("Failed to serialize save data: {}", e))?;
        
        let save_path = get_save_path();
        fs::write(&save_path, json)
            .map_err(|e| format!("Failed to write save file: {}", e))?;
        
        println!("[Save/Load System] Game saved to {:?}", save_path);
        Ok(())
    } else {
        Err("Player not found".to_string())
    }
}

/// Load game state
pub fn load_game() -> Result<SaveData, String> {
    let save_path = get_save_path();
    
    if !save_path.exists() {
        return Err("No save file found".to_string());
    }
    
    let json = fs::read_to_string(&save_path)
        .map_err(|e| format!("Failed to read save file: {}", e))?;
    
    let save_data: SaveData = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to deserialize save data: {}", e))?;
    
    println!("[Save/Load System] Game loaded from {:?}", save_path);
    Ok(save_data)
}

/// Apply loaded save data to player
pub fn apply_save_data(
    mut player_query: Query<(&mut Transform, &mut Health, &mut Shield, &mut Energy), With<Player>>,
    mut inventory: ResMut<Inventory>,
    mut upgrades: ResMut<PlayerUpgrades>,
    save_data: SaveData,
) {
    if let Ok((mut transform, mut health, mut shield, mut energy)) = player_query.get_single_mut() {
        transform.translation = save_data.player_position.into();
        transform.rotation = save_data.player_rotation.into();
        
        health.current = save_data.health;
        health.max = save_data.max_health;
        
        shield.current = save_data.shield;
        shield.max = save_data.max_shield;
        
        energy.current = save_data.energy;
        energy.max = save_data.max_energy;
        
        *inventory = save_data.inventory;
        *upgrades = save_data.upgrades;
        
        println!("[Save/Load System] Save data applied to player");
    }
}

/// Check if save file exists
pub fn save_exists() -> bool {
    get_save_path().exists()
}

