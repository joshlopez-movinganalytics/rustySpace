# Autofire/Aiming Mechanic

## Overview
An autofire system that automatically tracks and engages the nearest enemy when enabled. The system provides aim assist and automatic firing when enemies are within range and in your sights.

## Controls
- **Press K** to toggle autofire on/off

## Features

### 1. Auto-Targeting
- Automatically finds and locks onto the nearest enemy within 150 units
- Maintains lock on current target as long as they're in range
- Switches to new target if current one is destroyed or moves out of range

### 2. Active Tracking & Aiming
- **Automatically turns ship to face locked target**
- Uses predictive aiming (leads moving targets)
- Strength: 80% of ship's max turn rate (configurable via `aim_assist_strength`)
- Proportional control - turns faster when far from target, slows as it aligns
- Calculates target lead based on:
  - Target velocity
  - Distance to target
  - Projectile speed
- Only active when autofire is enabled and target is locked

### 3. Auto-Firing
- Fires current weapon when:
  - Target is within 120 units (configurable via `max_fire_range`)
  - Target is within ~8.6 degree cone (configurable via `fire_cone_angle`)
  - All normal firing conditions are met (energy, ammo, heat, cooldown)
- Automatically handles reloading when needed
- Respects weapon cooldowns and fire rates

### 4. Visual Feedback
- HUD indicator shows autofire status:
  - **⟨⟨ AUTOFIRE: LOCKED ⟩⟩** (cyan) - Target acquired and locked
  - **⟨⟨ AUTOFIRE: SEARCHING ⟩⟩** (yellow) - No target in range
  - Empty when disabled
- Console messages when toggling on/off

## Configuration

You can adjust these parameters in `AutofireController::default()`:

```rust
max_lock_range: 150.0,      // Maximum range to acquire targets
max_fire_range: 120.0,      // Maximum range to fire at targets
aim_assist_strength: 0.8,   // Tracking strength (0.0-1.0, as % of max turn rate)
fire_cone_angle: 0.15,      // Cone angle in radians (~8.6 degrees)
```

## Technical Implementation

### Components Added
- `AutofireController` - Component on player ship tracking autofire state

### Systems Added
1. `autofire_toggle_system` - Handles K key press to toggle on/off
2. `autofire_targeting_system` - Finds and tracks nearest enemy
3. `autofire_aiming_system` - Applies aim assist rotation
4. `autofire_firing_system` - Automatically fires when conditions are met

### UI Updates
- Added `AutofireStatusText` marker component
- Added status indicator to HUD in weapon section
- Integrated into `update_weapon_hud_system`

## Usage Tips
1. Enable autofire (K) when overwhelmed by enemies - it will handle aiming
2. Ship will automatically track and lead targets
3. You can still control throttle and movement while autofire handles aiming
4. Works with all weapon types and respects their characteristics
5. Disable when you need precise manual control or want to disengage
6. Great for fighting multiple enemies - just fly and it handles the shooting
7. Predictive aiming works best with faster projectile weapons (Laser, Railgun)

## Future Enhancements (Optional)
- Multiple target priority modes (nearest, weakest, strongest)
- Visual lead indicator for moving targets
- Adjustable settings via skill tree
- Target switching with Tab key
- Priority targets (focus fire on specific enemy types)

