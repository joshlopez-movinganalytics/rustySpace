# Weapons System Guide

## Overview

The weapon system has been completely redesigned with **strategic damage profiles** and **unique alt-fire modes**. Each weapon now has distinct roles in combat.

## How to Test

1. **Start the game** and press Enter at main menu
2. **Look at the console output** when you hit enemies - you'll see:
   ```
   [Combat] Laser hit: Shield dmg=30.0 (base=12.0x2.50), Hull dmg=3.6 (base=12.0x0.30)
   [Combat] Autocannon hit: Shield dmg=5.6 (base=14.0x0.40), Hull dmg=28.0 (base=14.0x2.00)
   ```
3. **Switch weapons** with 1, 2, 3 keys
4. **Alt-fire** with Right Mouse Button
5. **Missiles** will print when they acquire targets

## Weapon Profiles

### 1. **Laser** (Key: 1) - ANTI-SHIELD
**Role:** Shield Breaker  
**Primary Fire:** 
- Damage: 12
- Fire Rate: 6/sec
- Shield Damage: **30.0** (2.5x multiplier)
- Hull Damage: **3.6** (0.3x multiplier)
- Speed: 150 units/sec
- Energy: 4 per shot

**Alt-Fire (Right Click):** 3-Shot Burst
- Fires 3 shots rapidly
- Costs 12 energy (3x primary)
- Great for quickly dropping shields

**Tactical Use:**
- ✅ Primary weapon against shielded targets
- ✅ Lead with laser to strip shields
- ❌ Ineffective against hull - switch weapons when shields down

---

### 2. **Autocannon** (Key: 2) - ANTI-HULL
**Role:** Armor Piercer  
**Primary Fire:**
- Damage: 14
- Fire Rate: 8/sec (rapid fire!)
- Shield Damage: **5.6** (0.4x multiplier)
- Hull Damage: **28.0** (2.0x multiplier)
- Speed: 140 units/sec
- Energy: 3 per shot
- Spread: 0.015

**Alt-Fire (Right Click):** Shotgun Blast
- Fires 5 pellets in wide spread
- Each pellet: 60% damage
- Costs 15 energy (5x primary)
- Devastating at close range

**Tactical Use:**
- ✅ Finish enemies after shields are down
- ✅ High DPS against hull
- ❌ Wastes ammo on shields

---

### 3. **Plasma** (Key: 3) - BALANCED
**Role:** All-Rounder  
**Primary Fire:**
- Damage: 22
- Fire Rate: 2.5/sec
- Shield Damage: **26.4** (1.2x multiplier)
- Hull Damage: **28.6** (1.3x multiplier)
- Speed: 90 units/sec
- Energy: 12 per shot
- Spread: 0.02

**Alt-Fire (Hold & Release Right Click):** Charged Shot
- Hold right mouse to charge (up to 2 seconds)
- Damage scales with charge time (up to 3x)
- Has area damage radius (3x charge time)
- Costs energy based on charge

**Tactical Use:**
- ✅ Versatile - works on shields or hull
- ✅ Charged shot for heavy damage
- ✅ Good mid-range weapon

---

### 4. **Missile** (Unlock: Upgrade Menu) - HOMING EXPLOSIVE
**Role:** Fire-and-Forget  
**Primary Fire:**
- Damage: 40
- Fire Rate: 1.2/sec
- Shield Damage: **60.0** (1.5x multiplier - 60% of total)
- Hull Damage: **40.0** (1.0x multiplier - 40% of total)
- Speed: 60 units/sec (slow but tracks!)
- Energy: 20 per shot
- **HOMING:** Automatically tracks nearest enemy
- **AREA DAMAGE:** 8 unit radius

**Alt-Fire (Right Click):** Missile Swarm
- Fires 3 smaller missiles
- Each: 50% damage, 6 unit radius
- Costs 40 energy (2x primary)
- All missiles home independently

**Tactical Use:**
- ✅ Fire and forget - keeps tracking
- ✅ Area damage hits multiple enemies
- ✅ Good against evasive targets
- ⚠️ Slow projectile speed
- Console shows: `[Combat] Missile acquired target, homing strength: 15.0`

---

### 5. **Railgun** (Unlock: Upgrade Menu) - PRECISION PIERCING
**Role:** Sniper  
**Primary Fire:**
- Damage: 60
- Fire Rate: 0.8/sec (slow but powerful)
- Shield Damage: **36.0** (0.6x multiplier)
- Hull Damage: **150.0** (2.5x multiplier!)
- Speed: 300 units/sec (very fast)
- Energy: 35 per shot
- **PIERCING:** Goes through enemies

**Alt-Fire (Right Click):** Overcharged Shot
- 130% damage
- Still pierces
- Costs 52.5 energy (1.5x primary)

**Tactical Use:**
- ✅ Devastating to hull
- ✅ Can hit multiple enemies in a line
- ✅ Excellent for finishing blows
- ❌ Poor against shields
- ❌ High energy cost

---

### 6. **Ion Cannon** (Unlock: Upgrade Menu) - SHIELD DESTROYER
**Role:** Pure Shield Damage  
**Primary Fire:**
- Damage: 8
- Fire Rate: 3/sec
- Shield Damage: **40.0** (5.0x multiplier!)
- Hull Damage: **0.8** (0.1x multiplier - almost nothing)
- Speed: 120 units/sec
- Energy: 15 per shot

**Alt-Fire:** Shield Disruptor Pulse  
*(Not yet implemented - placeholder for future)*

**Tactical Use:**
- ✅ Absolutely destroys shields
- ✅ Best shield DPS in game
- ❌ Completely useless against hull
- Must combo with hull weapon

---

### 7. **Flak Cannon** (Unlock: Upgrade Menu) - AREA DENIAL
**Role:** Close-Range Crowd Control  
**Primary Fire:**
- Damage: 18
- Fire Rate: 2/sec
- Shield Damage: **18.0** (1.0x multiplier)
- Hull Damage: **27.0** (1.5x multiplier)
- Speed: 100 units/sec
- Energy: 14 per shot
- Spread: 0.04 (wide)
- **AREA DAMAGE:** 5 unit radius

**Alt-Fire:** Wide Barrage  
*(Not yet implemented - placeholder for future)*

**Tactical Use:**
- ✅ Hits multiple enemies
- ✅ Good damage to hull
- ✅ Area denial
- ⚠️ Inaccurate at range

---

### 8. **Beam Laser** (Unlock: Upgrade Menu) - CONTINUOUS
**Role:** Sustained Shield Damage  
**Primary Fire:**
- Damage: 7 per shot
- Fire Rate: 15/sec (beam-like)
- Shield Damage: **14.0** (2.0x multiplier)
- Hull Damage: **5.6** (0.8x multiplier)
- Speed: 250 units/sec
- Energy: 2.5 per shot

**Alt-Fire:** Focused Beam  
*(Not yet implemented - placeholder for future)*

**Tactical Use:**
- ✅ Steady shield pressure
- ✅ Low energy per shot
- ✅ High projectile speed
- ⚠️ Continuous fire drains energy fast

---

## Combat Strategy

### Phase 1: Shield Breaking
**Use:** Laser, Ion Cannon, or Beam Laser
- Focus fire with shield-breaking weapons
- Watch for shield bar to deplete
- Console shows high shield damage numbers

### Phase 2: Hull Damage
**Use:** Autocannon or Railgun
- Switch immediately when shields drop
- Autocannon for sustained DPS
- Railgun for burst damage
- Console shows high hull damage numbers

### Balanced Approach
**Use:** Plasma or Missiles
- Works on both shields and hull
- Less optimal but more flexible
- Good for beginners

### Advanced Tactics
1. **Laser + Autocannon Combo:** Strip shields with laser (key 1), switch to autocannon (key 2) for hull
2. **Ion Cannon + Railgun:** Ion for shields, railgun one-shot to hull
3. **Missile Swarm:** Fire and forget, handles itself
4. **Plasma Charged:** Hold right-click, release for massive damage

## Damage Multiplier Reference

| Weapon | vs Shields | vs Hull | Best For |
|--------|-----------|---------|----------|
| Laser | 2.5x | 0.3x | Shields |
| Autocannon | 0.4x | 2.0x | Hull |
| Plasma | 1.2x | 1.3x | Both |
| Missile | 1.5x | 1.0x | Area |
| Railgun | 0.6x | 2.5x | Hull |
| Ion Cannon | 5.0x | 0.1x | Shields |
| Flak Cannon | 1.0x | 1.5x | Area/Hull |
| Beam Laser | 2.0x | 0.8x | Shields |

## Testing Checklist

- [ ] Fire laser at enemy - see high shield damage in console
- [ ] Fire autocannon at enemy - see high hull damage in console  
- [ ] Fire plasma (key 3) - see balanced damage
- [ ] Right-click with laser - see 3-shot burst
- [ ] Right-click with autocannon - see shotgun spread
- [ ] Hold right-click with plasma - charge, then release
- [ ] Fire missile (if unlocked) - see "Missile acquired target" message
- [ ] Watch missile curve toward enemy
- [ ] See explosion effect on missile impact
- [ ] Watch railgun pierce through multiple enemies

## Console Output Examples

```
[Combat] Laser hit: Shield dmg=30.0 (base=12.0x2.50), Hull dmg=3.6 (base=12.0x0.30)
[Combat] Autocannon hit: Shield dmg=5.6 (base=14.0x0.40), Hull dmg=28.0 (base=14.0x2.00)
[Combat] Missile hit: Shield dmg=60.0 (base=40.0x1.50), Hull dmg=40.0 (base=40.0x1.00)
[Combat] Missile acquired target, homing strength: 15.0
```

## Known Issues / To Be Implemented

1. Some alt-fire modes are placeholders:
   - Ion Cannon alt-fire
   - Flak Cannon alt-fire  
   - Beam Laser alt-fire

2. Need to add more weapons to upgrade menu (currently only Plasma, Missile, Railgun unlockable)

3. Enemy AI doesn't use new damage profiles yet

## Future Enhancements

- Visual indicators for damage type (color-coded hit markers)
- Shield/Hull damage numbers floating above enemies
- Weapon charge meter for plasma
- Missile lock-on indicator
- Weapon effectiveness meter in HUD
- More alt-fire modes
- Weapon modification system

