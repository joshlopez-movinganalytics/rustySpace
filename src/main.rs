use bevy::prelude::*;
use bevy::core_pipeline::bloom::BloomSettings;

mod components;
mod resources;
mod systems;
mod utils;

use resources::GameState;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_event::<systems::progression::EnemyKillEvent>()
        .add_systems(Startup, (
            setup_game,
            systems::visuals::setup_starfield,
            systems::visuals::setup_planets,
        ))
        .add_systems(Update, (
            camera::camera_follow_system,
            camera::camera_free_look_system,
        ).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (
            movement::ship_movement_system,
            movement::mouse_flight_system,
            movement::apply_velocity_system,
            movement::apply_angular_velocity_system,
        ).run_if(in_state(GameState::InGame)))
        .init_resource::<systems::movement::MouseFlightSettings>()
        .init_resource::<systems::movement::MouseFlightState>()
        .add_systems(Update, (
            combat::weapon_state_system,
            combat::weapon_firing_system,
            combat::projectile_movement_system,
            combat::homing_projectile_system,
            combat::projectile_lifetime_system,
            combat::projectile_collision_system,
            combat::damage_system,
            combat::shield_recharge_system,
            combat::ship_death_system,
        ).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (
            ai::ai_controller_system,
            ai::ai_target_acquisition_system,
            ai::ai_weapon_selection_system,
            ai::ai_combat_system,
        ).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (
            spawning::enemy_spawner_system,
        ).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (
            resources_system::loot_collection_system,
            resources_system::spawn_loot_system,
            resources_system::animate_loot_system,
            resources_system::update_collection_particles,
        ).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (
            combat::energy_recharge_system,
        ).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (
            systems::effects::update_explosions,
            systems::effects::update_shield_effects,
            systems::effects::update_hull_spark_effects,
        ).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (
            systems::visuals::update_starfield,
            systems::visuals::update_planets,
            systems::visuals::update_ship_visuals_on_upgrade,
        ).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (
            progression::init_health_tracking_system,
            progression::track_damage_dealt_system,
            progression::track_enemy_kills_system,
            progression::track_resource_collection_system,
            progression::display_skill_point_gain_system,
        ).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (
            abilities::ability_activation_system,
            abilities::update_ability_cooldowns_system,
            abilities::apply_ability_effects_system,
            abilities::cleanup_ability_visuals_system,
            abilities::devastation_effect_system,
        ).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (
            ship_visuals::apply_class_visuals_system,
            ship_visuals::spawn_ship_attachments_system,
            ship_visuals::update_attachment_positions_system,
            ship_visuals::apply_ship_colors_system,
            ship_visuals::apply_ship_scale_system,
        ).run_if(in_state(GameState::InGame)))
        .add_systems(Update, (
            ui::update_hud_system,
            ui::update_weapon_hud_system,
            ui::update_targeting_reticule_system,
            ui::check_upgrade_availability_system,
            ui::update_upgrade_notification_pulse,
            ui::setup_enemy_health_bars,
            ui::update_enemy_health_bars,
            ui::apply_upgrades_to_player,
        ).run_if(in_state(GameState::InGame)))
        .add_systems(OnEnter(GameState::MainMenu), (
            ui::setup_main_menu,
            movement::release_cursor_lock,
            spawning::cleanup_on_main_menu,
        ))
        .add_systems(OnExit(GameState::MainMenu), ui::cleanup_main_menu)
        .add_systems(Update, ui::main_menu_system.run_if(in_state(GameState::MainMenu)))
        .add_systems(OnEnter(GameState::InGame), (
            movement::manage_cursor_lock,
            ui::setup_targeting_reticule,
        ))
        .add_systems(OnExit(GameState::InGame), ui::cleanup_targeting_reticule)
        .add_systems(OnEnter(GameState::Upgrade), (
            skill_tree_ui::setup_skill_tree_ui,
            movement::release_cursor_lock,
        ))
        .add_systems(OnExit(GameState::Upgrade), (
            skill_tree_ui::cleanup_skill_tree_ui,
            movement::manage_cursor_lock,
        ))
        .add_systems(Update, (
            skill_tree_ui::update_skill_node_states_system,
            skill_tree_ui::handle_class_tab_clicks_system,
            skill_tree_ui::handle_skill_node_clicks_system,
            ui::apply_upgrades_to_player, // Apply upgrades when viewing skill tree
            skill_tree_ui::update_stat_panel_system,
            skill_tree_ui::handle_skill_tree_close_system,
            skill_tree_ui::rebuild_skill_tree_on_tab_change_system,
            skill_tree_ui::handle_node_hover_preview_system,
            skill_tree_ui::skill_tree_scroll_system,
            skill_tree_ui::display_skill_node_tooltip_system,
            stat_visualization::update_radar_chart_system,
        ).run_if(in_state(GameState::Upgrade)))
        .add_systems(Update, ui::check_upgrade_key.run_if(in_state(GameState::InGame)))
        .add_systems(Update, ui::check_galaxy_map_key.run_if(in_state(GameState::InGame)))
        .add_systems(Update, ui::check_pause_key.run_if(in_state(GameState::InGame).or_else(in_state(GameState::Paused))))
        .add_systems(OnEnter(GameState::Paused), (
            ui::setup_pause_menu,
            movement::release_cursor_lock,
        ))
        .add_systems(OnExit(GameState::Paused), (
            ui::cleanup_pause_menu,
            movement::manage_cursor_lock,
        ))
        .add_systems(Update, ui::pause_menu_system.run_if(in_state(GameState::Paused)))
        .add_systems(OnEnter(GameState::GameOver), (
            ui::setup_game_over_menu,
            ui::cleanup_hud_on_game_over,
            movement::release_cursor_lock,
        ))
        .add_systems(OnExit(GameState::GameOver), ui::cleanup_game_over_menu)
        .add_systems(Update, ui::game_over_menu_system.run_if(in_state(GameState::GameOver)))
        .add_systems(OnEnter(GameState::InGame), (
            spawning::handle_restart_game,
            spawning::handle_load_game,
        ))
        .add_systems(OnEnter(GameState::GalaxyMap), (
            galaxy_ui::setup_galaxy_map_ui,
            movement::release_cursor_lock,
        ))
        .add_systems(OnExit(GameState::GalaxyMap), (
            galaxy_ui::cleanup_galaxy_map_ui,
            movement::manage_cursor_lock,
        ))
        .add_systems(Update, (
            galaxy_ui::galaxy_map_camera_controls,
            galaxy_ui::galaxy_map_close_system,
        ).run_if(in_state(GameState::GalaxyMap)))
        .add_systems(Update, (
            travel::check_jump_gate_proximity,
            travel::hyperspace_animation_system,
            travel::handle_system_transition,
            spawning::handle_respawn_system_content,
            galaxy::spawn_system_content,
            galaxy::update_planet_orbits,
            galaxy::animate_jump_gate_rings,
            galaxy::animate_jump_gate_glow,
        ).run_if(in_state(GameState::InGame)))
        .run();
}

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn directional light (sun)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(1.0, 0.95, 0.8),
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(50.0, 100.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.1, 0.1, 0.15),
        brightness: 150.0,
    });
    
    // Set darker background clear color
    commands.insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.05)));

    // Spawn camera with bloom enabled for glow effects
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                hdr: true, // Enable HDR for bloom to work properly
                ..default()
            },
            // Camera3dBundle already includes Tonemapping by default, no need to add it separately
            ..default()
        },
        BloomSettings {
            intensity: 0.5,
            low_frequency_boost: 0.02,
            low_frequency_boost_curvature: 0.2,
            high_pass_frequency: 0.7,
            prefilter_settings: bevy::core_pipeline::bloom::BloomPrefilterSettings {
                threshold: 1.0, // Only bloom objects with emissive > 2.0 (explosions)
                threshold_softness: 0.5,
            },
            composite_mode: bevy::core_pipeline::bloom::BloomCompositeMode::EnergyConserving,
        },
        components::camera::CameraController {
            follow_distance: 15.0,
            follow_height: 5.0,
            smoothness: 8.0, // Increased from 5.0 for smoother following
        },
    ));

    // Spawn player ship (parent entity with no mesh)
    let player_ship = commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        components::ship::Ship {
            max_speed: 80.0,
            acceleration: 40.0,
            turn_rate: 4.0,
            mass: 1000.0,
            boost_multiplier: 2.0,
        },
        components::ship::Velocity(Vec3::ZERO),
        components::ship::AngularVelocity(Vec3::ZERO),
        components::combat::Health {
            current: 100.0,
            max: 100.0,
        },
        components::combat::Shield {
            current: 100.0,
            max: 100.0,
            recharge_rate: 10.0,
            recharge_delay: 3.0,
            time_since_last_hit: 10.0,
        },
        components::ship::Player,
        components::combat::Faction::Player,
        components::combat::Energy {
            current: 100.0,
            max: 100.0,
            recharge_rate: 20.0,
        },
        components::abilities::AbilityController::new(),
        components::ship_classes::ShipVisualConfig::default(),
        components::ship_classes::ClassBonuses::new(),
    )).id();

    // Build modular ship visuals
    utils::ship_builder::build_ship(
        &mut commands,
        &mut meshes,
        &mut materials,
        components::ship::ShipType::Fighter,
        player_ship,
        Color::srgb(0.2, 0.5, 0.8),
    );

    // Add weapon mounts to player - starting with Laser, Autocannon, and Plasma
    commands.entity(player_ship).insert(components::combat::WeaponMount {
        weapons: vec![
            components::combat::Weapon::laser(),
            components::combat::Weapon::autocannon(),
            components::combat::Weapon::plasma(),
        ],
        current_weapon: 0,
    });

    // Initialize player inventory
    commands.insert_resource(components::resources::Inventory {
        scrap_metal: 100,  // Start with some resources for testing
        energy_cores: 50,
        rare_minerals: 25,
        tech_components: 10,
    });

    // Initialize player upgrades
    commands.insert_resource(components::upgrades::PlayerUpgrades::default());
    
    // Initialize class progression and skill points
    commands.insert_resource(components::ship_classes::ClassProgression::new());
    
    // Initialize progression tracker
    commands.insert_resource(systems::progression::ProgressionTracker::new());
    
    // Initialize active class tab (for skill tree UI)
    commands.insert_resource(systems::skill_tree_ui::ActiveClassTab::default());
    
    // Initialize hovered node tracker
    commands.insert_resource(systems::skill_tree_ui::HoveredNode::default());

    // Initialize game resources (reduced spawn time from 5s to 3s for more action)
    commands.insert_resource(resources::SpawnTimer(Timer::from_seconds(3.0, TimerMode::Repeating)));
    
    // Initialize galaxy
    let galaxy = resources::Galaxy::new(rand::random());
    commands.insert_resource(galaxy);
    
    // Trigger initial system content spawn
    commands.insert_resource(systems::galaxy::SpawnSystemContentFlag);
}

