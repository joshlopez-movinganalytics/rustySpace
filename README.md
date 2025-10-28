# Space Combat Game

A 3D space combat game built with Rust and Bevy engine using Vulkan rendering.

## Features

### Current Implementation (Phase 1-4)

- **6DOF Flight Controls**: Full freedom of movement with Newtonian physics
- **Combat System**: Multiple weapon types (Laser, Plasma, Missile, Railgun)
- **Enemy AI**: Behavior state machine with Patrol, Pursue, Attack, Evade, and Retreat states
- **Enemy Types**: Fighter, Corvette, Frigate, and Capital Ship with unique characteristics
- **Resource Collection**: Collect loot from destroyed ships
- **Ship Upgrades**: Comprehensive upgrade system for hull, shields, engines, power, and weapons
- **Physics-Based Projectiles**: Realistic projectile movement with velocity inheritance
- **Shield System**: Recharging shields with damage delay mechanics

### Controls

**Movement:**
- W/S - Forward/Backward thrust
- A/D - Strafe left/right
- Space/Left Ctrl - Vertical thrust up/down
- Left Shift - Boost

**Rotation:**
- Arrow Keys (Up/Down) - Pitch
- Arrow Keys (Left/Right) - Yaw
- Q/E - Roll

**Combat:**
- Left Mouse Button - Fire weapon
- 1/2/3 - Switch weapons

**Camera:**
- C + I/J/K/L - Free camera mode (debug)

## Building and Running

### Prerequisites

- Rust toolchain (latest stable)
- Vulkan drivers installed

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

## Project Structure

```
src/
├── components/          # ECS components
│   ├── ship.rs         # Ship characteristics and movement
│   ├── combat.rs       # Weapons, health, shields, energy
│   ├── ai.rs           # AI controller and enemy types
│   ├── resources.rs    # Loot and inventory
│   └── camera.rs       # Camera controller
├── systems/             # ECS systems
│   ├── movement.rs     # Ship movement and physics
│   ├── combat.rs       # Weapon firing and damage
│   ├── ai.rs           # AI behavior
│   ├── spawning.rs     # Enemy spawning
│   ├── resources_system.rs  # Loot collection
│   ├── camera.rs       # Camera follow
│   └── ui.rs           # User interface
├── resources/           # Game resources
│   └── game_state.rs   # Game state management
└── utils/              # Utility functions
    └── math.rs         # Math helpers

assets/
├── models/             # 3D models (GLTF)
├── textures/           # Texture files
├── audio/              # Sound effects and music
└── shaders/            # Custom shaders
```

## Gameplay

### Objective

Explore the galaxy, fight enemy ships, and collect resources to upgrade your ship. Each enemy type drops different amounts and types of resources:

- **Scrap Metal**: Basic hull upgrades
- **Energy Cores**: Power plant and energy systems
- **Rare Minerals**: Advanced shield upgrades
- **Tech Components**: Weapon and special system upgrades

### Enemy Types

1. **Fighter**: Fast, lightly armored, weak weapons
   - Health: 50 | Shields: 30
   - Aggressive tactics, high evasion

2. **Corvette**: Balanced stats, moderate threat
   - Health: 100 | Shields: 80
   - Balanced combat approach

3. **Frigate**: Slow, heavy shields, multiple weapons
   - Health: 200 | Shields: 150
   - Defensive, sustained fire

4. **Capital Ship**: Very slow, massive shields, all weapon types
   - Health: 500 | Shields: 400
   - Aggressive, overwhelming firepower

### Weapon Types

1. **Laser**: Fast fire rate, low damage, energy efficient
2. **Plasma**: Medium fire rate, moderate damage, spread fire
3. **Missile**: Slow fire rate, high damage, tracking capability
4. **Railgun**: Very slow, extremely high damage, long range

## Technical Details

### Vulkan Backend

The game uses Bevy's Vulkan backend for high-performance rendering. The backend is explicitly configured in the main application setup.

### Physics

- Newtonian physics model with inertia
- Projectiles inherit ship velocity
- Speed limiting and drag simulation
- 6 degrees of freedom movement

### AI System

The AI uses a behavior state machine with the following states:
- **Patrol**: Random movement when no target
- **Pursue**: Approach detected target
- **Attack**: Engage target with weapons
- **Evade**: Perform evasive maneuvers
- **Retreat**: Escape when heavily damaged

## Roadmap

### Phase 5: Resources & Upgrades (In Progress)
- [ ] Upgrade UI implementation
- [ ] Ship stat visualization
- [ ] Upgrade tree with prerequisites

### Phase 6: Procedural Galaxy
- [ ] Seed-based star system generation
- [ ] Jump gate travel system
- [ ] Galaxy map interface
- [ ] Multiple star systems

### Phase 7: Visual Polish
- [ ] GLTF model loading
- [ ] Particle effects (explosions, trails)
- [ ] Shield visual effects
- [ ] Enhanced lighting

### Phase 8: UI/UX
- [ ] HUD with health/shield bars
- [ ] Radar/minimap
- [ ] Target information display
- [ ] Damage indicators

### Phase 9: Audio
- [ ] Sound effects
- [ ] Dynamic music system

### Phase 10: Optimization
- [ ] Entity pooling
- [ ] LOD system
- [ ] Performance profiling

## Development

### Running in Debug Mode

```bash
cargo run
```

Debug mode includes:
- Free camera controls
- Console logging for game events

### Testing

```bash
cargo test
```

## License

[Add your license here]

## Credits

Built with:
- [Bevy Engine](https://bevyengine.org/)
- [Rapier3D](https://rapier.rs/) - Physics engine
- Rust programming language

