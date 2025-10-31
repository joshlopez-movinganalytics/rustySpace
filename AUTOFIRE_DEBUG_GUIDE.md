# Autofire Debug Guide

## Debug Logging Added

Comprehensive debug messages have been added to diagnose autofire tracking and firing issues.

### What's Being Logged

#### 1. Target Tracking (Every 0.5s)
```
[Autofire Track] Target pos: (x, y, z) | Vel: (vx, vy, vz) speed: S | Dist: D | Projectile speed: PS | Time-to-impact: T | Lead: L units | Intercept: (ix, iy, iz)
```

**What to look for:**
- **Target velocity**: Should match enemy movement (if they're moving)
- **Projectile speed**: Should match your current weapon (Laser=150, Plasma=90, etc.)
- **Time-to-impact**: Distance / Projectile speed
- **Lead**: How far ahead we're aiming (should be non-zero for moving targets)
- **Intercept point**: Where we predict the target will be

#### 2. Aiming Status (Every 0.5s)
```
[Autofire Aim] Forward: (fx, fy, fz) | To-intercept: (ix, iy, iz) | Angle: A° | Status: LOCKED/TRACKING
```

**What to look for:**
- **Forward vector**: Direction ship is facing
- **To-intercept vector**: Direction to predicted intercept point
- **Angle**: How far off target we are (should decrease over time)
- **Status**: 
  - `TRACKING` = Still rotating toward target
  - `LOCKED` = Aligned within 0.86° tolerance

#### 3. Firing Checks (Every 1.0s when in range)
```
[Autofire Fire] Distance: D/MAX | Angle: A°/MAX° | In cone: YES/NO
```

**What to look for:**
- **Distance check**: Must be < 120 units to fire
- **Angle check**: Must be < 4.6° to fire
- **In cone**: Should say "YES" when firing

#### 4. Projectile Firing (Each shot)
```
[Autofire Fire] 🔫 FIRING at target | Weapon speed: S | Player pos: (x, y, z) | Forward: (fx, fy, fz)
```

**What to look for:**
- When this appears, a shot was actually fired
- Weapon speed should match expected value
- Forward direction is where the shot will go

#### 5. Projectile Spawn (Each projectile)
```
[Projectile Spawn] Type: WEAPON | Pos: (x, y, z) | Dir: (dx, dy, dz) | Vel: (vx, vy, vz) | Speed: S
```

**What to look for:**
- **Position**: Should be slightly in front of ship (3 units)
- **Direction**: Should match ship's forward direction (with spread)
- **Velocity**: Direction × Speed
- **Speed**: Should match weapon's projectile_speed

#### 6. Hit Detection (Each hit)
```
[HIT ✓] WEAPON HIT at distance D | Projectile pos: (x, y, z) | Target pos: (x, y, z) | Shield dmg=S, Hull dmg=H
```

**What to look for:**
- **Distance**: Should be < 2.0 units (collision threshold)
- **Positions**: Should be very close together
- This confirms projectile reached the target

#### 7. Miss Detection (Each miss)
```
[MISS ✗] WEAPON expired | Final pos: (x, y, z)
```

**What to look for:**
- Appears when projectile lifetime expires (5 seconds)
- If you see many of these, shots are missing

## How to Debug

### Step 1: Enable Autofire
1. Press **K** to enable autofire
2. Should see console message: `[Combat] Autofire ENABLED`

### Step 2: Check Target Acquisition
Look for `[Autofire Track]` messages. If you don't see them:
- ❌ No enemies in range (< 150 units)
- ❌ Autofire not properly enabled
- ✓ Should appear every 0.5s when target locked

### Step 3: Analyze Tracking
```
[Autofire Track] Target pos: (50.0, 0.0, 0.0) | Vel: (10.0, 0.0, 0.0) speed: 10.0 | Dist: 50.0 | Projectile speed: 150.0 | Time-to-impact: 0.35s | Lead: 3.5 units | Intercept: (53.5, 0.0, 0.0)
```

**Analysis:**
- Target 50 units away, moving right at 10 u/s
- Using Laser (150 u/s)
- Will take 0.35s to reach intercept
- Leading by 3.5 units (10 × 0.35)
- Aiming at (53.5, 0.0, 0.0) instead of current position

### Step 4: Check Aiming
```
[Autofire Aim] Forward: (1.00, 0.00, 0.00) | To-intercept: (0.99, 0.00, 0.14) | Angle: 8.232° | Status: TRACKING
```

**Analysis:**
- Ship facing almost straight ahead
- Need to turn slightly up (0.14 in Y)
- 8° off target, still tracking
- Wait for "LOCKED" status

### Step 5: Verify Firing Conditions
```
[Autofire Fire] Distance: 48.2/120.0 | Angle: 2.1°/4.6° | In cone: YES
```

**Good conditions:**
- ✓ Distance: 48 < 120 (in range)
- ✓ Angle: 2.1° < 4.6° (in firing cone)
- ✓ Should fire!

### Step 6: Confirm Shot Fired
```
[Autofire Fire] 🔫 FIRING at target | Weapon speed: 150 | Player pos: (0.0, 0.0, 0.0) | Forward: (0.99, 0.00, 0.14)
[Projectile Spawn] Type: Laser | Pos: (3.0, 0.0, 0.4) | Dir: (0.99, 0.00, 0.14) | Vel: (148.5, 0.0, 21.0) | Speed: 150
```

**Analysis:**
- Shot fired from (3.0, 0.0, 0.4) - 3 units in front of ship
- Flying in direction (0.99, 0.00, 0.14)
- Velocity matches direction × speed

### Step 7: Track Result
Watch for either:

**Hit:**
```
[HIT ✓] Laser HIT at distance 1.2 | Projectile pos: (52.1, 0.0, 3.2) | Target pos: (53.0, 0.0, 3.5) | Shield dmg=30.0, Hull dmg=3.6
```

**Miss:**
```
[MISS ✗] Laser expired | Final pos: (750.0, 0.0, 105.0)
```

## Common Issues & Solutions

### Issue 1: Not Tracking
**Symptoms:**
- No `[Autofire Track]` messages
- Ship not turning

**Check:**
1. Is autofire enabled? (Press K)
2. Are enemies in range? (< 150 units)
3. Check console for "Autofire ENABLED" message

### Issue 2: Tracking Wrong Direction
**Symptoms:**
- Ship turning away from target
- Intercept point seems wrong

**Check tracking message:**
```
[Autofire Track] ... | Time-to-impact: 0.00s | Lead: 0.0 units ...
```

**Problem:** Time-to-impact is 0 or NaN
**Cause:** Projectile speed is 0 or target is at ship position

### Issue 3: Won't Fire
**Symptoms:**
- Ship tracking correctly ("LOCKED")
- No shots being fired

**Check firing message:**
```
[Autofire Fire] Distance: 48.2/120.0 | Angle: 2.1°/4.6° | In cone: NO
```

**Solutions:**
- If "In cone: NO" - angle too large, wait for alignment
- If no message - distance > 120 or cooldown active
- Check weapon ammo/energy

### Issue 4: All Shots Missing
**Symptoms:**
- Many `[MISS ✗]` messages
- Few or no `[HIT ✓]` messages

**Analysis:**
Compare spawn and target positions:
```
[Projectile Spawn] ... | Dir: (0.99, 0.00, 0.14) ...
[Autofire Track] ... | Intercept: (53.5, 0.0, 0.0)
```

**Problem:** Direction (0.99, 0.00, 0.14) doesn't point toward intercept (53.5, 0.0, 0.0)
**Cause:** Ship rotation not matching prediction

### Issue 5: Lead Calculation Wrong
**Symptoms:**
- Shots missing moving targets
- Lead seems too small or too large

**Check:**
```
[Autofire Track] ... | Projectile speed: 150.0 | Time-to-impact: 0.35s | Lead: 3.5 units
```

**Verify math:**
- Target speed × time = lead
- Example: 10 u/s × 0.35s = 3.5 units ✓

**If wrong:**
- Check projectile speed matches weapon
- Check target velocity is correct

## Example Debug Session

### Successful Lock and Hit
```
1. [Combat] Autofire ENABLED
2. [Autofire Track] Target pos: (60.0, 5.0, 0.0) | Vel: (15.0, 0.0, 0.0) speed: 15.0 | Dist: 60.4 | Projectile speed: 150.0 | Time-to-impact: 0.42s | Lead: 6.3 units | Intercept: (66.3, 5.0, 0.0)
3. [Autofire Aim] Forward: (0.95, 0.08, 0.00) | To-intercept: (0.99, 0.08, 0.00) | Angle: 2.3° | Status: TRACKING
4. [Autofire Aim] Forward: (0.99, 0.08, 0.00) | To-intercept: (0.99, 0.08, 0.00) | Angle: 0.1° | Status: LOCKED
5. [Autofire Fire] Distance: 60.1/120.0 | Angle: 0.1°/4.6° | In cone: YES
6. [Autofire Fire] 🔫 FIRING at target | Weapon speed: 150 | Player pos: (0.0, 0.0, 0.0) | Forward: (0.99, 0.08, 0.00)
7. [Projectile Spawn] Type: Laser | Pos: (3.0, 0.2, 0.0) | Dir: (0.99, 0.08, 0.00) | Vel: (148.5, 12.0, 0.0) | Speed: 150
8. [HIT ✓] Laser HIT at distance 1.8 | Projectile pos: (65.8, 5.1, 0.0) | Target pos: (66.2, 5.0, 0.0) | Shield dmg=30.0, Hull dmg=3.6
```

**Success!** Shot led the target correctly and hit.

### Failed Tracking
```
1. [Combat] Autofire ENABLED
2. [Autofire Track] Target pos: (100.0, 0.0, 0.0) | Vel: (0.0, 0.0, 0.0) speed: 0.0 | Dist: 100.0 | Projectile speed: 90.0 | Time-to-impact: 1.11s | Lead: 0.0 units | Intercept: (100.0, 0.0, 0.0)
3. [Autofire Aim] Forward: (0.50, 0.87, 0.00) | To-intercept: (1.00, 0.00, 0.00) | Angle: 60.0° | Status: TRACKING
4. [Autofire Aim] Forward: (0.87, 0.50, 0.00) | To-intercept: (1.00, 0.00, 0.00) | Angle: 30.0° | Status: TRACKING
5. [Autofire Aim] Forward: (0.99, 0.14, 0.00) | To-intercept: (1.00, 0.00, 0.00) | Angle: 8.1° | Status: TRACKING
6. [Autofire Fire] Distance: 98.2/120.0 | Angle: 8.1°/4.6° | In cone: NO
```

**Problem:** Can't get within firing cone (4.6°)
**Solution:** May need faster turn rate or wider cone

## Performance Note

Debug logging is **verbose** and will spam the console. This is intentional for debugging.

To disable after debugging, comment out or remove the `println!` statements in:
- `autofire_aiming_system`
- `autofire_firing_system`
- `fire_weapon`
- `projectile_lifetime_system`
- `projectile_collision_system`

