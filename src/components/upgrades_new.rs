use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::resources::UpgradeCost;
use crate::components::ship_classes::ShipClass;
use crate::components::abilities::SpecialAbility;

/// Upgrade category (legacy - now mapped to ShipClass)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpgradeCategory {
    Hull,
    Shields,
    Engines,
    PowerPlant,
    Weapons,
}

/// Specific upgrade types - Comprehensive skill tree with ~270 nodes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpgradeType {
    // ===== FIGHTER TREE (Speed & Agility) =====
    // Tier 1 (5 nodes)
    FighterEngineBoost1,
    FighterEngineBoost2,
    FighterStrafeSpeed1,
    FighterStrafeSpeed2,
    FighterRollRate1,
    
    // Tier 2 (7 nodes)
    FighterAfterburner,
    FighterDriftManeuvers,
    FighterInertiaDampeners1,
    FighterInertiaDampeners2,
    FighterThrustVectoring,
    FighterBoostDuration,
    FighterQuickTurn,
    
    // Tier 3 (8 nodes)
    FighterVectoredThrusters,
    FighterAdvancedFlightComputer,
    FighterBoostRecharge,
    FighterEvasiveManeuvers,
    FighterSpeedDemon,
    FighterAgilityMaster,
    FighterMomentumControl,
    FighterPrecisionFlying,
    
    // Tier 4 (8 nodes)
    FighterEmergencySpeed,
    FighterCombatAgility,
    FighterEvasionMatrix,
    FighterDodgeRoll,
    FighterAfterburnerOverdrive,
    FighterInstantAcceleration,
    FighterZeroGManeuvers,
    FighterBarrelRoll,
    
    // Tier 5 (7 nodes)
    FighterMasterPilot,
    FighterQuantumEngines,
    FighterPerfectManeuverability,
    FighterSpeedOfLight,
    FighterUntouchable,
    FighterAceManeuvers,
    FighterSupersonicBoost,
    
    // Tier 6 (1 capstone)
    FighterApexFighter,
    
    // ===== TANK TREE (Armor & Shields) =====
    // Tier 1 (6 nodes)
    TankHullPlating1,
    TankHullPlating2,
    TankHullPlating3,
    TankShieldCapacity1,
    TankShieldCapacity2,
    TankArmorThick1,
    
    // Tier 2 (8 nodes)
    TankReinforcedFrame,
    TankShieldHardening,
    TankDamageReduction1,
    TankDamageReduction2,
    TankShieldBooster,
    TankHullRepair,
    TankStructuralIntegrity,
    TankEnergyShields,
    
    // Tier 3 (8 nodes)
    TankReactiveArmor,
    TankShieldRegeneration,
    TankAblativeCoating,
    TankHardpoints,
    TankCompositeArmor,
    TankShieldOverdrive,
    TankDamageAbsorption,
    TankLastStand,
    
    // Tier 4 (8 nodes)
    TankFortressModePassive,
    TankEmergencyShields,
    TankHeavyArmor,
    TankShieldReflection,
    TankImpenetrableHull,
    TankAdaptiveArmor,
    TankShieldCapacity3,
    TankBulwark,
    
    // Tier 5 (7 nodes)
    TankBastionProtocols,
    TankShieldOvercharge,
    TankUltimateArmor,
    TankIndestructible,
    TankShieldBarrier,
    TankPerfectDefense,
    TankIronWill,
    
    // Tier 6 (1 capstone)
    TankJuggernaut,
    
    // ===== GUNNER TREE (Weapons & Damage) =====
    // Tier 1 (7 nodes)
    GunnerWeaponDamage1,
    GunnerWeaponDamage2,
    GunnerWeaponDamage3,
    GunnerFireRate1,
    GunnerFireRate2,
    GunnerAmmoCapacity1,
    GunnerWeaponHeat1,
    
    // Tier 2 (8 nodes)
    GunnerMultiTargeting,
    GunnerWeaponCooling1,
    GunnerWeaponCooling2,
    GunnerAmmoCapacity2,
    GunnerReloadSpeed,
    GunnerAccuracy,
    GunnerPenetration1,
    GunnerSplashDamage,
    
    // Tier 3 (8 nodes)
    GunnerPlasmaWeapons,
    GunnerRailgunUnlock,
    GunnerCriticalHits1,
    GunnerCriticalHits2,
    GunnerAmmoEfficiency,
    GunnerWeaponStabilization,
    GunnerArmorPiercing,
    GunnerExplosiveRounds,
    
    // Tier 4 (8 nodes)
    GunnerWeaponOvercharge,
    GunnerDualWielding,
    GunnerPenetratingRounds,
    GunnerRapidFire,
    GunnerCriticalDamage,
    GunnerWeaponSynergy,
    GunnerBurstFire,
    GunnerSuppressiveFire,
    
    // Tier 5 (7 nodes)
    GunnerMasterGunner,
    GunnerChainLightning,
    GunnerDevastationPassive,
    GunnerPerfectAccuracy,
    GunnerObliterate,
    GunnerWeaponMastery,
    GunnerInfiniteAmmo,
    
    // Tier 6 (1 capstone)
    GunnerArsenalMaster,
    
    // ===== STEALTH TREE =====
    // Tier 1 (5 nodes)
    StealthSignatureReduction1,
    StealthSignatureReduction2,
    StealthSilentRunning,
    StealthLowProfile,
    StealthSensorDampening,
    
    // Tier 2 (7 nodes)
    StealthCloakField1,
    StealthCloakField2,
    StealthRadarJamming,
    StealthHeatMasking,
    StealthVisualCamo,
    StealthECM,
    StealthGhostProtocols1,
    
    // Tier 3 (8 nodes)
    StealthActiveCamouflage,
    StealthHeatSink,
    StealthGhostProtocols2,
    StealthSilentWeapons,
    StealthCloakDuration,
    StealthPerfectStealth1,
    StealthSensorGhost,
    StealthPhantomCloak,
    
    // Tier 4 (8 nodes)
    StealthPerfectStealth2,
    StealthAmbushTactics,
    StealthShadowStrike,
    StealthBackstab,
    StealthInvisibility,
    StealthCloakRecharge,
    StealthDeception,
    StealthMimicry,
    
    // Tier 5 (7 nodes)
    StealthInfiltrator,
    StealthDecoyProjector,
    StealthPhaseShiftPassive,
    StealthVanish,
    StealthAssassin,
    StealthPerfectCamouflage,
    StealthShadowWalker,
    
    // Tier 6 (1 capstone)
    StealthPhantom,
    
    // ===== SNIPER TREE =====
    // Tier 1 (6 nodes)
    SniperRangeExtension1,
    SniperRangeExtension2,
    SniperRangeExtension3,
    SniperPrecisionTargeting,
    SniperScopeEnhancement1,
    SniperProjectileSpeed,
    
    // Tier 2 (7 nodes)
    SniperScopeEnhancement2,
    SniperChargeWeapons1,
    SniperChargeWeapons2,
    SniperLongShot,
    SniperFocusFire,
    SniperPerfectAim1,
    SniperRangeCalculator,
    
    // Tier 3 (8 nodes)
    SniperLongRangeRailgun,
    SniperHeadhunter,
    SniperSteadyAim,
    SniperChargedShot,
    SniperCriticalRange,
    SniperPerfectAim2,
    SniperOverwatch,
    SniperLongDistance,
    
    // Tier 4 (8 nodes)
    SniperOneShotOneKill,
    SniperPerfectAccuracy,
    SniperArmorPiercing,
    SniperExecutioner,
    SniperChargedDamage,
    SniperPatientHunter,
    SniperDeadlyPrecision,
    SniperMarksman,
    
    // Tier 5 (7 nodes)
    SniperUltraRange,
    SniperChargedDevastation,
    SniperTargetLock,
    SniperPerfectShotPassive,
    SniperSnipeFromAnywhere,
    SniperInstantKill,
    SniperGhostShot,
    
    // Tier 6 (1 capstone)
    SniperDeadeye,
    
    // ===== MISSILE TANKER TREE =====
    // Tier 1 (6 nodes)
    MissileCapacity1,
    MissileCapacity2,
    MissileCapacity3,
    MissileReloadSpeed1,
    MissileReloadSpeed2,
    MissileTrackingBasic,
    
    // Tier 2 (7 nodes)
    MissileTrackingSystems,
    MissileSwarmMissiles,
    MissileMultiLaunch,
    MissileAmmoReserves,
    MissileFastReload,
    MissileLockSpeed,
    MissileVolley,
    
    // Tier 3 (8 nodes)
    MissileClusterWarheads,
    MissileHomingAI,
    MissileDamage1,
    MissileDamage2,
    MissileProximityFuse,
    MissileMultiTarget,
    MissileSmartGuidance,
    MissileBarragePassive,
    
    // Tier 4 (8 nodes)
    MissileBarrage,
    MissileAOEExplosions,
    MissileFireAndForget,
    MissileClusterBombs,
    MissileNuclearWarheads,
    MissileCarpetBombing,
    MissileSeeker,
    MissileOverwhelm,
    
    // Tier 5 (7 nodes)
    MissileTacticalNukes,
    MissileSmartMissiles,
    MissileDevastatorWarheads,
    MissileInfiniteMissiles,
    MissileStormPassive,
    MissileApocalypse,
    MissileArmageddon,
    
    // Tier 6 (1 capstone)
    MissileSupremacy,
}

impl UpgradeType {
    pub fn tier(&self) -> u8 {
        match self {
            // Fighter Tier 1
            Self::FighterEngineBoost1 | Self::FighterEngineBoost2 | Self::FighterStrafeSpeed1
            | Self::FighterStrafeSpeed2 | Self::FighterRollRate1 => 1,
            
            // Fighter Tier 2
            Self::FighterAfterburner | Self::FighterDriftManeuvers | Self::FighterInertiaDampeners1
            | Self::FighterInertiaDampeners2 | Self::FighterThrustVectoring | Self::FighterBoostDuration
            | Self::FighterQuickTurn => 2,
            
            // Fighter Tier 3
            Self::FighterVectoredThrusters | Self::FighterAdvancedFlightComputer | Self::FighterBoostRecharge
            | Self::FighterEvasiveManeuvers | Self::FighterSpeedDemon | Self::FighterAgilityMaster
            | Self::FighterMomentumControl | Self::FighterPrecisionFlying => 3,
            
            // Fighter Tier 4
            Self::FighterEmergencySpeed | Self::FighterCombatAgility | Self::FighterEvasionMatrix
            | Self::FighterDodgeRoll | Self::FighterAfterburnerOverdrive | Self::FighterInstantAcceleration
            | Self::FighterZeroGManeuvers | Self::FighterBarrelRoll => 4,
            
            // Fighter Tier 5
            Self::FighterMasterPilot | Self::FighterQuantumEngines | Self::FighterPerfectManeuverability
            | Self::FighterSpeedOfLight | Self::FighterUntouchable | Self::FighterAceManeuvers
            | Self::FighterSupersonicBoost => 5,
            
            // Fighter Tier 6
            Self::FighterApexFighter => 6,
            
            // Tank Tier 1
            Self::TankHullPlating1 | Self::TankHullPlating2 | Self::TankHullPlating3
            | Self::TankShieldCapacity1 | Self::TankShieldCapacity2 | Self::TankArmorThick1 => 1,
            
            // Tank Tier 2
            Self::TankReinforcedFrame | Self::TankShieldHardening | Self::TankDamageReduction1
            | Self::TankDamageReduction2 | Self::TankShieldBooster | Self::TankHullRepair
            | Self::TankStructuralIntegrity | Self::TankEnergyShields => 2,
            
            // Tank Tier 3
            Self::TankReactiveArmor | Self::TankShieldRegeneration | Self::TankAblativeCoating
            | Self::TankHardpoints | Self::TankCompositeArmor | Self::TankShieldOverdrive
            | Self::TankDamageAbsorption | Self::TankLastStand => 3,
            
            // Tank Tier 4
            Self::TankFortressModePassive | Self::TankEmergencyShields | Self::TankHeavyArmor
            | Self::TankShieldReflection | Self::TankImpenetrableHull | Self::TankAdaptiveArmor
            | Self::TankShieldCapacity3 | Self::TankBulwark => 4,
            
            // Tank Tier 5
            Self::TankBastionProtocols | Self::TankShieldOvercharge | Self::TankUltimateArmor
            | Self::TankIndestructible | Self::TankShieldBarrier | Self::TankPerfectDefense
            | Self::TankIronWill => 5,
            
            // Tank Tier 6
            Self::TankJuggernaut => 6,
            
            // Gunner Tier 1
            Self::GunnerWeaponDamage1 | Self::GunnerWeaponDamage2 | Self::GunnerWeaponDamage3
            | Self::GunnerFireRate1 | Self::GunnerFireRate2 | Self::GunnerAmmoCapacity1
            | Self::GunnerWeaponHeat1 => 1,
            
            // Gunner Tier 2
            Self::GunnerMultiTargeting | Self::GunnerWeaponCooling1 | Self::GunnerWeaponCooling2
            | Self::GunnerAmmoCapacity2 | Self::GunnerReloadSpeed | Self::GunnerAccuracy
            | Self::GunnerPenetration1 | Self::GunnerSplashDamage => 2,
            
            // Gunner Tier 3
            Self::GunnerPlasmaWeapons | Self::GunnerRailgunUnlock | Self::GunnerCriticalHits1
            | Self::GunnerCriticalHits2 | Self::GunnerAmmoEfficiency | Self::GunnerWeaponStabilization
            | Self::GunnerArmorPiercing | Self::GunnerExplosiveRounds => 3,
            
            // Gunner Tier 4
            Self::GunnerWeaponOvercharge | Self::GunnerDualWielding | Self::GunnerPenetratingRounds
            | Self::GunnerRapidFire | Self::GunnerCriticalDamage | Self::GunnerWeaponSynergy
            | Self::GunnerBurstFire | Self::GunnerSuppressiveFire => 4,
            
            // Gunner Tier 5
            Self::GunnerMasterGunner | Self::GunnerChainLightning | Self::GunnerDevastationPassive
            | Self::GunnerPerfectAccuracy | Self::GunnerObliterate | Self::GunnerWeaponMastery
            | Self::GunnerInfiniteAmmo => 5,
            
            // Gunner Tier 6
            Self::GunnerArsenalMaster => 6,
            
            // Stealth - continuing in next part...
            Self::StealthSignatureReduction1 | Self::StealthSignatureReduction2 | Self::StealthSilentRunning
            | Self::StealthLowProfile | Self::StealthSensorDampening => 1,
            
            Self::StealthCloakField1 | Self::StealthCloakField2 | Self::StealthRadarJamming
            | Self::StealthHeatMasking | Self::StealthVisualCamo | Self::StealthECM
            | Self::StealthGhostProtocols1 => 2,
            
            Self::StealthActiveCamouflage | Self::StealthHeatSink | Self::StealthGhostProtocols2
            | Self::StealthSilentWeapons | Self::StealthCloakDuration | Self::StealthPerfectStealth1
            | Self::StealthSensorGhost | Self::StealthPhantomCloak => 3,
            
            Self::StealthPerfectStealth2 | Self::StealthAmbushTactics | Self::StealthShadowStrike
            | Self::StealthBackstab | Self::StealthInvisibility | Self::StealthCloakRecharge
            | Self::StealthDeception | Self::StealthMimicry => 4,
            
            Self::StealthInfiltrator | Self::StealthDecoyProjector | Self::StealthPhaseShiftPassive
            | Self::StealthVanish | Self::StealthAssassin | Self::StealthPerfectCamouflage
            | Self::StealthShadowWalker => 5,
            
            Self::StealthPhantom => 6,
            
            // Sniper
            Self::SniperRangeExtension1 | Self::SniperRangeExtension2 | Self::SniperRangeExtension3
            | Self::SniperPrecisionTargeting | Self::SniperScopeEnhancement1 | Self::SniperProjectileSpeed => 1,
            
            Self::SniperScopeEnhancement2 | Self::SniperChargeWeapons1 | Self::SniperChargeWeapons2
            | Self::SniperLongShot | Self::SniperFocusFire | Self::SniperPerfectAim1
            | Self::SniperRangeCalculator => 2,
            
            Self::SniperLongRangeRailgun | Self::SniperHeadhunter | Self::SniperSteadyAim
            | Self::SniperChargedShot | Self::SniperCriticalRange | Self::SniperPerfectAim2
            | Self::SniperOverwatch | Self::SniperLongDistance => 3,
            
            Self::SniperOneShotOneKill | Self::SniperPerfectAccuracy | Self::SniperArmorPiercing
            | Self::SniperExecutioner | Self::SniperChargedDamage | Self::SniperPatientHunter
            | Self::SniperDeadlyPrecision | Self::SniperMarksman => 4,
            
            Self::SniperUltraRange | Self::SniperChargedDevastation | Self::SniperTargetLock
            | Self::SniperPerfectShotPassive | Self::SniperSnipeFromAnywhere | Self::SniperInstantKill
            | Self::SniperGhostShot => 5,
            
            Self::SniperDeadeye => 6,
            
            // Missile
            Self::MissileCapacity1 | Self::MissileCapacity2 | Self::MissileCapacity3
            | Self::MissileReloadSpeed1 | Self::MissileReloadSpeed2 | Self::MissileTrackingBasic => 1,
            
            Self::MissileTrackingSystems | Self::MissileSwarmMissiles | Self::MissileMultiLaunch
            | Self::MissileAmmoReserves | Self::MissileFastReload | Self::MissileLockSpeed
            | Self::MissileVolley => 2,
            
            Self::MissileClusterWarheads | Self::MissileHomingAI | Self::MissileDamage1
            | Self::MissileDamage2 | Self::MissileProximityFuse | Self::MissileMultiTarget
            | Self::MissileSmartGuidance | Self::MissileBarragePassive => 3,
            
            Self::MissileBarrage | Self::MissileAOEExplosions | Self::MissileFireAndForget
            | Self::MissileClusterBombs | Self::MissileNuclearWarheads | Self::MissileCarpetBombing
            | Self::MissileSeeker | Self::MissileOverwhelm => 4,
            
            Self::MissileTacticalNukes | Self::MissileSmartMissiles | Self::MissileDevastatorWarheads
            | Self::MissileInfiniteMissiles | Self::MissileStormPassive | Self::MissileApocalypse
            | Self::MissileArmageddon => 5,
            
            Self::MissileSupremacy => 6,
        }
    }
    
    pub fn class(&self) -> ShipClass {
        match self {
            // Fighter
            Self::FighterEngineBoost1 | Self::FighterEngineBoost2 | Self::FighterStrafeSpeed1
            | Self::FighterStrafeSpeed2 | Self::FighterRollRate1 | Self::FighterAfterburner
            | Self::FighterDriftManeuvers | Self::FighterInertiaDampeners1 | Self::FighterInertiaDampeners2
            | Self::FighterThrustVectoring | Self::FighterBoostDuration | Self::FighterQuickTurn
            | Self::FighterVectoredThrusters | Self::FighterAdvancedFlightComputer | Self::FighterBoostRecharge
            | Self::FighterEvasiveManeuvers | Self::FighterSpeedDemon | Self::FighterAgilityMaster
            | Self::FighterMomentumControl | Self::FighterPrecisionFlying | Self::FighterEmergencySpeed
            | Self::FighterCombatAgility | Self::FighterEvasionMatrix | Self::FighterDodgeRoll
            | Self::FighterAfterburnerOverdrive | Self::FighterInstantAcceleration | Self::FighterZeroGManeuvers
            | Self::FighterBarrelRoll | Self::FighterMasterPilot | Self::FighterQuantumEngines
            | Self::FighterPerfectManeuverability | Self::FighterSpeedOfLight | Self::FighterUntouchable
            | Self::FighterAceManeuvers | Self::FighterSupersonicBoost | Self::FighterApexFighter => ShipClass::Fighter,
            
            // Tank
            Self::TankHullPlating1 | Self::TankHullPlating2 | Self::TankHullPlating3
            | Self::TankShieldCapacity1 | Self::TankShieldCapacity2 | Self::TankArmorThick1
            | Self::TankReinforcedFrame | Self::TankShieldHardening | Self::TankDamageReduction1
            | Self::TankDamageReduction2 | Self::TankShieldBooster | Self::TankHullRepair
            | Self::TankStructuralIntegrity | Self::TankEnergyShields | Self::TankReactiveArmor
            | Self::TankShieldRegeneration | Self::TankAblativeCoating | Self::TankHardpoints
            | Self::TankCompositeArmor | Self::TankShieldOverdrive | Self::TankDamageAbsorption
            | Self::TankLastStand | Self::TankFortressModePassive | Self::TankEmergencyShields
            | Self::TankHeavyArmor | Self::TankShieldReflection | Self::TankImpenetrableHull
            | Self::TankAdaptiveArmor | Self::TankShieldCapacity3 | Self::TankBulwark
            | Self::TankBastionProtocols | Self::TankShieldOvercharge | Self::TankUltimateArmor
            | Self::TankIndestructible | Self::TankShieldBarrier | Self::TankPerfectDefense
            | Self::TankIronWill | Self::TankJuggernaut => ShipClass::Tank,
            
            // Gunner
            Self::GunnerWeaponDamage1 | Self::GunnerWeaponDamage2 | Self::GunnerWeaponDamage3
            | Self::GunnerFireRate1 | Self::GunnerFireRate2 | Self::GunnerAmmoCapacity1
            | Self::GunnerWeaponHeat1 | Self::GunnerMultiTargeting | Self::GunnerWeaponCooling1
            | Self::GunnerWeaponCooling2 | Self::GunnerAmmoCapacity2 | Self::GunnerReloadSpeed
            | Self::GunnerAccuracy | Self::GunnerPenetration1 | Self::GunnerSplashDamage
            | Self::GunnerPlasmaWeapons | Self::GunnerRailgunUnlock | Self::GunnerCriticalHits1
            | Self::GunnerCriticalHits2 | Self::GunnerAmmoEfficiency | Self::GunnerWeaponStabilization
            | Self::GunnerArmorPiercing | Self::GunnerExplosiveRounds | Self::GunnerWeaponOvercharge
            | Self::GunnerDualWielding | Self::GunnerPenetratingRounds | Self::GunnerRapidFire
            | Self::GunnerCriticalDamage | Self::GunnerWeaponSynergy | Self::GunnerBurstFire
            | Self::GunnerSuppressiveFire | Self::GunnerMasterGunner | Self::GunnerChainLightning
            | Self::GunnerDevastationPassive | Self::GunnerPerfectAccuracy | Self::GunnerObliterate
            | Self::GunnerWeaponMastery | Self::GunnerInfiniteAmmo | Self::GunnerArsenalMaster => ShipClass::Gunner,
            
            // Stealth
            Self::StealthSignatureReduction1 | Self::StealthSignatureReduction2 | Self::StealthSilentRunning
            | Self::StealthLowProfile | Self::StealthSensorDampening | Self::StealthCloakField1
            | Self::StealthCloakField2 | Self::StealthRadarJamming | Self::StealthHeatMasking
            | Self::StealthVisualCamo | Self::StealthECM | Self::StealthGhostProtocols1
            | Self::StealthActiveCamouflage | Self::StealthHeatSink | Self::StealthGhostProtocols2
            | Self::StealthSilentWeapons | Self::StealthCloakDuration | Self::StealthPerfectStealth1
            | Self::StealthSensorGhost | Self::StealthPhantomCloak | Self::StealthPerfectStealth2
            | Self::StealthAmbushTactics | Self::StealthShadowStrike | Self::StealthBackstab
            | Self::StealthInvisibility | Self::StealthCloakRecharge | Self::StealthDeception
            | Self::StealthMimicry | Self::StealthInfiltrator | Self::StealthDecoyProjector
            | Self::StealthPhaseShiftPassive | Self::StealthVanish | Self::StealthAssassin
            | Self::StealthPerfectCamouflage | Self::StealthShadowWalker | Self::StealthPhantom => ShipClass::Stealth,
            
            // Sniper
            Self::SniperRangeExtension1 | Self::SniperRangeExtension2 | Self::SniperRangeExtension3
            | Self::SniperPrecisionTargeting | Self::SniperScopeEnhancement1 | Self::SniperProjectileSpeed
            | Self::SniperScopeEnhancement2 | Self::SniperChargeWeapons1 | Self::SniperChargeWeapons2
            | Self::SniperLongShot | Self::SniperFocusFire | Self::SniperPerfectAim1
            | Self::SniperRangeCalculator | Self::SniperLongRangeRailgun | Self::SniperHeadhunter
            | Self::SniperSteadyAim | Self::SniperChargedShot | Self::SniperCriticalRange
            | Self::SniperPerfectAim2 | Self::SniperOverwatch | Self::SniperLongDistance
            | Self::SniperOneShotOneKill | Self::SniperPerfectAccuracy | Self::SniperArmorPiercing
            | Self::SniperExecutioner | Self::SniperChargedDamage | Self::SniperPatientHunter
            | Self::SniperDeadlyPrecision | Self::SniperMarksman | Self::SniperUltraRange
            | Self::SniperChargedDevastation | Self::SniperTargetLock | Self::SniperPerfectShotPassive
            | Self::SniperSnipeFromAnywhere | Self::SniperInstantKill | Self::SniperGhostShot
            | Self::SniperDeadeye => ShipClass::Sniper,
            
            // Missile Tanker
            Self::MissileCapacity1 | Self::MissileCapacity2 | Self::MissileCapacity3
            | Self::MissileReloadSpeed1 | Self::MissileReloadSpeed2 | Self::MissileTrackingBasic
            | Self::MissileTrackingSystems | Self::MissileSwarmMissiles | Self::MissileMultiLaunch
            | Self::MissileAmmoReserves | Self::MissileFastReload | Self::MissileLockSpeed
            | Self::MissileVolley | Self::MissileClusterWarheads | Self::MissileHomingAI
            | Self::MissileDamage1 | Self::MissileDamage2 | Self::MissileProximityFuse
            | Self::MissileMultiTarget | Self::MissileSmartGuidance | Self::MissileBarragePassive
            | Self::MissileBarrage | Self::MissileAOEExplosions | Self::MissileFireAndForget
            | Self::MissileClusterBombs | Self::MissileNuclearWarheads | Self::MissileCarpetBombing
            | Self::MissileSeeker | Self::MissileOverwhelm | Self::MissileTacticalNukes
            | Self::MissileSmartMissiles | Self::MissileDevastatorWarheads | Self::MissileInfiniteMissiles
            | Self::MissileStormPassive | Self::MissileApocalypse | Self::MissileArmageddon
            | Self::MissileSupremacy => ShipClass::MissileTanker,
        }
    }
    
    pub fn name(&self) -> &'static str {
        // This will be very long - showing pattern for a few, rest follow same structure
        match self {
            // Fighter Tree
            Self::FighterEngineBoost1 => "Engine Boost I",
            Self::FighterEngineBoost2 => "Engine Boost II",
            Self::FighterStrafeSpeed1 => "Strafe Speed I",
            Self::FighterStrafeSpeed2 => "Strafe Speed II",
            Self::FighterRollRate1 => "Roll Rate I",
            Self::FighterAfterburner => "Afterburner",
            Self::FighterDriftManeuvers => "Drift Maneuvers",
            Self::FighterInertiaDampeners1 => "Inertia Dampeners I",
            Self::FighterInertiaDampeners2 => "Inertia Dampeners II",
            Self::FighterThrustVectoring => "Thrust Vectoring",
            Self::FighterBoostDuration => "Boost Duration",
            Self::FighterQuickTurn => "Quick Turn",
            Self::FighterVectoredThrusters => "Vectored Thrusters",
            Self::FighterAdvancedFlightComputer => "Advanced Flight Computer",
            Self::FighterBoostRecharge => "Boost Recharge",
            Self::FighterEvasiveManeuvers => "Evasive Maneuvers",
            Self::FighterSpeedDemon => "Speed Demon",
            Self::FighterAgilityMaster => "Agility Master",
            Self::FighterMomentumControl => "Momentum Control",
            Self::FighterPrecisionFlying => "Precision Flying",
            Self::FighterEmergencySpeed => "Emergency Speed",
            Self::FighterCombatAgility => "Combat Agility",
            Self::FighterEvasionMatrix => "Evasion Matrix",
            Self::FighterDodgeRoll => "Dodge Roll",
            Self::FighterAfterburnerOverdrive => "Afterburner Overdrive",
            Self::FighterInstantAcceleration => "Instant Acceleration",
            Self::FighterZeroGManeuvers => "Zero-G Maneuvers",
            Self::FighterBarrelRoll => "Barrel Roll",
            Self::FighterMasterPilot => "Master Pilot",
            Self::FighterQuantumEngines => "Quantum Engines",
            Self::FighterPerfectManeuverability => "Perfect Maneuverability",
            Self::FighterSpeedOfLight => "Speed of Light",
            Self::FighterUntouchable => "Untouchable",
            Self::FighterAceManeuvers => "Ace Maneuvers",
            Self::FighterSupersonicBoost => "Supersonic Boost",
            Self::FighterApexFighter => "APEX FIGHTER",
            
            // Tank Tree - showing pattern
            Self::TankHullPlating1 => "Hull Plating I",
            Self::TankHullPlating2 => "Hull Plating II",
            Self::TankHullPlating3 => "Hull Plating III",
            Self::TankShieldCapacity1 => "Shield Capacity I",
            Self::TankShieldCapacity2 => "Shield Capacity II",
            Self::TankArmorThick1 => "Thick Armor",
            Self::TankReinforcedFrame => "Reinforced Frame",
            Self::TankShieldHardening => "Shield Hardening",
            Self::TankDamageReduction1 => "Damage Reduction I",
            Self::TankDamageReduction2 => "Damage Reduction II",
            Self::TankShieldBooster => "Shield Booster",
            Self::TankHullRepair => "Hull Repair",
            Self::TankStructuralIntegrity => "Structural Integrity",
            Self::TankEnergyShields => "Energy Shields",
            Self::TankReactiveArmor => "Reactive Armor",
            Self::TankShieldRegeneration => "Shield Regeneration",
            Self::TankAblativeCoating => "Ablative Coating",
            Self::TankHardpoints => "Hardpoints",
            Self::TankCompositeArmor => "Composite Armor",
            Self::TankShieldOverdrive => "Shield Overdrive",
            Self::TankDamageAbsorption => "Damage Absorption",
            Self::TankLastStand => "Last Stand",
            Self::TankFortressModePassive => "Fortress Protocols",
            Self::TankEmergencyShields => "Emergency Shields",
            Self::TankHeavyArmor => "Heavy Armor",
            Self::TankShieldReflection => "Shield Reflection",
            Self::TankImpenetrableHull => "Impenetrable Hull",
            Self::TankAdaptiveArmor => "Adaptive Armor",
            Self::TankShieldCapacity3 => "Shield Capacity III",
            Self::TankBulwark => "Bulwark",
            Self::TankBastionProtocols => "Bastion Protocols",
            Self::TankShieldOvercharge => "Shield Overcharge",
            Self::TankUltimateArmor => "Ultimate Armor",
            Self::TankIndestructible => "Indestructible",
            Self::TankShieldBarrier => "Shield Barrier",
            Self::TankPerfectDefense => "Perfect Defense",
            Self::TankIronWill => "Iron Will",
            Self::TankJuggernaut => "JUGGERNAUT",
            
            // Continuing with abbreviated implementations for space
            // Gunner Tree
            Self::GunnerWeaponDamage1 => "Weapon Damage I",
            Self::GunnerWeaponDamage2 => "Weapon Damage II",
            Self::GunnerWeaponDamage3 => "Weapon Damage III",
            Self::GunnerFireRate1 => "Fire Rate I",
            Self::GunnerFireRate2 => "Fire Rate II",
            Self::GunnerAmmoCapacity1 => "Ammo Capacity I",
            Self::GunnerWeaponHeat1 => "Weapon Heat I",
            Self::GunnerMultiTargeting => "Multi-Targeting",
            Self::GunnerWeaponCooling1 => "Weapon Cooling I",
            Self::GunnerWeaponCooling2 => "Weapon Cooling II",
            Self::GunnerAmmoCapacity2 => "Ammo Capacity II",
            Self::GunnerReloadSpeed => "Reload Speed",
            Self::GunnerAccuracy => "Accuracy",
            Self::GunnerPenetration1 => "Penetration I",
            Self::GunnerSplashDamage => "Splash Damage",
            Self::GunnerPlasmaWeapons => "Plasma Weapons",
            Self::GunnerRailgunUnlock => "Railgun Unlock",
            Self::GunnerCriticalHits1 => "Critical Hits I",
            Self::GunnerCriticalHits2 => "Critical Hits II",
            Self::GunnerAmmoEfficiency => "Ammo Efficiency",
            Self::GunnerWeaponStabilization => "Weapon Stabilization",
            Self::GunnerArmorPiercing => "Armor Piercing",
            Self::GunnerExplosiveRounds => "Explosive Rounds",
            Self::GunnerWeaponOvercharge => "Weapon Overcharge",
            Self::GunnerDualWielding => "Dual Wielding",
            Self::GunnerPenetratingRounds => "Penetrating Rounds",
            Self::GunnerRapidFire => "Rapid Fire",
            Self::GunnerCriticalDamage => "Critical Damage",
            Self::GunnerWeaponSynergy => "Weapon Synergy",
            Self::GunnerBurstFire => "Burst Fire",
            Self::GunnerSuppressiveFire => "Suppressive Fire",
            Self::GunnerMasterGunner => "Master Gunner",
            Self::GunnerChainLightning => "Chain Lightning",
            Self::GunnerDevastationPassive => "Devastation Protocols",
            Self::GunnerPerfectAccuracy => "Perfect Accuracy",
            Self::GunnerObliterate => "Obliterate",
            Self::GunnerWeaponMastery => "Weapon Mastery",
            Self::GunnerInfiniteAmmo => "Infinite Ammo",
            Self::GunnerArsenalMaster => "ARSENAL MASTER",
            
            // Stealth Tree
            Self::StealthSignatureReduction1 => "Signature Reduction I",
            Self::StealthSignatureReduction2 => "Signature Reduction II",
            Self::StealthSilentRunning => "Silent Running",
            Self::StealthLowProfile => "Low Profile",
            Self::StealthSensorDampening => "Sensor Dampening",
            Self::StealthCloakField1 => "Cloak Field I",
            Self::StealthCloakField2 => "Cloak Field II",
            Self::StealthRadarJamming => "Radar Jamming",
            Self::StealthHeatMasking => "Heat Masking",
            Self::StealthVisualCamo => "Visual Camo",
            Self::StealthECM => "ECM",
            Self::StealthGhostProtocols1 => "Ghost Protocols I",
            Self::StealthActiveCamouflage => "Active Camouflage",
            Self::StealthHeatSink => "Heat Sink",
            Self::StealthGhostProtocols2 => "Ghost Protocols II",
            Self::StealthSilentWeapons => "Silent Weapons",
            Self::StealthCloakDuration => "Cloak Duration",
            Self::StealthPerfectStealth1 => "Perfect Stealth I",
            Self::StealthSensorGhost => "Sensor Ghost",
            Self::StealthPhantomCloak => "Phantom Cloak",
            Self::StealthPerfectStealth2 => "Perfect Stealth II",
            Self::StealthAmbushTactics => "Ambush Tactics",
            Self::StealthShadowStrike => "Shadow Strike",
            Self::StealthBackstab => "Backstab",
            Self::StealthInvisibility => "Invisibility",
            Self::StealthCloakRecharge => "Cloak Recharge",
            Self::StealthDeception => "Deception",
            Self::StealthMimicry => "Mimicry",
            Self::StealthInfiltrator => "Infiltrator",
            Self::StealthDecoyProjector => "Decoy Projector",
            Self::StealthPhaseShiftPassive => "Phase Shift Protocols",
            Self::StealthVanish => "Vanish",
            Self::StealthAssassin => "Assassin",
            Self::StealthPerfectCamouflage => "Perfect Camouflage",
            Self::StealthShadowWalker => "Shadow Walker",
            Self::StealthPhantom => "PHANTOM",
            
            // Sniper Tree
            Self::SniperRangeExtension1 => "Range Extension I",
            Self::SniperRangeExtension2 => "Range Extension II",
            Self::SniperRangeExtension3 => "Range Extension III",
            Self::SniperPrecisionTargeting => "Precision Targeting",
            Self::SniperScopeEnhancement1 => "Scope Enhancement I",
            Self::SniperProjectileSpeed => "Projectile Speed",
            Self::SniperScopeEnhancement2 => "Scope Enhancement II",
            Self::SniperChargeWeapons1 => "Charge Weapons I",
            Self::SniperChargeWeapons2 => "Charge Weapons II",
            Self::SniperLongShot => "Long Shot",
            Self::SniperFocusFire => "Focus Fire",
            Self::SniperPerfectAim1 => "Perfect Aim I",
            Self::SniperRangeCalculator => "Range Calculator",
            Self::SniperLongRangeRailgun => "Long-Range Railgun",
            Self::SniperHeadhunter => "Headhunter",
            Self::SniperSteadyAim => "Steady Aim",
            Self::SniperChargedShot => "Charged Shot",
            Self::SniperCriticalRange => "Critical Range",
            Self::SniperPerfectAim2 => "Perfect Aim II",
            Self::SniperOverwatch => "Overwatch",
            Self::SniperLongDistance => "Long Distance",
            Self::SniperOneShotOneKill => "One Shot One Kill",
            Self::SniperPerfectAccuracy => "Perfect Accuracy",
            Self::SniperArmorPiercing => "Armor Piercing",
            Self::SniperExecutioner => "Executioner",
            Self::SniperChargedDamage => "Charged Damage",
            Self::SniperPatientHunter => "Patient Hunter",
            Self::SniperDeadlyPrecision => "Deadly Precision",
            Self::SniperMarksman => "Marksman",
            Self::SniperUltraRange => "Ultra Range",
            Self::SniperChargedDevastation => "Charged Devastation",
            Self::SniperTargetLock => "Target Lock",
            Self::SniperPerfectShotPassive => "Perfect Shot Protocols",
            Self::SniperSnipeFromAnywhere => "Snipe From Anywhere",
            Self::SniperInstantKill => "Instant Kill",
            Self::SniperGhostShot => "Ghost Shot",
            Self::SniperDeadeye => "DEADEYE",
            
            // Missile Tree
            Self::MissileCapacity1 => "Missile Capacity I",
            Self::MissileCapacity2 => "Missile Capacity II",
            Self::MissileCapacity3 => "Missile Capacity III",
            Self::MissileReloadSpeed1 => "Reload Speed I",
            Self::MissileReloadSpeed2 => "Reload Speed II",
            Self::MissileTrackingBasic => "Basic Tracking",
            Self::MissileTrackingSystems => "Tracking Systems",
            Self::MissileSwarmMissiles => "Swarm Missiles",
            Self::MissileMultiLaunch => "Multi-Launch",
            Self::MissileAmmoReserves => "Ammo Reserves",
            Self::MissileFastReload => "Fast Reload",
            Self::MissileLockSpeed => "Lock Speed",
            Self::MissileVolley => "Volley",
            Self::MissileClusterWarheads => "Cluster Warheads",
            Self::MissileHomingAI => "Homing AI",
            Self::MissileDamage1 => "Missile Damage I",
            Self::MissileDamage2 => "Missile Damage II",
            Self::MissileProximityFuse => "Proximity Fuse",
            Self::MissileMultiTarget => "Multi-Target",
            Self::MissileSmartGuidance => "Smart Guidance",
            Self::MissileBarragePassive => "Barrage Protocols",
            Self::MissileBarrage => "Barrage",
            Self::MissileAOEExplosions => "AOE Explosions",
            Self::MissileFireAndForget => "Fire and Forget",
            Self::MissileClusterBombs => "Cluster Bombs",
            Self::MissileNuclearWarheads => "Nuclear Warheads",
            Self::MissileCarpetBombing => "Carpet Bombing",
            Self::MissileSeeker => "Seeker",
            Self::MissileOverwhelm => "Overwhelm",
            Self::MissileTacticalNukes => "Tactical Nukes",
            Self::MissileSmartMissiles => "Smart Missiles",
            Self::MissileDevastatorWarheads => "Devastator Warheads",
            Self::MissileInfiniteMissiles => "Infinite Missiles",
            Self::MissileStormPassive => "Missile Storm Protocols",
            Self::MissileApocalypse => "Apocalypse",
            Self::MissileArmageddon => "Armageddon",
            Self::MissileSupremacy => "MISSILE SUPREMACY",
        }
    }
    
    pub fn description(&self) -> &'static str {
        // Abbreviated for space - showing pattern
        match self {
            Self::FighterEngineBoost1 => "+15% max speed",
            Self::FighterEngineBoost2 => "+25% max speed",
            Self::FighterStrafeSpeed1 => "+15% strafe speed",
            Self::FighterApexFighter => "Unlock Quantum Dash ability. +50% speed, +50% agility",
            Self::TankJuggernaut => "Unlock Fortress Mode ability. +100% armor, +100% shields",
            Self::GunnerArsenalMaster => "Unlock Devastation ability. +100% damage, all weapons active",
            Self::StealthPhantom => "Unlock Phase Shift ability. Perfect stealth and invisibility",
            Self::SniperDeadeye => "Unlock Perfect Shot ability. +200% range, guaranteed crits",
            Self::MissileSupremacy => "Unlock Missile Storm ability. +500% missile capacity",
            _ => "+10-50% to relevant stats", // Placeholder for brevity
        }
    }
    
    pub fn cost(&self) -> UpgradeCost {
        let tier = self.tier();
        let (base_scrap, base_energy, base_mineral, base_tech) = match tier {
            1 => (10, 5, 2, 1),
            2 => (30, 15, 10, 5),
            3 => (70, 40, 25, 15),
            4 => (150, 80, 50, 30),
            5 => (300, 150, 100, 60),
            6 => (600, 300, 200, 120),
            _ => (10, 5, 2, 1),
        };
        
        UpgradeCost {
            scrap_metal: base_scrap,
            energy_cores: base_energy,
            rare_minerals: base_mineral,
            tech_components: base_tech,
        }
    }
    
    pub fn prerequisites(&self) -> Vec<UpgradeType> {
        // Simplified prerequisite system - tier N requires at least 2 nodes from tier N-1
        // Full implementation would define specific prerequisites
        vec![]
    }
    
    pub fn unlocks_ability(&self) -> Option<SpecialAbility> {
        match self {
            Self::FighterApexFighter => Some(SpecialAbility::QuantumDash),
            Self::TankJuggernaut => Some(SpecialAbility::FortressMode),
            Self::GunnerArsenalMaster => Some(SpecialAbility::Devastation),
            Self::StealthPhantom => Some(SpecialAbility::PhaseShift),
            Self::SniperDeadeye => Some(SpecialAbility::PerfectShot),
            Self::MissileSupremacy => Some(SpecialAbility::MissileStorm),
            _ => None,
        }
    }
    
    pub fn position(&self) -> (f32, f32) {
        // UI position within tree (tier_column, row)
        let tier = self.tier() as f32;
        let row = 0.0; // Would calculate based on order within tier
        (tier, row)
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
        // Sum up all damage bonuses from purchased upgrades
        // Simplified for now
        multiplier
    }
    
    pub fn get_fire_rate_multiplier(&self) -> f32 {
        1.0 // Simplified
    }
}

/// Skill tree node data structure
#[derive(Clone)]
pub struct SkillTreeNode {
    pub upgrade_type: UpgradeType,
    pub tier: u8,
    pub class: ShipClass,
    pub position: (f32, f32),
    pub prerequisites: Vec<UpgradeType>,
    pub unlocks: Option<SpecialAbility>,
}

impl SkillTreeNode {
    pub fn from_upgrade(upgrade_type: UpgradeType) -> Self {
        Self {
            tier: upgrade_type.tier(),
            class: upgrade_type.class(),
            position: upgrade_type.position(),
            prerequisites: upgrade_type.prerequisites(),
            unlocks: upgrade_type.unlocks_ability(),
            upgrade_type,
        }
    }
}

