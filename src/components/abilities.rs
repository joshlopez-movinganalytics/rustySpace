use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Special abilities unlocked from capstones
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpecialAbility {
    // Fighter capstone
    QuantumDash,
    
    // Tank capstone
    FortressMode,
    
    // Gunner capstone
    Devastation,
    
    // Stealth capstone
    PhaseShift,
    
    // Sniper capstone
    PerfectShot,
    
    // Missile Tanker capstone
    MissileStorm,
}

impl SpecialAbility {
    pub fn name(&self) -> &'static str {
        match self {
            Self::QuantumDash => "Quantum Dash",
            Self::FortressMode => "Fortress Mode",
            Self::Devastation => "Devastation",
            Self::PhaseShift => "Phase Shift",
            Self::PerfectShot => "Perfect Shot",
            Self::MissileStorm => "Missile Storm",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            Self::QuantumDash => "Instantly teleport 50 units forward (Cooldown: 10s)",
            Self::FortressMode => "Become immobile but invulnerable for 3 seconds (Cooldown: 30s)",
            Self::Devastation => "All weapons fire simultaneously for 2 seconds (Cooldown: 20s)",
            Self::PhaseShift => "Become invisible and invulnerable for 2 seconds (Cooldown: 25s)",
            Self::PerfectShot => "Next shot deals 1000% damage (Cooldown: 15s)",
            Self::MissileStorm => "Launch 50 missiles in all directions (Cooldown: 45s)",
        }
    }
    
    pub fn cooldown(&self) -> f32 {
        match self {
            Self::QuantumDash => 10.0,
            Self::FortressMode => 30.0,
            Self::Devastation => 20.0,
            Self::PhaseShift => 25.0,
            Self::PerfectShot => 15.0,
            Self::MissileStorm => 45.0,
        }
    }
    
    pub fn activation_key(&self) -> KeyCode {
        match self {
            Self::QuantumDash => KeyCode::KeyZ,
            Self::FortressMode => KeyCode::KeyX,
            Self::Devastation => KeyCode::KeyC,
            Self::PhaseShift => KeyCode::KeyV,
            Self::PerfectShot => KeyCode::KeyB,
            Self::MissileStorm => KeyCode::KeyN,
        }
    }
}

/// Component tracking unlocked abilities and their cooldowns
#[derive(Component, Clone, Default, Serialize, Deserialize)]
pub struct AbilityController {
    pub unlocked_abilities: Vec<SpecialAbility>,
    pub cooldowns: Vec<(SpecialAbility, f32)>,
    pub active_effects: Vec<ActiveEffect>,
}

impl AbilityController {
    pub fn new() -> Self {
        Self {
            unlocked_abilities: Vec::new(),
            cooldowns: Vec::new(),
            active_effects: Vec::new(),
        }
    }
    
    pub fn unlock_ability(&mut self, ability: SpecialAbility) {
        if !self.unlocked_abilities.contains(&ability) {
            self.unlocked_abilities.push(ability);
            println!("[Abilities] Unlocked: {}", ability.name());
        }
    }
    
    pub fn is_unlocked(&self, ability: SpecialAbility) -> bool {
        self.unlocked_abilities.contains(&ability)
    }
    
    pub fn is_on_cooldown(&self, ability: SpecialAbility) -> bool {
        self.cooldowns.iter().any(|(a, _)| *a == ability)
    }
    
    pub fn get_cooldown_remaining(&self, ability: SpecialAbility) -> f32 {
        self.cooldowns
            .iter()
            .find(|(a, _)| *a == ability)
            .map(|(_, time)| *time)
            .unwrap_or(0.0)
    }
    
    pub fn activate(&mut self, ability: SpecialAbility) -> bool {
        if self.is_unlocked(ability) && !self.is_on_cooldown(ability) {
            self.cooldowns.push((ability, ability.cooldown()));
            println!("[Abilities] Activated: {}", ability.name());
            true
        } else {
            false
        }
    }
    
    pub fn update_cooldowns(&mut self, delta: f32) {
        self.cooldowns.retain_mut(|(ability, time)| {
            *time -= delta;
            if *time <= 0.0 {
                println!("[Abilities] {} ready!", ability.name());
                false
            } else {
                true
            }
        });
        
        self.active_effects.retain_mut(|effect| {
            effect.remaining_duration -= delta;
            effect.remaining_duration > 0.0
        });
    }
    
    pub fn add_active_effect(&mut self, effect: ActiveEffect) {
        self.active_effects.push(effect);
    }
    
    pub fn has_active_effect(&self, effect_type: AbilityEffectType) -> bool {
        self.active_effects.iter().any(|e| e.effect_type == effect_type)
    }
}

/// Active ability effects
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveEffect {
    pub effect_type: AbilityEffectType,
    pub remaining_duration: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbilityEffectType {
    Invulnerable,
    Invisible,
    AllWeaponsFiring,
    DamageBoost(u32), // Percentage
    Immobile,
}

/// Marker for ability visual effects
#[derive(Component)]
pub struct AbilityVisualEffect {
    pub ability: SpecialAbility,
    pub lifetime: f32,
}

