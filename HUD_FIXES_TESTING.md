# HUD Fixes - Testing Guide

## What Was Fixed

### Issue 1: Scroll scrolled whole page
**Root Cause:** The scroll container itself was being moved instead of its content.

**Fix Applied:**
- Created nested structure: 
  - `UpgradeScrollContainer` (outer wrapper with `overflow: clip_y()`)
  - `UpgradeScrollContent` (inner content that moves)
- Scroll system now targets `UpgradeScrollContent` only
- Outer container stays fixed, inner content slides up/down

### Issue 2: Reticle stuck to player ship
**Root Cause:** Unclear - possibly rendering/positioning issue.

**Fix Applied:**
- Reticle now uses full-screen flex container for centering
- Parent uses `justify_content: Center` and `align_items: Center`
- Child reticle is centered via flex layout (not absolute positioning)
- Added `ZIndex::Global(100)` to ensure it's on top
- Added Without<Player> to queries to prevent conflicts
- Set up as completely separate UI tree (not child of HudRoot)

---

## How to Test

### Test 1: Resource Display
```
1. Start game (press Enter at main menu)
2. Look at top-left HUD
3. Should see:
   HULL ████
   SHIELDS ████
   ENERGY ████
   SCRAP: 100
   CORES: 50
   MINERALS: 25
   TECH: 10
   
4. Destroy enemies and collect loot
5. Watch numbers update in real-time
```

**Expected:** Resource numbers change as you collect loot.

---

### Test 2: Targeting Reticule Position
```
1. Start game
2. Look at CENTER of screen (not near player ship)
3. Should see green circle with white dot in exact center
4. Move your ship around
5. Reticle should STAY in center of screen (not follow ship)
6. Fly toward an enemy
7. When aiming at enemy, reticle turns RED
8. Turn away, reticle turns GREEN again
```

**Expected:** Reticle is screen-centered at all times, changes color on target.

**Console should show:**
```
[UI System] Setting up targeting reticule
```

---

### Test 3: Trackpad Scrolling
```
1. Press U to open upgrade menu
2. Use two-finger swipe UP on trackpad
3. Content should scroll DOWN (showing lower items)
4. Use two-finger swipe DOWN on trackpad
5. Content should scroll UP (showing upper items)
6. Scroll should stop at top (can't scroll above first item)
7. Title and resources should STAY FIXED at top
8. Instructions should STAY FIXED at bottom
```

**Expected:** 
- Only the upgrade list scrolls
- Header and footer stay in place
- Smooth scrolling with trackpad
- Content can't scroll beyond limits

---

### Test 4: Upgrade Notification
```
1. Start game with default resources
2. Look at HUD below resources
3. Should see "⚡ UPGRADES AVAILABLE (U)" pulsing
4. Press U and buy all affordable upgrades
5. Return to game (ESC)
6. Notification should disappear (no upgrades affordable)
7. Collect more loot
8. When you can afford upgrade, notification reappears
```

**Expected:** Notification shows/hides based on affordability, pulses when visible.

---

## Debug Console Messages

Watch for these messages:

```
[UI System] Setting up targeting reticule
[UI System] Resources - Scrap: X, Energy Cores: Y...
[UI System] Cleaning up N targeting reticule(s)
```

---

## Known Behaviors

1. **Reticle Centering:** Uses flex layout for perfect centering
2. **Scroll Speed:** Trackpad = 1:1 pixel, Mouse wheel = 50px per line
3. **Scroll Limits:** -2000px to 0px (should be enough for all upgrades)
4. **Notification Pulse:** 1.5 second period, scales 1.0x to 1.15x

---

## If Issues Persist

### Reticle still moving:
- Check console for "Setting up targeting reticule" message
- Verify reticle is being spawned OnEnter(InGame)
- Check if cleanup is being called when leaving game

### Scroll not working:
- Verify you're in Upgrade state (press U)
- Check that UpgradeScrollContent component exists
- Try mouse wheel as alternative

### Resources not updating:
- Check console for inventory change messages
- Verify ResourceText components are spawned
- Check that inventory is actually changing

---

## Architecture

```
UI Tree Structure:

HudRoot (top-left)
├── Health bar
├── Shield bar
├── Energy bar
└── Resources
    ├── SCRAP: 100
    ├── CORES: 50
    ├── MINERALS: 25
    └── TECH: 10

TargetingReticule (screen center, separate tree)
├── Container (flex centered)
    ├── Circle (border)
    └── Dot (filled)

UpgradeMenuRoot
├── Title
├── Resources display
├── UpgradeScrollContainer (clips)
│   └── UpgradeScrollContent (moves on scroll)
│       ├── Hull category
│       ├── Shield category
│       └── etc...
└── Instructions (fixed)
```

The reticle is completely independent and should not be affected by player movement.

