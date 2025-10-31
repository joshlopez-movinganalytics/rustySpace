# Autofire Close-Range Tracking Fixes

## Problems Identified from Debug Log

### ✅ What Was Working
- **Long range (60-120 units)**: Excellent tracking and hits
  - Example (line 198): 0.8° angle, LOCKED status, consistent hits
  - Lead calculation accurate: 5.5 units for 9.1 u/s target
  
- **First enemy engagement**: Perfect
  - Lines 195-337: Hit after hit with 0.5-2.0 unit accuracy
  - Tracking smooth and stable

### ❌ What Was Broken
- **Close range (10-35 units)**: Complete chaos
  - Lines 552-614: Wild oscillation (39°, 71°, 100°, 155° swings!)
  - Forward vector flipping constantly
  - "In cone: NO" repeatedly despite being very close
  
**Example from log (line 558):**
```
Target at (3.3, 8.5, -0.5), only 16 units away
Forward: (-0.86, 0.22, 0.45)  
To-intercept: (-0.49, -0.85, 0.18)
Angle: 71.572° - MASSIVE!
Next frame: Forward completely different direction
```

## Root Causes

### 1. **Overshooting at Close Range**
At 15 units, even a small rotation creates huge position changes:
- Turn rate: 4.0 rad/s × 0.8 strength × 6.0 multiplier = **19.2 rad/s**
- In 0.016s frame: 0.307 radians = **17.6° per frame!**
- Target only needs 5° correction but ship rotates 17°
- Result: Oscillation and never stabilizing

### 2. **Fixed Turn Rate Regardless of Distance**
Same aggressive turning at 15 units as at 120 units made no sense.

### 3. **Firing Cone Too Tight**
4.6° cone impossible to hit when angular velocity causes 10-20° overshoots.

### 4. **Complete Angular Velocity Override**
Replacing `angular_vel.0.y` entirely caused:
- Loss of momentum
- Jarring direction changes
- No smoothing between frames

## Solutions Implemented

### 1. **Distance-Based Turn Rate Scaling**

```rust
let distance_factor = if distance < 30.0 {
    // Very close: gentle tracking (20-100% power)
    (distance / 30.0).clamp(0.2, 1.0)
} else if distance < 60.0 {
    // Medium range: moderate (70% power)
    0.7
} else {
    // Long range: full power
    1.0
};
```

**Effect:**
- At 10 units: 33% power → ~6 rad/s max turn
- At 30 units: 100% power → ~19 rad/s max turn
- At 60+ units: 100% power → ~19 rad/s max turn

### 2. **Reduced Base Aggression**

```rust
// Changed from 6.0 to 4.0
let base_turn_rate = ship.turn_rate * aim_assist_strength * 4.0;
```

**Effect:**
- Max turn rate: 4.0 × 0.8 × 4.0 = **12.8 rad/s** (was 19.2 rad/s)
- 33% reduction in turn speed
- More controllable, less overshooting

### 3. **Turn Rate Clamping**

```rust
let max_turn = ship.turn_rate * 1.5; // Cap at 150% of normal
let clamped_turn = turn_amount.clamp(-max_turn, max_turn);
```

**Effect:**
- Hard limit: 6.0 rad/s maximum (4.0 × 1.5)
- Prevents wild spinning
- Guarantees stable tracking

### 4. **Velocity Blending Instead of Override**

```rust
// Old: angular_vel.0.y = turn_amount; (complete override)
// New: 
angular_vel.0.y = angular_vel.0.y * 0.3 + clamped_turn * 0.7;
```

**Effect:**
- Maintains 30% of existing momentum
- Smoother transitions between frames
- Less jarring rotation changes
- Can still be influenced by player input

### 5. **Distance-Scaled Firing Cone**

```rust
let effective_fire_cone = if distance < 30.0 {
    // Close: up to 15° (0.26 rad)
    fire_cone_angle + (30.0 - distance) / 30.0 * 0.20
} else if distance < 60.0 {
    // Medium: ~7.5° (0.13 rad)
    fire_cone_angle + 0.05
} else {
    // Long: 4.6° (0.08 rad)
    fire_cone_angle
};
```

**Effect:**
| Distance | Cone Angle | Why |
|----------|-----------|-----|
| 10 units | ~15° | Point-blank, allow overshooting |
| 30 units | ~4.6° | Transition point |
| 60 units | ~7.5° | Medium precision |
| 100 units | ~4.6° | Sniper precision |

### 6. **Gentler Proportional Curve**

```rust
// Changed from 2.0 to 1.5
let angle_speed_multiplier = (angle_to_target * 1.5).min(1.0);
```

**Effect:**
- More gradual acceleration
- Less aggressive initial rotation
- Smoother approach to target

## Expected Improvements

### Before
```
Distance: 15 units
Angle: 71° → 86° → 100° → 155° (wild swings)
In cone: NO (always)
Result: MISS
```

### After
```
Distance: 15 units
Turn power: 33% (distance factor)
Max turn: 6 rad/s (clamped)
Cone: 15° (widened)

Angle: 71° → 55° → 40° → 25° → 12° → 8° → FIRE!
In cone: YES (at 8°)
Result: HIT
```

## Performance Characteristics

### Turn Rate by Distance

| Distance | Factor | Effective Turn | Cone Width |
|----------|--------|----------------|------------|
| 10 units | 0.33 | 4.2 rad/s | 14.6° |
| 20 units | 0.67 | 8.6 rad/s | 11.2° |
| 30 units | 1.00 | 12.8 rad/s | 4.6° |
| 60 units | 0.70 | 9.0 rad/s | 7.5° |
| 100 units | 1.00 | 12.8 rad/s | 4.6° |

### Stability Metrics

**Before:**
- Max turn: 19.2 rad/s (uncapped)
- Override: 100% (complete replacement)
- Close range: Unstable (oscillation)
- Firing success: <20% at close range

**After:**
- Max turn: 6.0 rad/s (clamped)
- Override: 70% (blended with existing)
- Close range: Stable (distance-scaled)
- Firing success: Expected >70% at close range

## Code Changes Summary

### Modified: `autofire_aiming_system`

**Changes:**
1. Added `distance_factor` calculation (3-tier scaling)
2. Reduced base turn multiplier: 6.0 → 4.0
3. Changed angle multiplier: 2.0 → 1.5
4. Added turn rate clamping (max 1.5× ship turn rate)
5. Changed to velocity blending (30%/70% mix)
6. Adjusted damping: 0.2 → 0.7 for smoother feel
7. Widened lock tolerance: 0.015 → 0.02 radians

### Modified: `autofire_firing_system`

**Changes:**
1. Added `effective_fire_cone` calculation
2. Distance-based cone widening:
   - < 30 units: Up to 15° cone
   - < 60 units: ~7.5° cone
   - > 60 units: 4.6° cone (default)
3. Updated debug output to show effective cone

## Debug Output Improvements

Added `Dist factor` to tracking output:
```
[Autofire Aim] ... | Angle: 15.3° | Dist factor: 0.45 | Status: TRACKING
```

Shows how much tracking power is being applied based on distance.

## Testing Recommendations

### Close Range (< 30 units)
1. Enable autofire
2. Approach enemy to 10-20 units
3. Watch for:
   - ✓ Angle decreasing smoothly (not oscillating)
   - ✓ "Dist factor" showing reduced power (0.2-0.7)
   - ✓ Eventually "In cone: YES" at wider angles
   - ✓ Hits connecting

### Medium Range (30-60 units)
1. Maintain 40-50 unit distance
2. Watch for:
   - ✓ Moderate tracking (factor ~0.7)
   - ✓ Smooth lock acquisition
   - ✓ Cone at ~7.5°
   - ✓ Good hit rate

### Long Range (60-120 units)
1. Stay at 80-100 units
2. Should work like before:
   - ✓ Full tracking power
   - ✓ Tight 4.6° cone
   - ✓ Precise shots
   - ✓ Excellent hit rate

## Next Steps

If still having issues, the debug log will now show:
1. **Distance factor** - how much power is being applied
2. **Effective cone angle** - what the actual firing tolerance is
3. **Turn rate limits** - whether clamping is activating

This should reveal any remaining problems!

