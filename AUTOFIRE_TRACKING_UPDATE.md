# Autofire Tracking Enhancement

## What Changed

The autofire system has been upgraded from **subtle aim assist** to **full active tracking**.

### Before
- Provided gentle nudges toward target
- Required manual aiming
- Strength: 30%
- Worked alongside manual control

### After  
- **Actively turns ship to face target**
- **Predictive aiming with target lead**
- Strength: 80% of max turn rate
- Takes over aiming when enabled

## Key Improvements

### 1. Predictive Targeting
The system now calculates where the target **will be** when projectiles arrive:

```rust
// Estimate time to impact
let time_to_impact = distance / projectile_speed;

// Predict target position
let predicted_position = target_transform.translation + target_velocity.0 * time_to_impact;
```

This means shots lead moving targets automatically, greatly improving hit rate.

### 2. Proportional Control
Tracking uses intelligent speed adjustment:

```rust
// Turn faster when far from target
let turn_speed_multiplier = (angle_to_target * 3.0).min(1.0);

// Apply proportional rotation
let turn_amount = turn_direction * base_turn_rate * turn_speed_multiplier;
```

- **Far from target**: Fast rotation to catch up
- **Near alignment**: Slower, precise adjustment
- **On target**: Minimal movement to maintain lock

### 3. Direct Control Override
When autofire is active, it takes control of ship rotation:

```rust
// Set angular velocity directly (overrides manual yaw)
angular_vel.0.y = turn_amount;
```

Player can still:
- Control throttle (forward/back)
- Strafe (if implemented)
- Boost
- Switch weapons
- Toggle autofire on/off

## Technical Details

### Modified Functions

**`autofire_aiming_system`**
- Changed from additive aim assist to direct rotation control
- Added velocity tracking for enemies
- Implemented predictive aiming algorithm
- Added proportional control for smooth tracking
- Increased strength from 0.3 to 0.8

### Performance Characteristics

| Metric | Value |
|--------|-------|
| Tracking Strength | 80% of ship turn rate |
| Alignment Tolerance | 0.01 radians (~0.57°) |
| Update Rate | Every frame |
| Prediction Method | Linear extrapolation |
| Default Turn Rate | 4.0 rad/s |
| Effective Tracking | ~3.2 rad/s |

## Gameplay Impact

### Combat Flow
1. Press **K** to enable autofire
2. Ship automatically rotates toward nearest enemy
3. Leads moving targets for better accuracy
4. Fires when target enters firing cone
5. Switches targets when current is destroyed

### Player Focus
With autofire enabled, player can focus on:
- **Positioning** - Get into advantageous positions
- **Dodging** - Evade incoming fire
- **Throttle Control** - Manage distance
- **Weapon Selection** - Choose right tool for the job
- **Ability Usage** - Time special abilities
- **Target Management** - Toggle to switch focus

### Tactical Considerations

**Best Used When:**
- Fighting multiple weak enemies
- Need to focus on dodging
- Want consistent damage output
- Target is highly mobile

**Disable When:**
- Need precise shot placement
- Want to conserve ammo on specific target
- Prefer full manual control
- Target is behind cover

## Code Changes

### Files Modified
1. `src/systems/combat.rs` - Updated `autofire_aiming_system`
2. `src/components/combat.rs` - Changed default `aim_assist_strength` to 0.8
3. `AUTOFIRE_FEATURE.md` - Updated documentation

### Lines Changed
- ~35 lines modified in combat.rs
- ~1 line modified in combat component
- Documentation updates

## Testing Recommendations

1. **Accuracy Test**
   - Enable autofire against moving target
   - Verify shots lead the target properly
   - Check hit percentage improvement

2. **Control Test**
   - Toggle autofire on/off
   - Verify smooth transitions
   - Test manual override still works for throttle

3. **Multi-Target Test**
   - Fight 3+ enemies
   - Check target switching behavior
   - Verify tracking maintains lock

4. **Edge Cases**
   - Very close targets (< 10 units)
   - Very fast targets
   - Targets moving perpendicular
   - Targets changing direction rapidly

## Future Enhancements

Potential improvements for later:

1. **Weapon-Specific Projectile Speeds**
   - Use actual weapon projectile_speed for more accurate leading
   - Current: uses average 100 units/s
   - Better: use weapon.projectile_speed from current weapon

2. **Smart Target Selection**
   - Priority targeting (weakest, closest, dangerous)
   - Sticky targeting option
   - Manual target lock/unlock

3. **Visual Feedback**
   - Lead indicator showing predicted impact point
   - Target lock UI element
   - Tracking cone visualization

4. **Configurable Aggressiveness**
   - Settings for tracking strength
   - Different modes (passive assist vs active tracking)
   - Per-weapon tuning

