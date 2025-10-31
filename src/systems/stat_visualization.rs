use bevy::prelude::*;
use crate::components::{
    ship::Player,
    combat::{Health, Shield, Energy},
    ship::Ship,
    ship_classes::ClassBonuses,
};

/// Calculate stat values for radar chart
pub fn calculate_stat_distribution(
    health: &Health,
    shield: &Shield,
    energy: &Energy,
    ship: &Ship,
    bonuses: &ClassBonuses,
) -> StatDistribution {
    // Normalize stats to 0-100 range for radar chart
    let max_health_ref = 500.0; // Reference max health for scaling
    let max_speed_ref = 150.0;   // Reference max speed
    
    StatDistribution {
        speed: ((ship.max_speed / max_speed_ref) * 100.0).min(100.0),
        defense: (((health.max / max_health_ref) + (shield.max / max_health_ref) * 0.5 + bonuses.damage_reduction * 100.0) * 50.0).min(100.0),
        offense: ((bonuses.damage_multiplier + bonuses.fire_rate_multiplier - 2.0) * 50.0).min(100.0),
        stealth: ((bonuses.stealth_level + bonuses.evasion_chance) * 100.0).min(100.0),
        range: (bonuses.projectile_speed_multiplier * 30.0 + bonuses.critical_chance * 100.0).min(100.0),
        support: ((energy.max / 200.0 + bonuses.energy_recharge_multiplier * 20.0) * 50.0).min(100.0),
    }
}

/// Stat distribution for radar chart (0-100 range)
#[derive(Clone, Copy, Debug)]
pub struct StatDistribution {
    pub speed: f32,
    pub defense: f32,
    pub offense: f32,
    pub stealth: f32,
    pub range: f32,
    pub support: f32,
}

impl Default for StatDistribution {
    fn default() -> Self {
        Self {
            speed: 50.0,
            defense: 50.0,
            offense: 50.0,
            stealth: 50.0,
            range: 50.0,
            support: 50.0,
        }
    }
}

/// Preview stat distribution (for hover preview)
#[derive(Clone, Copy, Debug)]
pub struct PreviewStatDistribution {
    pub base: StatDistribution,
    pub preview: StatDistribution,
}

/// Marker for radar chart container
#[derive(Component)]
pub struct RadarChartContainer;

/// Marker for radar chart axes
#[derive(Component)]
pub struct RadarChartAxis;

/// Marker for radar chart fill
#[derive(Component)]
pub struct RadarChartFill;

/// Marker for radar chart preview (hover overlay)
#[derive(Component)]
pub struct RadarChartPreview;

/// Create visual radar chart
pub fn spawn_radar_chart(
    parent: &mut ChildBuilder,
    stats: StatDistribution,
) {
    let chart_size = 200.0;
    let center = chart_size / 2.0;
    
    parent.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(chart_size),
                height: Val::Px(chart_size),
                position_type: PositionType::Relative,
                ..default()
            },
            ..default()
        },
        RadarChartContainer,
    )).with_children(|chart| {
        // Draw axes (6 lines from center)
        for i in 0..6 {
            let angle = (i as f32 * 60.0 - 90.0).to_radians();
            let x = center + (center * 0.95 * angle.cos());
            let y = center + (center * 0.95 * angle.sin());
            
            chart.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(center),
                        top: Val::Px(center),
                        width: Val::Px(center * 0.95),
                        height: Val::Px(2.0),
                        ..default()
                    },
                    background_color: Color::srgb(0.3, 0.3, 0.4).into(),
                    transform: Transform::from_rotation(Quat::from_rotation_z(angle)),
                    ..default()
                },
                RadarChartAxis,
            ));
        }
        
        // Draw stat labels at ends of axes
        let labels = ["Speed", "Defense", "Offense", "Stealth", "Range", "Support"];
        for (i, label) in labels.iter().enumerate() {
            let angle = (i as f32 * 60.0 - 90.0).to_radians();
            let label_dist = center * 1.05;
            let x = center + (label_dist * angle.cos());
            let y = center + (label_dist * angle.sin());
            
            chart.spawn((
                TextBundle::from_section(
                    *label,
                    TextStyle {
                        font_size: 11.0,
                        color: Color::srgb(0.7, 0.7, 0.8),
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(x),
                    top: Val::Px(y),
                    ..default()
                }),
            ));
        }
        
        // Draw concentric circles for reference
        for radius_pct in [0.25, 0.5, 0.75, 1.0] {
            let radius = center * radius_pct;
            // Draw as a border on a node (simplified)
            chart.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(center - radius),
                    top: Val::Px(center - radius),
                    width: Val::Px(radius * 2.0),
                    height: Val::Px(radius * 2.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                border_color: Color::srgba(0.2, 0.2, 0.3, 0.5).into(),
                background_color: Color::NONE.into(),
                border_radius: BorderRadius::all(Val::Px(radius)),
                ..default()
            });
        }
        
        // Draw filled polygon for stats
        spawn_radar_chart_fill(chart, stats, center);
    });
}

/// Spawn radar chart filled polygon
fn spawn_radar_chart_fill(
    parent: &mut ChildBuilder,
    stats: StatDistribution,
    center: f32,
) {
    // Create hexagon points from stats
    let points = [
        (0, stats.speed),
        (60, stats.defense),
        (120, stats.offense),
        (180, stats.stealth),
        (240, stats.range),
        (300, stats.support),
    ];
    
    // Draw filled area as multiple triangles (simplified as a visual representation)
    // In Bevy UI, we'll use a simplified approach with a background node
    // For a proper radar chart, we'd need custom rendering, but we can use colored sections
    
    let avg_stat = (stats.speed + stats.defense + stats.offense + stats.stealth + stats.range + stats.support) / 6.0;
    let fill_radius = center * (avg_stat / 100.0);
    
    parent.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(center - fill_radius),
                top: Val::Px(center - fill_radius),
                width: Val::Px(fill_radius * 2.0),
                height: Val::Px(fill_radius * 2.0),
                ..default()
            },
            background_color: Color::srgba(0.2, 0.6, 1.0, 0.4).into(),
            border_radius: BorderRadius::all(Val::Px(fill_radius)),
            ..default()
        },
        RadarChartFill,
    ));
}

/// Update radar chart with current stats
pub fn update_radar_chart_system(
    player_query: Query<(
        &Health,
        &Shield,
        &Energy,
        &Ship,
        &ClassBonuses,
    ), With<Player>>,
    mut chart_query: Query<&mut Style, (With<RadarChartFill>, Without<RadarChartPreview>)>,
) {
    if chart_query.is_empty() {
        return; // Chart not spawned yet
    }
    
    if let Ok((health, shield, energy, ship, bonuses)) = player_query.get_single() {
        let stats = calculate_stat_distribution(health, shield, energy, ship, bonuses);
        
        // Update fill size based on average stat
        let avg_stat = (stats.speed + stats.defense + stats.offense + stats.stealth + stats.range + stats.support) / 6.0;
        let center = 100.0;
        let fill_radius = center * (avg_stat / 100.0).clamp(0.1, 1.0);
        
        for mut style in chart_query.iter_mut() {
            style.width = Val::Px(fill_radius * 2.0);
            style.height = Val::Px(fill_radius * 2.0);
            style.left = Val::Px(center - fill_radius);
            style.top = Val::Px(center - fill_radius);
        }
    }
}

/// Calculate preview stats (what stats would be if hovered upgrade was purchased)
pub fn calculate_preview_stats(
    current_stats: &StatDistribution,
    hovered_upgrade: &crate::components::upgrades::UpgradeType,
) -> StatDistribution {
    let mut preview = *current_stats;
    
    // Apply preview bonuses based on upgrade type
    // Simplified preview calculation
    use crate::components::upgrades::UpgradeType::*;
    match hovered_upgrade {
        // Fighter upgrades
        FighterEngineBoost1 | FighterEngineBoost2 | FighterAfterburner => {
            preview.speed += 15.0;
        }
        FighterDriftManeuvers | FighterRollRate1 => {
            preview.speed += 10.0;
        }
        
        // Tank upgrades
        TankHullPlating1 | TankHullPlating2 | TankHullPlating3 => {
            preview.defense += 10.0;
        }
        TankShieldCapacity1 | TankShieldCapacity2 | TankShieldCapacity3 => {
            preview.defense += 15.0;
        }
        TankDamageReduction1 | TankDamageReduction2 => {
            preview.defense += 8.0;
        }
        
        // Gunner upgrades
        GunnerWeaponDamage1 | GunnerWeaponDamage2 | GunnerWeaponDamage3 => {
            preview.offense += 12.0;
        }
        GunnerFireRate1 | GunnerFireRate2 => {
            preview.offense += 10.0;
        }
        
        // Stealth upgrades
        StealthSignatureReduction1 | StealthSignatureReduction2 | StealthCloakField1 => {
            preview.stealth += 15.0;
        }
        
        // Sniper upgrades
        SniperRangeExtension1 | SniperRangeExtension2 | SniperRangeExtension3 => {
            preview.range += 20.0;
        }
        SniperPerfectAim1 | SniperPerfectAim2 => {
            preview.range += 15.0;
        }
        
        // Missile upgrades
        MissileCapacity1 | MissileCapacity2 | MissileCapacity3 => {
            preview.support += 12.0;
        }
        MissileDamage1 | MissileDamage2 => {
            preview.offense += 10.0;
        }
        
        _ => {
            // Default preview boost
            match hovered_upgrade.class() {
                crate::components::ship_classes::ShipClass::Fighter => preview.speed += 5.0,
                crate::components::ship_classes::ShipClass::Tank => preview.defense += 5.0,
                crate::components::ship_classes::ShipClass::Gunner => preview.offense += 5.0,
                crate::components::ship_classes::ShipClass::Stealth => preview.stealth += 5.0,
                crate::components::ship_classes::ShipClass::Sniper => preview.range += 5.0,
                crate::components::ship_classes::ShipClass::MissileTanker => preview.support += 5.0,
            }
        }
    }
    
    // Clamp to 0-100
    preview.speed = preview.speed.clamp(0.0, 100.0);
    preview.defense = preview.defense.clamp(0.0, 100.0);
    preview.offense = preview.offense.clamp(0.0, 100.0);
    preview.stealth = preview.stealth.clamp(0.0, 100.0);
    preview.range = preview.range.clamp(0.0, 100.0);
    preview.support = preview.support.clamp(0.0, 100.0);
    
    preview
}

