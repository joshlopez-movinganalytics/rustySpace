# Quick Start Guide

## Install and Run (5 minutes)

### 1. Prerequisites
- Rust installed: https://rustup.rs/
- Vulkan drivers (usually pre-installed on modern systems)

### 2. Build
```bash
cd rust-test
cargo build --release
```

### 3. Run
```bash
cargo run --release
```

## Your First Game (2 minutes)

### 1. Main Menu
- Press `ENTER` to start

### 2. Basic Combat
- Use `W/A/S/D` to move
- Use `Arrow Keys` to rotate
- Click `Left Mouse` to shoot
- Destroy the red enemy ships

### 3. Collect Resources
- Fly near the colored orbs that appear
- Watch the console for resource notifications

### 4. Upgrade Your Ship
- Press `U` to open upgrades
- Scroll through available upgrades
- Press `ESC` to return to combat

## Controls Cheat Sheet

```
MOVEMENT                COMBAT              UI
├─ W/S: Forward/Back   ├─ Mouse: Fire      ├─ U: Upgrades
├─ A/D: Strafe         ├─ 1/2/3: Weapons   └─ ESC: Close menu
├─ Space: Up           └─ Shift: Boost
├─ Ctrl: Down
└─ Arrows: Rotate
   Q/E: Roll
```

## Tips for Success

### Combat Tips
1. **Keep Moving**: Stationary targets are easy to hit
2. **Use Boost**: Press Shift to escape or chase
3. **Watch Your Energy**: Wait for recharge if needed
4. **Shields First**: Let shields recharge before taking more damage
5. **Circle Strafe**: Use A/D while shooting to dodge

### Upgrade Strategy
1. **Start with Shields**: Shield Capacity I is affordable early
2. **Then Weapons**: Unlock Plasma Cannon for more damage
3. **Speed Next**: Engine upgrades help you survive
4. **Balance**: Don't neglect any one area
5. **Save for Railgun**: Most powerful weapon in game

### Enemy Types
- **Red (Fighter)**: Fast but weak - easy pickings
- **Orange (Corvette)**: Balanced threat
- **Dark Red (Frigate)**: Tanky, dangerous
- **Deep Red (Capital)**: Boss-level enemy

### Resource Priority
```
Early Game:  Scrap Metal + Energy Cores
Mid Game:    Add Rare Minerals
Late Game:   Save Tech Components for weapons
```

## Common Issues

### Game Won't Start
- Check Vulkan drivers are installed
- Try debug mode: `cargo run`

### Low FPS
- Use release mode: `cargo run --release`
- Close other applications

### Can't Hit Enemies
- Enemies are fast - lead your shots
- Get closer before firing
- Try slower weapons (Plasma/Missile)

### Running Out of Energy
- Wait for recharge (automatic)
- Fire in bursts, not continuously
- Upgrade Power Recharge

## Advanced Techniques

### 6DOF Combat
```
Use all 3 dimensions:
- Thrust up/down to dodge
- Roll with Q/E for evasive moves
- Combine strafe + pitch for spirals
```

### Energy Management
```
Energy per weapon shot:
Laser: 5   (spam friendly)
Plasma: 15 (burst fire)
Missile: 25 (careful shots)
Railgun: 40 (precision only)
```

### AI Exploitation
```
- Enemies attack from range
- Get close to disrupt them
- They retreat when damaged
- Focus fire one at a time
```

## Progression Path

### Level 1 (0-100 resources)
- Survive basic fighters
- Learn controls
- Collect initial resources

### Level 2 (100-500 resources)
- Buy first upgrades
- Unlock Plasma Cannon
- Fight corvettes

### Level 3 (500-1000 resources)
- Multiple upgrades
- Unlock Missiles
- Handle frigates

### Level 4 (1000+ resources)
- Advanced upgrades
- Unlock Railgun
- Challenge capital ships

## Fun Challenges

1. **Pacifist**: Collect 100 resources (hard!)
2. **Rambo**: Destroy 10 enemies without upgrading
3. **Speedrun**: Get Railgun in 10 minutes
4. **Untouchable**: Fight 5 enemies without damage
5. **Ace Pilot**: Destroy a Capital Ship

## Next Steps

After mastering the basics:
1. Read FEATURES.md for detailed mechanics
2. Check IMPLEMENTATION_SUMMARY.md for technical details
3. Read the source code to understand systems
4. Contribute improvements or mods

## Getting Help

### Check the Logs
- Console shows all game events
- Look for [System Name] prefixes
- Useful for debugging issues

### Common Questions

**Q: How do I pause?**  
A: Game doesn't pause yet - upgrade menu (U) effectively pauses

**Q: Can I save?**  
A: Not yet - play sessions are temporary

**Q: Multiplayer?**  
A: Not implemented - single player only

**Q: How do I win?**  
A: No win condition yet - goal is to upgrade fully

**Q: Can I mod it?**  
A: Yes! It's open source Rust code

## Performance Tuning

### Low-End Systems
```toml
# In Cargo.toml, try:
[profile.release]
opt-level = 2  # Instead of 3
lto = false    # Faster compile
```

### High-End Systems
```bash
# Already optimized for performance
# Should run at 60+ FPS
```

## Have Fun!

This is a complete, playable game. Experiment, explore, and enjoy the space combat!

Report bugs or suggestions to the developer.

---

**Remember**: W/A/S/D to move, Mouse to shoot, U for upgrades!

