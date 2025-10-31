use bevy::prelude::*;
use crate::components::{
    ship::Player,
    combat::Health,
    ai::Enemy,
    resources::Inventory,
    ship_classes::ClassProgression,
};

/// Track damage dealt for skill point rewards
#[derive(Resource, Default)]
pub struct ProgressionTracker {
    pub total_damage_dealt: f32,
    pub total_enemies_killed: u32,
    pub total_resources_collected: u32,
    pub damage_threshold: f32,
    pub resource_threshold: u32,
}

impl ProgressionTracker {
    pub fn new() -> Self {
        Self {
            total_damage_dealt: 0.0,
            total_enemies_killed: 0,
            total_resources_collected: 0,
            damage_threshold: 1000.0, // 1 point per 1000 damage
            resource_threshold: 10,    // 1 point per 10 resources
        }
    }
    
    pub fn add_damage(&mut self, damage: f32, class_progression: &mut ClassProgression) {
        self.total_damage_dealt += damage;
        
        while self.total_damage_dealt >= self.damage_threshold {
            self.total_damage_dealt -= self.damage_threshold;
            class_progression.award_skill_points(1);
        }
    }
    
    pub fn add_kill(&mut self, kill_value: u32, class_progression: &mut ClassProgression) {
        self.total_enemies_killed += 1;
        class_progression.award_skill_points(kill_value);
    }
    
    pub fn add_resources(&mut self, amount: u32, class_progression: &mut ClassProgression) {
        self.total_resources_collected += amount;
        
        while self.total_resources_collected >= self.resource_threshold {
            self.total_resources_collected -= self.resource_threshold;
            class_progression.award_skill_points(1);
        }
    }
}

/// Event for enemy kills with kill value
#[derive(Event)]
pub struct EnemyKillEvent {
    pub kill_value: u32,
}

/// System to award skill points from enemy kills
pub fn track_enemy_kills_system(
    mut commands: Commands,
    mut kill_events: EventReader<EnemyKillEvent>,
    mut progression: ResMut<ProgressionTracker>,
    mut class_progression: ResMut<ClassProgression>,
) {
    for event in kill_events.read() {
        progression.add_kill(event.kill_value, &mut class_progression);
        println!("[Progression] Enemy killed! Awarded {} skill point(s)", event.kill_value);
    }
}

/// Track health changes to calculate damage dealt
#[derive(Component)]
pub struct PreviousHealth(pub f32);

/// Initialize previous health tracking for enemies
pub fn init_health_tracking_system(
    mut commands: Commands,
    query: Query<(Entity, &Health), (With<Enemy>, Without<PreviousHealth>)>,
) {
    for (entity, health) in query.iter() {
        commands.entity(entity).insert(PreviousHealth(health.current));
    }
}

/// Track damage dealt to award skill points
pub fn track_damage_dealt_system(
    mut progression: ResMut<ProgressionTracker>,
    mut class_progression: ResMut<ClassProgression>,
    mut query: Query<(&Health, &mut PreviousHealth), With<Enemy>>,
) {
    for (health, mut prev_health) in query.iter_mut() {
        if health.current < prev_health.0 {
            let damage = prev_health.0 - health.current;
            progression.add_damage(damage, &mut class_progression);
            prev_health.0 = health.current;
        }
    }
}

/// Track resource collection for skill points
pub fn track_resource_collection_system(
    mut progression: ResMut<ProgressionTracker>,
    mut class_progression: ResMut<ClassProgression>,
    inventory: Res<Inventory>,
) {
    if inventory.is_changed() {
        let total = inventory.scrap_metal + inventory.energy_cores 
                  + inventory.rare_minerals + inventory.tech_components;
        
        if total > progression.total_resources_collected {
            let gained = total - progression.total_resources_collected;
            progression.total_resources_collected = total;
            
            // Award points based on threshold
            let points_to_award = gained / progression.resource_threshold;
            if points_to_award > 0 {
                class_progression.award_skill_points(points_to_award);
            }
        }
    }
}

/// Display skill point gains with floating text (placeholder - would need UI implementation)
pub fn display_skill_point_gain_system(
    class_progression: Res<ClassProgression>,
) {
    if class_progression.is_changed() {
        let available = class_progression.available_points();
        if available > 0 {
            println!("[Progression] ⭐ Skill Points Available: {}", available);
        }
    }
}

