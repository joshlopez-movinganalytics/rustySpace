# Frigate Disappearance Bug - Fixes Summary

## What Happened

The frigate entity vanished mysteriously during gameplay without going through the normal death sequence (no explosion, no loot, no death message). The turret kept trying to track it, resulting in NaN values.

## Changes Made

### 1. Fixed Missing Error Handling in Turret Aiming System
**File:** `src/systems/combat.rs` - `autofire_aiming_system()`

**Before:**
```rust
if let Ok((target_transform, target_velocity)) = enemy_query.get(target_entity) {
    // ... tracking code ...
}
// No else - target stays set to invalid entity!
```

**After:**
```rust
if let Ok((target_transform, target_velocity)) = enemy_query.get(target_entity) {
    // ... tracking code ...
} else {
    // Clear invalid target
    println!("[Turret Track] ⚠️ TARGET LOST - Entity no longer exists, clearing target");
    turret.current_target = None;
}
```

### 2. Added Comprehensive Logging to Targeting System
**File:** `src/systems/combat.rs` - `autofire_targeting_system()`

Now logs:
- When targets are acquired: `🎯 New target acquired: Frigate at 126.5 units`
- When targets vanish unexpectedly: `⚠️ TARGET VANISHED - Old target no longer in query`
- When targets go out of range: `Target Frigate out of range (250.0 > 200.0)`

### 3. Added Death Logging to Damage System
**File:** `src/systems/combat.rs` - `damage_system()`

Now logs when ships are marked as dead:
```rust
if health.current <= 0.0 {
    println!("[Damage System] Marking {:?} as dead (Health: {:.1})", 
        enemy.enemy_type, health.current);
    commands.entity(entity).try_insert(DeadShip);
}
```

## How This Helps

### Immediate Benefits
✅ **No more NaN spam** - Turret clears invalid targets immediately
✅ **Better debugging** - We'll see exactly when and why targets disappear
✅ **More stable** - Graceful handling of edge cases

### Diagnostic Value
The next time this happens, the logs will show us:

**Scenario A - Normal Death:**
```
[HIT ✓] Laser HIT at distance 2.5 | Shield dmg=30.0, Hull dmg=3.6
[Damage System] Marking Frigate as dead (Health: -2.3)
[Combat System] Enemy ship destroyed
[Turret Targeting] ⚠️ TARGET VANISHED - Old target no longer in query, acquiring new target
```

**Scenario B - Mysterious Disappearance (the bug):**
```
[Turret Track] Target pos: (126.5, -96.5, 308.9) | Vel: (12.3, 4.5, -8.1) ...
[Turret Targeting] ⚠️ TARGET VANISHED - Old target no longer in query, acquiring new target
[Turret Track] ⚠️ TARGET LOST - Entity no longer exists, clearing target
```
**Notice:** No damage message, no death message = entity despawned by something else!

## Next Steps

1. **Play test** and watch for these new log messages
2. **If the bug happens again**, check the logs:
   - Did you see `[Damage System] Marking Frigate as dead`?
     - **YES** → Normal death, working as intended
     - **NO** → Entity despawned by something else (the bug!)

3. **If it's the bug**, the logs will show:
   - What type of ship vanished
   - Where it was when it vanished
   - Whether it happened during targeting or aiming phase

## Compilation Status

✅ **Successfully compiled** with no errors
⚠️ Minor warnings present (unrelated to the fix)

## Testing Checklist

When you run the game next, watch for:
- [ ] New target acquisition messages
- [ ] Target loss messages when ships die normally
- [ ] If the bug happens, note if there's a death message or not
- [ ] No more NaN spam in turret tracking

## Files Modified

- `src/systems/combat.rs` (3 functions updated)
- `FRIGATE_DISAPPEARANCE_BUG.md` (new documentation)
- `FIXES_SUMMARY.md` (this file)

