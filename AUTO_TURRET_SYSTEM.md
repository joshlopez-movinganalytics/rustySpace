# Auto-Turret System

## Overview

Converted the autofire system into a **proper auto-turret** mounted on the back of your ship. The turret operates **completely independently** from your ship's flight controls, tracking and engaging enemies while you focus on flying.

## Key Features

### 🎯 **Independent Operation**
- **Separate rotation system** - turret rotates on its own, not tied to ship rotation
- **Own targeting logic** - finds and tracks nearest enemy independently
- **Own weapon** - turret has its own laser weapon with independent cooldown
- **Full 3D tracking** - can aim in ANY direction (360° in all axes)

### 📍 **Turret Placement**
- **Mounted on back** of ship (-3 units)
- **Elevated** (+2 units above ship)
- **Visible model** - glowing blue sphere with barrel
- **Rotates independently** - you can see it turning to track targets

### 🔫 **Combat Capabilities**
- **Range**: 120 units
- **Lock range**: 150 units  
- **Turn rate**: 6.0 rad/s (faster than ship)
- **Firing cone**: 5.7 degrees
- **Weapon**: Laser (default)
- **Predictive aiming**: Leads moving targets

## How It Works

### Controls
- **Press K** to toggle turret on/off
- **Ship controls unchanged** - fly normally
- **Turret tracks automatically** - no input needed

### Turret Behavior

**When Enabled:**
1. **Scans** for nearest enemy within 150 units
2. **Locks** onto target
3. **Rotates** independently to lead the target
4. **Fires** when target is within 120 units and in 5.7° cone
5. **Switches** to new target when current destroyed

**Visual Feedback:**
- HUD shows: `⟨⟨ AUTO-TURRET: LOCKED ⟩⟩` (cyan)
- HUD shows: `⟨⟨ AUTO-TURRET: SEARCHING ⟩⟩` (yellow)
- Turret physically rotates to face targets
- Console messages for tracking/firing

## Technical Implementation

### Components

**AutoTurret** (replaces AutofireController):
```rust
pub struct AutoTurret {
    pub enabled: bool,
    pub current_target: Option<Entity>,
    pub current_rotation: Quat,  // Independent rotation!
    pub max_lock_range: f32,     // 150.0
    pub max_fire_range: f32,     // 120.0
    pub turn_rate: f32,          // 6.0 rad/s
    pub fire_cone_angle: f32,    // 0.1 rad (~5.7°)
    pub weapon: Weapon,          // Own weapon
    pub firing_cooldown: f32,    // Own cooldown
}
```

**TurretVisual** - marker for visual entity

### Systems

1. **autofire_toggle_system** - K key toggle
2. **autofire_targeting_system** - Find nearest enemy
3. **autofire_aiming_system** - Rotate turret to intercept point
4. **update_turret_visual_system** - Sync visual with rotation
5. **autofire_firing_system** - Fire from turret position

### Key Differences from Old System

| Aspect | Old Autofire | New Turret |
|--------|-------------|-----------|
| **Rotation** | Ship rotation | Independent rotation |
| **Control** | Overrode flight controls | No flight interference |
| **Position** | Ship center | Back of ship |
| **3D Tracking** | Limited (pitch/yaw conflict) | Full 360° |
| **Visual** | None | Physical turret model |
| **Weapon** | Used ship weapons | Own dedicated weapon |

### Rotation Algorithm

```rust
// 1. Calculate turret world position
turret_pos = ship_pos + up*2.0 - forward*3.0

// 2. Calculate intercept point with lead
intercept = target_pos + target_vel * time_to_impact

// 3. Get turret's current direction
turret_forward = turret_rotation * Vec3::Z

// 4. Calculate rotation needed
to_intercept = (intercept - turret_pos).normalize()
rotation_axis = turret_forward.cross(to_intercept)
angle = acos(dot(turret_forward, to_intercept))

// 5. Apply rotation (capped by turn rate)
rotation_amount = min(angle, turn_rate * dt)
turret_rotation = Quat::from_axis_angle(axis, amount) * turret_rotation

// 6. Update visual
turret_visual.rotation = turret_rotation
```

### Firing Logic

```rust
// Turret fires when:
1. Enabled
2. Has target
3. Distance < 120 units
4. Angle < 5.7 degrees
5. Weapon ready (cooldown, energy, heat, ammo)

// Fires from turret position with turret rotation
projectile_pos = turret_pos + turret_forward * 1.5
projectile_vel = turret_forward * weapon_speed
```

## Advantages

### ✅ **No Flight Control Interference**
- Ship rotation controlled by player/mouse
- Turret rotation completely independent
- No more fighting between systems!

### ✅ **True 3D Tracking**
- Can target enemies **above** ship
- Can target enemies **below** ship
- Can target enemies **behind** ship
- Full sphere of coverage

### ✅ **Visual Clarity**
- You can **see** the turret tracking
- Physical model shows where it's aiming
- Glowing blue design fits cyberpunk theme

### ✅ **Better Gameplay**
- **You fly** - dodge, position, maneuver
- **Turret shoots** - provides covering fire
- **True multitasking** - offense + defense
- More engaging than autofire

## Debug Output

### Tracking:
```
[Turret Track] Target pos: (50.0, 10.0, 30.0) | Vel: (5.0, 0.0, -3.0) speed: 5.8 | Dist: 60.2 | Lead: 3.4 units
```

### Aiming:
```
[Turret Aim] Turret fwd: (0.85, 0.15, 0.50) | To-intercept: (0.87, 0.12, 0.48) | Angle: 2.1° | Status: LOCKED
```

### Firing:
```
[Turret Fire] Distance: 58.3/120.0 | Angle: 2.1°/5.7° | In cone: YES
[Turret Fire] 🎯 TURRET FIRING | Weapon speed: 150 | Turret pos: (2.0, 4.0, -5.0)
[Projectile Spawn] Type: Laser | Pos: (3.3, 4.2, -3.7) | Dir: (0.87, 0.12, 0.48) | Vel: (130.5, 18.0, 72.0) | Speed: 150
```

### Hits:
```
[HIT ✓] Laser HIT at distance 1.2 | Projectile pos: (48.9, 10.3, 29.8) | Target pos: (50.0, 10.0, 30.0) | Shield dmg=30.0, Hull dmg=1.2
```

## Configuration

Turret parameters in `AutoTurret::default()`:
```rust
max_lock_range: 150.0,     // Acquisition range
max_fire_range: 120.0,     // Firing range  
turn_rate: 6.0,            // Rotation speed (rad/s)
fire_cone_angle: 0.1,      // Firing tolerance (~5.7°)
weapon: Weapon::laser(),   // Turret weapon
```

## Visual Design

**Turret Base:**
- Sphere (0.8 radius)
- Blue metallic (0.3, 0.6, 0.9)
- Glowing emissive
- Position: (0, 2, -3) relative to ship

**Turret Barrel:**
- Capsule (0.15 radius, 1.2 length)
- Bright blue (0.4, 0.7, 1.0)  
- High metallic/low roughness
- Extended 1 unit forward from base

## Gameplay Tips

1. **Enable with K** before engaging
2. **Fly aggressively** - turret covers you
3. **Position strategically** - get turret good angles
4. **Use manual weapons too** - turret is supplemental
5. **Watch the turret** - see what it's tracking
6. **Disable to conserve energy** when not in combat

## Future Enhancements

Possible improvements:
- Multiple turrets (top, sides, bottom)
- Upgradeable turret weapons
- Turret armor/health
- Different turret types (missile, railgun, etc.)
- Manual turret control override
- Target priority settings
- Turret damage/destruction mechanics

