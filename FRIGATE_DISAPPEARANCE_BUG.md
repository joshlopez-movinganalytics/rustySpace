# Frigate Disappearance Bug - Investigation & Fix

## The Problem

While playing the game, the auto-turret was tracking and firing at a Frigate at 126.5 units away. Suddenly, the turret started logging NaN (Not a Number) values for the target's position and velocity, indicating the entity no longer existed. However:

- **No death message** was logged
- **No explosion** was spawned
- **No loot** was dropped
- The frigate just **vanished**

## Log Evidence

From terminal output:
```
Line 255: [Turret Fire] Distance: 126.5/200.0 | Angle: 70.108°/5.730° | In cone: NO
Line 257: [Turret Fire] 🎯 TURRET FIRING | Weapon: Laser | Speed: 150 | Turret pos: (-575.0, -96.5, 308.9)
Line 259: [Turret Track] Target pos: (NaN, NaN, NaN) | Vel: (NaN, NaN, NaN) speed: NaN | Dist: NaN | Lead: NaN units
```

The entity disappeared between frames without going through the normal death system.

## Root Causes Identified

### 1. Missing Error Handling in Turret Systems

The turret aiming and targeting systems had **no error handling** when an enemy entity vanished:

```rust
// OLD CODE - autofire_aiming_system
let target_entity = turret.current_target.unwrap();
if let Ok((target_transform, target_velocity)) = enemy_query.get(target_entity) {
    // ... tracking code ...
}
// NO ELSE BLOCK! Target stays set to invalid entity
```

This caused:
- The turret to keep "targeting" a non-existent entity
- NaN values when trying to calculate tracking (entity doesn't exist)
- Confusing debug output

### 2. Insufficient Logging

There was no logging to detect:
- When a target vanished from the query without dying
- When the damage system marks ships as dead
- When targets are acquired/lost in the targeting system

This made it impossible to diagnose where entities disappeared.

## Why the Frigate Disappeared

**Still investigating**, but possible causes:

1. **Bevy ECS internal despawn** - Some component might have panicked during update, causing Bevy to despawn the entity
2. **Parent-child hierarchy issue** - Ships use `despawn_recursive()` which might have edge cases
3. **Component removal bug** - If a required component is removed, query might fail
4. **Race condition** - Entity marked for despawn but not yet processed

The new logging will help us identify the exact cause when it happens again.

## Fixes Implemented

### Fix 1: Clear Invalid Targets (autofire_aiming_system)

```rust
let target_entity = turret.current_target.unwrap();
if let Ok((target_transform, target_velocity)) = enemy_query.get(target_entity) {
    // ... existing tracking code ...
} else {
    // NEW: Target no longer exists (died, despawned, or otherwise removed)
    println!("[Turret Track] ⚠️ TARGET LOST - Entity no longer exists, clearing target");
    turret.current_target = None;
}
```

**Impact:** The turret will now gracefully handle when a target vanishes and clear it properly.

### Fix 2: Enhanced Logging in Targeting System

```rust
pub fn autofire_targeting_system(
    mut player_query: Query<(&Transform, &mut AutoTurret), With<Player>>,
    enemy_query: Query<(Entity, &Transform, &Health, &Enemy), (With<Enemy>, Without<Player>)>,
) {
    // ... code ...
    
    if let Ok((_, target_transform, _, enemy_type)) = enemy_query.get(old_target_entity) {
        // ... range check ...
    } else {
        // NEW: Log when target vanishes
        println!("[Turret Targeting] ⚠️ TARGET VANISHED - Old target no longer in query, acquiring new target");
        turret.current_target = closest_enemy;
    }
    
    // NEW: Log when acquiring new targets
    if closest_enemy.is_some() {
        if let Some(new_target) = closest_enemy {
            if let Ok((_, _, _, enemy_type)) = enemy_query.get(new_target) {
                println!("[Turret Targeting] 🎯 New target acquired: {:?} at {:.1} units",
                    enemy_type.enemy_type, closest_distance);
            }
        }
    }
}
```

**Impact:** We'll now see exactly when targets are acquired, lost, or vanish unexpectedly.

### Fix 3: Death Logging in Damage System

```rust
pub fn damage_system(
    mut commands: Commands,
    query: Query<(Entity, &Health, Option<&Enemy>)>,
) {
    for (entity, health, enemy) in query.iter() {
        if health.current <= 0.0 {
            // NEW: Log when marking ships as dead
            if let Some(enemy) = enemy {
                println!("[Damage System] Marking {:?} as dead (Health: {:.1})", 
                    enemy.enemy_type, health.current);
            } else {
                println!("[Damage System] Marking player as dead (Health: {:.1})", 
                    health.current);
            }
            commands.entity(entity).try_insert(DeadShip);
        }
    }
}
```

**Impact:** We'll see if the frigate was marked as dead, or if it vanished without going through the damage system.

## Expected Log Output (Next Time This Happens)

### If the frigate dies normally:
```
[HIT ✓] Laser HIT at distance X | ...
[Damage System] Marking Frigate as dead (Health: -5.2)
[Combat System] Enemy ship destroyed
[Turret Targeting] ⚠️ TARGET VANISHED - Old target no longer in query, acquiring new target
```

### If the frigate vanishes mysteriously:
```
[Turret Track] Target pos: (126.5, -96.5, 308.9) | ...
[Turret Targeting] ⚠️ TARGET VANISHED - Old target no longer in query, acquiring new target
[Turret Track] ⚠️ TARGET LOST - Entity no longer exists, clearing target
```
**(Note: NO damage or death message)**

## Next Steps

1. **Play test** and watch for the new log messages
2. **If it happens again**, the logs will tell us:
   - Did the damage system mark it dead?
   - Did it vanish between frames?
   - When exactly did the entity disappear?

3. **If we see the "VANISHED" message without a death message**, we know it's a bug in entity lifecycle management, not combat

## Files Modified

- `src/systems/combat.rs`:
  - `autofire_targeting_system()` - Added logging and `Enemy` component to query
  - `autofire_aiming_system()` - Added else block to clear invalid targets
  - `damage_system()` - Added death logging

## Prevention

The fixes ensure:
✅ Invalid targets are cleared immediately
✅ We get comprehensive logging to diagnose the root cause
✅ The turret won't try to track ghost entities
✅ No more NaN spam in logs

