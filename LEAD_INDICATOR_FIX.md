# Lead Indicator Accuracy Fix

## Problem Statement

The lead indicator was inaccurate because:

1. **Projectiles were inheriting momentum** from the shooter, making the interception calculation more complex
2. **The intercept calculation was overly complicated** by trying to account for shooter velocity
3. **The mathematical model didn't match the actual game physics**

## Solution

### 1. Removed Momentum Inheritance from Projectiles

**Changed:** Projectiles now travel at a fixed speed in world space, not relative to the shooter.

**Before:**
```rust
let projectile_velocity = projectile_direction * weapon.projectile_speed + velocity.0;
```

**After:**
```rust
// Projectiles do NOT inherit momentum - they travel at fixed speed relative to world
let projectile_velocity = projectile_direction * weapon.projectile_speed;
```

**Files Modified:**
- `src/systems/combat.rs` - Player weapons (lines 222, 327)
- `src/systems/ai.rs` - Enemy weapons (line 312)

This makes the game feel more responsive and predictable, similar to classic space shooters.

---

### 2. Simplified Lead Indicator Calculation

**The Classic Interception Problem:**

Given:
- **P0**: Target's current position
- **V0**: Target's velocity vector (direction × speed)
- **P1**: Shooter's position (projectile origin)
- **s1**: Projectile speed (scalar)

Find:
- **V**: The direction vector for the projectile (where to aim)
- **t**: Time of impact

**Mathematical Solution:**

At time `t`:
- Projectile position: `P1 + V*t` where `|V| = s1`
- Target position: `P0 + V0*t`

These must be equal for an intercept:
```
P1 + V*t = P0 + V0*t
```

Since we know the projectile speed but not the direction, we rearrange:
```
|P0 + V0*t - P1| = s1 * t
```

Squaring both sides to eliminate the magnitude:
```
|to_target + target_velocity*t|² = projectile_speed² * t²
```

Expanding:
```
|to_target|² + 2*(to_target · target_velocity)*t + |target_velocity|²*t² = projectile_speed²*t²
```

This gives us a quadratic equation:
```
a*t² + b*t + c = 0
```

Where:
- `a = |target_velocity|² - projectile_speed²`
- `b = 2 * (to_target · target_velocity)`
- `c = |to_target|²`

**Code Implementation:**

```rust
fn calculate_intercept_point(
    shooter_pos: Vec3,
    _shooter_velocity: Vec3,  // Not used since projectiles don't inherit momentum
    target_pos: Vec3,
    target_velocity: Vec3,
    projectile_speed: f32,
    _shoot_direction: Vec3,
) -> Option<Vec3> {
    let to_target = target_pos - shooter_pos;
    
    // Quadratic coefficients
    let a = target_velocity.length_squared() - projectile_speed * projectile_speed;
    let b = 2.0 * to_target.dot(target_velocity);
    let c = to_target.length_squared();
    
    // Solve for time t
    let time_to_intercept = solve_quadratic(a, b, c)?;
    
    // Calculate intercept position
    Some(target_pos + target_velocity * time_to_intercept)
}
```

---

## Special Cases Handled

### 1. **Linear Case** (`a ≈ 0`)
When target speed ≈ projectile speed, the equation becomes linear:
```
b*t + c = 0  →  t = -c/b
```

### 2. **No Solution** (`discriminant < 0`)
Target is too fast to intercept. Falls back to simple prediction:
```rust
let simple_time = distance / projectile_speed;
let fallback_pos = target_pos + target_velocity * simple_time;
```

### 3. **Target Moving Away**
If both solutions are negative, intercept is impossible (return `None`).

### 4. **Very Close Target**
If distance < 0.1, just aim at current target position.

---

## Benefits

### 1. **Accuracy**
- ✅ Exact mathematical solution
- ✅ Accounts for target velocity precisely
- ✅ No iterative approximation errors

### 2. **Performance**
- ✅ Single calculation instead of 15+ iterations
- ✅ Simpler code path
- ✅ Fewer safety checks needed

### 3. **Predictability**
- ✅ Projectiles always travel at the same world speed
- ✅ More intuitive for players
- ✅ Consistent with classic space shooters

### 4. **Code Quality**
- ✅ Cleaner, more maintainable code
- ✅ Well-documented with mathematical explanation
- ✅ Easier to debug and test

---

## Testing Recommendations

1. **Stationary Targets**
   - Lead indicator should NOT appear (target speed < 5.0)
   - Shots should hit directly where aimed

2. **Moving Targets (Perpendicular)**
   - Lead indicator should appear ahead of target
   - Shots should hit the predicted position

3. **Moving Targets (Away)**
   - Lead indicator should appear far ahead
   - If target too fast, indicator may disappear (no solution)

4. **Moving Targets (Toward)**
   - Lead indicator should appear behind or at target
   - Easier intercept, shorter lead time

5. **Player Movement**
   - Player velocity no longer affects lead indicator
   - Shots go exactly where aimed regardless of player speed

6. **Fast vs Slow Projectiles**
   - Faster weapons (lasers): smaller lead
   - Slower weapons (missiles): larger lead

---

## Visual Differences

### Before:
- Lead indicator tried to account for player momentum
- Could appear in unexpected places
- Less accurate for edge cases

### After:
- Lead indicator only accounts for target motion
- More consistent and predictable
- Exactly where you need to aim for a hit

---

## Edge Cases

### Target Faster Than Projectile
If `|target_velocity| > projectile_speed`, discriminant may be negative:
- **Approaching target**: Still solvable (positive time)
- **Fleeing target**: No solution (falls back to simple prediction)

### Extreme Angles
When target moves perpendicular to line of fire:
- Automatically handled by the dot product in `b`
- Lead indicator appears at correct angle

### Zero Velocity
If target is stationary (`target_velocity ≈ 0`):
- `a = -projectile_speed²` (negative)
- `b = 0`
- `t = √(c/projectile_speed²) = distance/speed`
- Lead indicator not shown (target speed < 5.0 threshold)

---

## Mathematical Verification

For a target at position `(100, 0, 0)` moving at `(0, 10, 0)` with projectile speed `150`:

```
to_target = (100, 0, 0)
target_velocity = (0, 10, 0)

a = 10² - 150² = 100 - 22500 = -22400
b = 2 * (100*0 + 0*10 + 0*0) = 0
c = 100² = 10000

discriminant = 0 - 4*(-22400)*10000 = 896,000,000
sqrt(discriminant) = 29,933.26

t = (0 - 29933.26) / (2 * -22400) = 0.668 seconds

intercept_point = (100, 0, 0) + (0, 10, 0) * 0.668
                = (100, 6.68, 0)
```

The projectile will intercept the target 6.68 units ahead of its current position.

---

## Files Changed

1. **src/systems/combat.rs**
   - Removed momentum inheritance from player projectiles (2 locations)
   - Added comments explaining the change

2. **src/systems/ai.rs**
   - Removed momentum inheritance from enemy projectiles
   - Added comments explaining the change

3. **src/systems/ui.rs**
   - Completely rewrote `calculate_intercept_point()` function
   - Updated comments and documentation
   - Removed player velocity from lead indicator visibility check
   - Simplified the algorithm significantly

---

## Performance Impact

- **Projectile Physics**: Negligible (simpler calculation)
- **Lead Indicator**: Improved (1 calculation vs 15 iterations)
- **Overall**: Small performance gain

---

## Known Limitations

1. **Assumes Constant Velocity**: Targets changing direction will cause misses (realistic!)
2. **No Acceleration**: Doesn't account for target acceleration
3. **No Obstacles**: Doesn't predict target collision/avoidance
4. **Fixed Speed**: Projectiles travel at constant speed (could add drag in future)

These are intentional design choices that make the game feel more skill-based and less "auto-aim".

