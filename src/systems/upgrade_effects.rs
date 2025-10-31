use crate::components::{
    upgrades::UpgradeType,
    ship_classes::ClassBonuses,
    combat::{WeaponMount, WeaponType},
};

/// Apply upgrade effect to bonuses - comprehensive implementation for all 220+ nodes
pub fn apply_upgrade_effect(
    upgrade: UpgradeType,
    bonuses: &mut ClassBonuses,
    weapon_mount: &mut WeaponMount,
) -> bool {
    // Track if an ability was unlocked
    let mut ability_unlocked = false;
    
    use UpgradeType::*;
    match upgrade {
        // ===== FIGHTER TREE =====
        FighterEngineBoost1 => bonuses.speed_multiplier += 0.15,
        FighterEngineBoost2 => bonuses.speed_multiplier += 0.25,
        FighterStrafeSpeed1 => bonuses.speed_multiplier += 0.10,
        FighterStrafeSpeed2 => bonuses.speed_multiplier += 0.15,
        FighterRollRate1 => bonuses.turn_rate_multiplier += 0.15,
        
        FighterAfterburner => {
            bonuses.speed_multiplier += 0.20;
            bonuses.evasion_chance += 0.05;
        }
        FighterDriftManeuvers => {
            bonuses.turn_rate_multiplier += 0.20;
            bonuses.evasion_chance += 0.03;
        }
        FighterInertiaDampeners1 => bonuses.turn_rate_multiplier += 0.15,
        FighterInertiaDampeners2 => bonuses.turn_rate_multiplier += 0.25,
        FighterThrustVectoring => bonuses.turn_rate_multiplier += 0.18,
        FighterBoostDuration => bonuses.evasion_chance += 0.05,
        FighterQuickTurn => bonuses.turn_rate_multiplier += 0.12,
        
        FighterVectoredThrusters => {
            bonuses.speed_multiplier += 0.25;
            bonuses.turn_rate_multiplier += 0.20;
        }
        FighterAdvancedFlightComputer => {
            bonuses.turn_rate_multiplier += 0.30;
            bonuses.evasion_chance += 0.08;
        }
        FighterBoostRecharge => bonuses.evasion_chance += 0.05,
        FighterEvasiveManeuvers => bonuses.evasion_chance += 0.10,
        FighterSpeedDemon => bonuses.speed_multiplier += 0.35,
        FighterAgilityMaster => {
            bonuses.turn_rate_multiplier += 0.40;
            bonuses.evasion_chance += 0.12;
        }
        FighterMomentumControl => bonuses.turn_rate_multiplier += 0.25,
        FighterPrecisionFlying => bonuses.evasion_chance += 0.08,
        
        FighterEmergencySpeed => bonuses.speed_multiplier += 0.40,
        FighterCombatAgility => {
            bonuses.turn_rate_multiplier += 0.35;
            bonuses.evasion_chance += 0.15;
        }
        FighterEvasionMatrix => bonuses.evasion_chance += 0.20,
        FighterDodgeRoll => bonuses.evasion_chance += 0.12,
        FighterAfterburnerOverdrive => bonuses.speed_multiplier += 0.45,
        FighterInstantAcceleration => bonuses.speed_multiplier += 0.30,
        FighterZeroGManeuvers => {
            bonuses.turn_rate_multiplier += 0.50;
            bonuses.evasion_chance += 0.10;
        }
        FighterBarrelRoll => bonuses.evasion_chance += 0.10,
        
        FighterMasterPilot => {
            bonuses.speed_multiplier += 0.30;
            bonuses.turn_rate_multiplier += 0.30;
            bonuses.evasion_chance += 0.15;
        }
        FighterQuantumEngines => bonuses.speed_multiplier += 0.60,
        FighterPerfectManeuverability => {
            bonuses.turn_rate_multiplier += 0.70;
            bonuses.evasion_chance += 0.20;
        }
        FighterSpeedOfLight => bonuses.speed_multiplier += 0.80,
        FighterUntouchable => bonuses.evasion_chance += 0.35,
        FighterAceManeuvers => {
            bonuses.turn_rate_multiplier += 0.60;
            bonuses.evasion_chance += 0.25;
        }
        FighterSupersonicBoost => bonuses.speed_multiplier += 0.50,
        FighterApexFighter => {
            bonuses.speed_multiplier += 0.50;
            bonuses.turn_rate_multiplier += 0.50;
            bonuses.evasion_chance += 0.30;
            // Ability unlock handled elsewhere
        }
        
        // ===== TANK TREE =====
        TankHullPlating1 => bonuses.health_multiplier += 0.15,
        TankHullPlating2 => bonuses.health_multiplier += 0.25,
        TankHullPlating3 => bonuses.health_multiplier += 0.40,
        TankShieldCapacity1 => bonuses.shield_multiplier += 0.15,
        TankShieldCapacity2 => bonuses.shield_multiplier += 0.25,
        TankArmorThick1 => bonuses.damage_reduction += 0.08,
        
        TankReinforcedFrame => bonuses.health_multiplier += 0.20,
        TankShieldHardening => bonuses.damage_reduction += 0.10,
        TankDamageReduction1 => bonuses.damage_reduction += 0.10,
        TankDamageReduction2 => bonuses.damage_reduction += 0.15,
        TankShieldBooster => bonuses.shield_multiplier += 0.20,
        TankHullRepair => bonuses.health_multiplier += 0.18,
        TankStructuralIntegrity => {
            bonuses.health_multiplier += 0.15;
            bonuses.damage_reduction += 0.05;
        }
        TankEnergyShields => bonuses.shield_multiplier += 0.18,
        
        TankReactiveArmor => bonuses.damage_reduction += 0.20,
        TankShieldRegeneration => bonuses.shield_recharge_multiplier += 0.30,
        TankAblativeCoating => bonuses.damage_reduction += 0.15,
        TankHardpoints => bonuses.health_multiplier += 0.12,
        TankCompositeArmor => {
            bonuses.health_multiplier += 0.10;
            bonuses.damage_reduction += 0.12;
        }
        TankShieldOverdrive => bonuses.shield_recharge_multiplier += 0.40,
        TankDamageAbsorption => bonuses.damage_reduction += 0.25,
        TankLastStand => {
            bonuses.health_multiplier += 0.15;
            bonuses.damage_reduction += 0.10;
        }
        
        TankFortressModePassive => bonuses.damage_reduction += 0.30,
        TankEmergencyShields => bonuses.shield_multiplier += 0.35,
        TankHeavyArmor => {
            bonuses.health_multiplier += 0.30;
            bonuses.damage_reduction += 0.20;
        }
        TankShieldReflection => bonuses.damage_reduction += 0.15,
        TankImpenetrableHull => {
            bonuses.health_multiplier += 0.40;
            bonuses.damage_reduction += 0.30;
        }
        TankAdaptiveArmor => bonuses.damage_reduction += 0.35,
        TankShieldCapacity3 => bonuses.shield_multiplier += 0.40,
        TankBulwark => {
            bonuses.health_multiplier += 0.25;
            bonuses.shield_multiplier += 0.25;
            bonuses.damage_reduction += 0.20;
        }
        
        TankBastionProtocols => {
            bonuses.health_multiplier += 0.35;
            bonuses.shield_multiplier += 0.35;
            bonuses.damage_reduction += 0.25;
        }
        TankShieldOvercharge => bonuses.shield_recharge_multiplier += 0.60,
        TankUltimateArmor => {
            bonuses.health_multiplier += 0.50;
            bonuses.damage_reduction += 0.40;
        }
        TankIndestructible => {
            bonuses.health_multiplier += 0.60;
            bonuses.shield_multiplier += 0.60;
            bonuses.damage_reduction += 0.50;
        }
        TankShieldBarrier => bonuses.shield_multiplier += 0.50,
        TankPerfectDefense => {
            bonuses.damage_reduction += 0.60;
            bonuses.shield_recharge_multiplier += 0.50;
        }
        TankIronWill => bonuses.damage_reduction += 0.45,
        TankJuggernaut => {
            bonuses.health_multiplier += 1.0;
            bonuses.shield_multiplier += 1.0;
            bonuses.damage_reduction += 0.60;
        }
        
        // ===== GUNNER TREE =====
        GunnerWeaponDamage1 => bonuses.damage_multiplier += 0.15,
        GunnerWeaponDamage2 => bonuses.damage_multiplier += 0.25,
        GunnerWeaponDamage3 => bonuses.damage_multiplier += 0.35,
        GunnerFireRate1 => bonuses.fire_rate_multiplier += 0.15,
        GunnerFireRate2 => bonuses.fire_rate_multiplier += 0.25,
        GunnerAmmoCapacity1 => bonuses.energy_multiplier += 0.10,
        GunnerWeaponHeat1 => bonuses.fire_rate_multiplier += 0.10,
        
        GunnerMultiTargeting => bonuses.damage_multiplier += 0.10,
        GunnerWeaponCooling1 => bonuses.fire_rate_multiplier += 0.12,
        GunnerWeaponCooling2 => bonuses.fire_rate_multiplier += 0.20,
        GunnerAmmoCapacity2 => bonuses.energy_multiplier += 0.20,
        GunnerReloadSpeed => bonuses.fire_rate_multiplier += 0.15,
        GunnerAccuracy => bonuses.critical_chance += 0.05,
        GunnerPenetration1 => bonuses.damage_multiplier += 0.12,
        GunnerSplashDamage => bonuses.damage_multiplier += 0.15,
        
        GunnerPlasmaWeapons => {
            let has_plasma = weapon_mount.weapons.iter().any(|w| matches!(w.weapon_type, WeaponType::Plasma));
            if !has_plasma {
                weapon_mount.weapons.push(crate::components::combat::Weapon::plasma());
                ability_unlocked = true;
            }
        }
        GunnerRailgunUnlock => {
            let has_railgun = weapon_mount.weapons.iter().any(|w| matches!(w.weapon_type, WeaponType::Railgun));
            if !has_railgun {
                weapon_mount.weapons.push(crate::components::combat::Weapon::railgun());
                ability_unlocked = true;
            }
        }
        GunnerCriticalHits1 => bonuses.critical_chance += 0.08,
        GunnerCriticalHits2 => bonuses.critical_chance += 0.15,
        GunnerAmmoEfficiency => bonuses.energy_recharge_multiplier += 0.20,
        GunnerWeaponStabilization => bonuses.critical_chance += 0.10,
        GunnerArmorPiercing => bonuses.damage_multiplier += 0.18,
        GunnerExplosiveRounds => bonuses.damage_multiplier += 0.20,
        
        GunnerWeaponOvercharge => {
            bonuses.damage_multiplier += 0.30;
            bonuses.fire_rate_multiplier += 0.20;
        }
        GunnerDualWielding => bonuses.fire_rate_multiplier += 0.40,
        GunnerPenetratingRounds => bonuses.damage_multiplier += 0.25,
        GunnerRapidFire => bonuses.fire_rate_multiplier += 0.35,
        GunnerCriticalDamage => bonuses.critical_multiplier += 0.50,
        GunnerWeaponSynergy => {
            bonuses.damage_multiplier += 0.20;
            bonuses.fire_rate_multiplier += 0.20;
        }
        GunnerBurstFire => bonuses.fire_rate_multiplier += 0.30,
        GunnerSuppressiveFire => bonuses.fire_rate_multiplier += 0.25,
        
        GunnerMasterGunner => {
            bonuses.damage_multiplier += 0.40;
            bonuses.fire_rate_multiplier += 0.30;
        }
        GunnerChainLightning => bonuses.damage_multiplier += 0.35,
        GunnerDevastationPassive => bonuses.fire_rate_multiplier += 0.50,
        GunnerPerfectAccuracy => bonuses.critical_chance += 0.25,
        GunnerObliterate => bonuses.damage_multiplier += 0.60,
        GunnerWeaponMastery => {
            bonuses.damage_multiplier += 0.50;
            bonuses.fire_rate_multiplier += 0.50;
        }
        GunnerInfiniteAmmo => bonuses.energy_recharge_multiplier += 1.0,
        GunnerArsenalMaster => {
            bonuses.damage_multiplier += 1.0;
            bonuses.fire_rate_multiplier += 1.0;
            bonuses.critical_chance += 0.30;
        }
        
        // ===== STEALTH TREE =====
        StealthSignatureReduction1 => bonuses.stealth_level += 0.15,
        StealthSignatureReduction2 => bonuses.stealth_level += 0.25,
        StealthSilentRunning => {
            bonuses.stealth_level += 0.20;
            bonuses.evasion_chance += 0.05;
        }
        StealthLowProfile => bonuses.stealth_level += 0.18,
        StealthSensorDampening => {
            bonuses.stealth_level += 0.15;
            bonuses.detection_range_multiplier -= 0.10;
        }
        
        StealthCloakField1 => {
            bonuses.stealth_level += 0.25;
            bonuses.detection_range_multiplier -= 0.15;
        }
        StealthCloakField2 => {
            bonuses.stealth_level += 0.35;
            bonuses.detection_range_multiplier -= 0.25;
        }
        StealthRadarJamming => bonuses.detection_range_multiplier -= 0.30,
        StealthHeatMasking => {
            bonuses.stealth_level += 0.20;
            bonuses.detection_range_multiplier -= 0.20;
        }
        StealthVisualCamo => bonuses.stealth_level += 0.22,
        StealthECM => bonuses.detection_range_multiplier -= 0.35,
        StealthGhostProtocols1 => {
            bonuses.stealth_level += 0.30;
            bonuses.evasion_chance += 0.08;
        }
        
        StealthActiveCamouflage => {
            bonuses.stealth_level += 0.45;
            bonuses.detection_range_multiplier -= 0.40;
        }
        StealthHeatSink => bonuses.detection_range_multiplier -= 0.30,
        StealthGhostProtocols2 => {
            bonuses.stealth_level += 0.50;
            bonuses.evasion_chance += 0.15;
        }
        StealthSilentWeapons => bonuses.stealth_level += 0.25,
        StealthCloakDuration => bonuses.stealth_level += 0.20,
        StealthPerfectStealth1 => {
            bonuses.stealth_level += 0.60;
            bonuses.detection_range_multiplier -= 0.50;
        }
        StealthSensorGhost => bonuses.detection_range_multiplier -= 0.60,
        StealthPhantomCloak => {
            bonuses.stealth_level += 0.70;
            bonuses.evasion_chance += 0.20;
        }
        
        StealthPerfectStealth2 => {
            bonuses.stealth_level += 0.80;
            bonuses.detection_range_multiplier -= 0.70;
        }
        StealthAmbushTactics => {
            bonuses.stealth_level += 0.35;
            bonuses.damage_multiplier += 0.20; // Bonus when stealthed
        }
        StealthShadowStrike => {
            bonuses.damage_multiplier += 0.30;
            bonuses.stealth_level += 0.25;
        }
        StealthBackstab => bonuses.damage_multiplier += 0.40,
        StealthInvisibility => {
            bonuses.stealth_level += 1.0;
            bonuses.detection_range_multiplier -= 0.80;
        }
        StealthCloakRecharge => bonuses.stealth_level += 0.15,
        StealthDeception => bonuses.detection_range_multiplier -= 0.50,
        StealthMimicry => bonuses.stealth_level += 0.30,
        
        StealthInfiltrator => {
            bonuses.stealth_level += 0.90;
            bonuses.evasion_chance += 0.25;
        }
        StealthDecoyProjector => bonuses.evasion_chance += 0.20,
        StealthPhaseShiftPassive => {
            bonuses.stealth_level += 0.85;
            bonuses.damage_reduction += 0.10;
        }
        StealthVanish => {
            bonuses.stealth_level += 0.95;
            bonuses.detection_range_multiplier -= 0.90;
        }
        StealthAssassin => {
            bonuses.stealth_level += 0.75;
            bonuses.damage_multiplier += 0.50;
        }
        StealthPerfectCamouflage => {
            bonuses.stealth_level += 1.2;
            bonuses.detection_range_multiplier -= 1.0;
        }
        StealthShadowWalker => {
            bonuses.stealth_level += 0.80;
            bonuses.evasion_chance += 0.30;
        }
        StealthPhantom => {
            bonuses.stealth_level += 1.5;
            bonuses.detection_range_multiplier -= 1.2;
            bonuses.evasion_chance += 0.40;
        }
        
        // ===== SNIPER TREE =====
        SniperRangeExtension1 => bonuses.projectile_speed_multiplier += 0.15,
        SniperRangeExtension2 => bonuses.projectile_speed_multiplier += 0.25,
        SniperRangeExtension3 => bonuses.projectile_speed_multiplier += 0.35,
        SniperPrecisionTargeting => bonuses.critical_chance += 0.10,
        SniperScopeEnhancement1 => bonuses.critical_chance += 0.08,
        SniperProjectileSpeed => bonuses.projectile_speed_multiplier += 0.20,
        
        SniperScopeEnhancement2 => bonuses.critical_chance += 0.15,
        SniperChargeWeapons1 => bonuses.damage_multiplier += 0.18,
        SniperChargeWeapons2 => bonuses.damage_multiplier += 0.30,
        SniperLongShot => bonuses.projectile_speed_multiplier += 0.25,
        SniperFocusFire => bonuses.critical_chance += 0.12,
        SniperPerfectAim1 => bonuses.critical_chance += 0.18,
        SniperRangeCalculator => bonuses.projectile_speed_multiplier += 0.20,
        
        SniperLongRangeRailgun => {
            bonuses.projectile_speed_multiplier += 0.50;
            bonuses.damage_multiplier += 0.25;
        }
        SniperHeadhunter => bonuses.critical_chance += 0.25,
        SniperSteadyAim => bonuses.critical_chance += 0.20,
        SniperChargedShot => bonuses.damage_multiplier += 0.40,
        SniperCriticalRange => bonuses.critical_multiplier += 0.30,
        SniperPerfectAim2 => bonuses.critical_chance += 0.30,
        SniperOverwatch => {
            bonuses.projectile_speed_multiplier += 0.35;
            bonuses.critical_chance += 0.15;
        }
        SniperLongDistance => bonuses.projectile_speed_multiplier += 0.40,
        
        SniperOneShotOneKill => bonuses.critical_multiplier += 0.60,
        SniperPerfectAccuracy => bonuses.critical_chance += 0.40,
        SniperArmorPiercing => {
            bonuses.damage_multiplier += 0.35;
            bonuses.critical_chance += 0.15;
        }
        SniperExecutioner => bonuses.critical_multiplier += 0.80,
        SniperChargedDamage => bonuses.damage_multiplier += 0.60,
        SniperPatientHunter => bonuses.critical_chance += 0.35,
        SniperDeadlyPrecision => {
            bonuses.critical_chance += 0.50;
            bonuses.critical_multiplier += 0.50;
        }
        SniperMarksman => {
            bonuses.critical_chance += 0.40;
            bonuses.projectile_speed_multiplier += 0.30;
        }
        
        SniperUltraRange => bonuses.projectile_speed_multiplier += 0.70,
        SniperChargedDevastation => bonuses.damage_multiplier += 0.90,
        SniperTargetLock => bonuses.critical_chance += 0.60,
        SniperPerfectShotPassive => bonuses.critical_multiplier += 1.0,
        SniperSnipeFromAnywhere => bonuses.projectile_speed_multiplier += 1.0,
        SniperInstantKill => bonuses.critical_multiplier += 1.5,
        SniperGhostShot => {
            bonuses.projectile_speed_multiplier += 0.80;
            bonuses.stealth_level += 0.20;
        }
        SniperDeadeye => {
            bonuses.critical_chance += 1.0;
            bonuses.critical_multiplier += 1.0;
            bonuses.projectile_speed_multiplier += 0.90;
        }
        
        // ===== MISSILE TANKER TREE =====
        MissileCapacity1 => bonuses.missile_count_multiplier += 0.15,
        MissileCapacity2 => bonuses.missile_count_multiplier += 0.25,
        MissileCapacity3 => bonuses.missile_count_multiplier += 0.40,
        MissileReloadSpeed1 => bonuses.energy_recharge_multiplier += 0.15,
        MissileReloadSpeed2 => bonuses.energy_recharge_multiplier += 0.25,
        MissileTrackingBasic => bonuses.projectile_speed_multiplier += 0.10,
        
        MissileTrackingSystems => bonuses.projectile_speed_multiplier += 0.20,
        MissileSwarmMissiles => bonuses.missile_count_multiplier += 0.30,
        MissileMultiLaunch => bonuses.missile_count_multiplier += 0.35,
        MissileAmmoReserves => bonuses.energy_multiplier += 0.25,
        MissileFastReload => bonuses.energy_recharge_multiplier += 0.30,
        MissileLockSpeed => bonuses.projectile_speed_multiplier += 0.15,
        MissileVolley => bonuses.missile_count_multiplier += 0.40,
        
        MissileClusterWarheads => bonuses.damage_multiplier += 0.25,
        MissileHomingAI => bonuses.projectile_speed_multiplier += 0.30,
        MissileDamage1 => bonuses.damage_multiplier += 0.18,
        MissileDamage2 => bonuses.damage_multiplier += 0.30,
        MissileProximityFuse => bonuses.damage_multiplier += 0.20,
        MissileMultiTarget => bonuses.damage_multiplier += 0.22,
        MissileSmartGuidance => {
            bonuses.projectile_speed_multiplier += 0.40;
            bonuses.damage_multiplier += 0.15;
        }
        MissileBarragePassive => bonuses.missile_count_multiplier += 0.50,
        
        MissileBarrage => {
            bonuses.missile_count_multiplier += 0.60;
            bonuses.damage_multiplier += 0.20;
        }
        MissileAOEExplosions => bonuses.damage_multiplier += 0.35,
        MissileFireAndForget => bonuses.missile_count_multiplier += 0.55,
        MissileClusterBombs => bonuses.damage_multiplier += 0.40,
        MissileNuclearWarheads => bonuses.damage_multiplier += 0.70,
        MissileCarpetBombing => {
            bonuses.missile_count_multiplier += 0.70;
            bonuses.damage_multiplier += 0.30;
        }
        MissileSeeker => bonuses.projectile_speed_multiplier += 0.50,
        MissileOverwhelm => {
            bonuses.missile_count_multiplier += 0.60;
            bonuses.damage_multiplier += 0.25;
        }
        
        MissileTacticalNukes => bonuses.damage_multiplier += 1.0,
        MissileSmartMissiles => {
            bonuses.projectile_speed_multiplier += 0.60;
            bonuses.damage_multiplier += 0.50;
        }
        MissileDevastatorWarheads => bonuses.damage_multiplier += 0.90,
        MissileInfiniteMissiles => bonuses.missile_count_multiplier += 2.0,
        MissileStormPassive => bonuses.missile_count_multiplier += 1.0,
        MissileApocalypse => {
            bonuses.missile_count_multiplier += 1.5;
            bonuses.damage_multiplier += 0.80;
        }
        MissileArmageddon => {
            bonuses.missile_count_multiplier += 2.0;
            bonuses.damage_multiplier += 1.2;
        }
        MissileSupremacy => {
            bonuses.missile_count_multiplier += 5.0;
            bonuses.damage_multiplier += 1.5;
            bonuses.projectile_speed_multiplier += 0.80;
        }
    }
    
    ability_unlocked
}

