# Enhanced Loot & Resource System

## Overview

Enemies now drop valuable resources when destroyed that players can collect to upgrade their ship. The system features magnetic pull collection, visual effects, and balanced drop rates.

## Features

### 💎 Resource Types

1. **Scrap Metal** (Gray Cubes)
   - Drop rate: 40% chance per loot piece
   - Use: Basic hull and armor upgrades
   - Most common resource

2. **Energy Cores** (Blue Spheres)
   - Drop rate: 30% chance per loot piece
   - Use: Shield and power upgrades
   - Common resource

3. **Rare Minerals** (Purple Spheres)
   - Drop rate: 20% chance per loot piece
   - Use: Advanced upgrades
   - Uncommon resource

4. **Tech Components** (Yellow Cubes)
   - Drop rate: 10% chance per loot piece
   - Use: Weapon unlocks and high-tier upgrades
   - Rare resource

### 📦 Drop Rates by Enemy Type

| Enemy Type    | Loot Pieces | Amount Each | Total Resources |
|---------------|-------------|-------------|-----------------|
| Fighter       | 2 pieces    | 1x each     | 2 total         |
| Corvette      | 3 pieces    | 2x each     | 6 total         |
| Frigate       | 4 pieces    | 3x each     | 12 total        |
| Capital Ship  | 6 pieces    | 5x each     | 30 total        |

**Example:** A Corvette drops 3 loot pieces, each worth 2 of a random resource type. You might get 2 Scrap Metal, 2 Energy Cores, and 2 Tech Components.

### 🧲 Magnetic Collection System

**Collection Mechanics:**
- **Attraction Radius:** 15 units - Loot pulls toward you automatically
- **Collection Radius:** 3 units - Get close to collect
- **Pull Strength:** Increases as you get closer
- **Visual Feedback:** Sparkle effect on collection

**How It Works:**
1. Destroy an enemy ship
2. Loot spawns in a circle around the destroyed ship
3. Fly within 15 units - loot automatically pulls toward you
4. Get within 3 units - loot is collected instantly
5. See colorful particle burst confirming collection
6. Resources added to your inventory

### ✨ Visual Effects

**Loot Appearance:**
- **Animated Rotation:** Each piece spins at a random speed
- **Bobbing Motion:** Gentle up/down floating effect
- **Emissive Glow:** Bright, easy-to-spot colors
- **Different Shapes:** Cubes for metal/tech, spheres for energy/minerals
- **Metallic Sheen:** Professional 3D material appearance

**Collection Effects:**
- **Particle Burst:** 8 colorful particles explode outward
- **Upward Motion:** Particles rise and fade
- **Color Matched:** Particles match the resource type
- **Scale Down:** Particles shrink as they fade

**Loot Lifetime:**
- Loot persists for 60 seconds
- After 60 seconds, uncollected loot despawns
- Prevents performance issues from accumulating loot

### 🎮 Gameplay Impact

**Resource Flow:**
1. Defeat enemies → Resources drop
2. Collect resources automatically when nearby
3. Press **U** to open upgrade menu
4. Purchase upgrades with collected resources
5. Stronger ship → Defeat tougher enemies → Better loot

**Starting Resources:**
- Scrap Metal: 100
- Energy Cores: 50
- Rare Minerals: 25
- Tech Components: 10

**Farming Strategy:**
- Fighters: Quick kills, steady scrap/energy income
- Corvettes: Balanced resource farming
- Frigates: Good for rare minerals
- Capital Ships: High-value targets for all resources

### 🔧 Technical Implementation

**New Systems:**
- `loot_collection_system` - Magnetic pull and collection with effects
- `spawn_loot_system` - Improved drop rates and visual variety
- `animate_loot_system` - Rotation and bobbing animations
- `update_collection_particles` - Collection effect particles

**Components:**
- `Loot` - Resource type and amount
- `LootVisual` - Lifetime and rotation speed
- `CollectionParticle` - Temporary visual effect
- `Velocity` - Physics for loot movement

**Key Features:**
- Weighted random distribution (more common resources)
- Circular spawn pattern around destroyed ships
- Different mesh types for visual variety
- Velocity inheritance from destroyed ship
- Automatic cleanup after 60 seconds
- Loot cleared on restart/load/main menu return

### 📊 Resource Weighting

The system uses weighted random distribution to ensure a good balance:

```
Scrap Metal:      40% ████████
Energy Cores:     30% ██████
Rare Minerals:    20% ████
Tech Components:  10% ██
```

This ensures players always have enough common resources while rare resources remain valuable.

### 🎯 Collection Tips

1. **Fly Through Debris Fields:** After a big battle, circle around to collect everything
2. **Don't Chase Individual Pieces:** The magnetic pull does the work for you
3. **Watch for Glow:** Brightly glowing objects are valuable loot
4. **Capital Ships = Jackpot:** 30 resources is a huge boost
5. **60-Second Timer:** Don't leave loot sitting too long
6. **Collection Radius is Small:** Get close to actually pick it up

### 🚀 Future Enhancement Ideas

- Rare "super loot" drops with bonus amounts
- Loot multipliers for kill streaks
- Special loot from specific enemy types
- Resource conversion system
- Loot vacuum ability upgrade
- Audio feedback for collection
- Loot quality tiers (common/rare/epic)
- Visual trails leading to loot

### 🐛 Cleanup Behavior

Loot is automatically cleaned up in these scenarios:
- **Game Restart:** All loot despawned
- **Load Save:** All loot cleared
- **Return to Main Menu:** All loot removed
- **60-Second Timeout:** Individual pieces despawn

This prevents performance issues and ensures clean state transitions.

### 📝 Console Messages

The system logs helpful debug information:
```
[Resources System] Collected Scrap Metal x2
[Resources System] Collected Energy Core x3
[Resources System] Collected Tech Component x5
```

### 🎨 Visual Color Guide

- **Gray/White Glow:** Scrap Metal - Common building material
- **Cyan/Blue Glow:** Energy Cores - Power systems
- **Magenta/Purple Glow:** Rare Minerals - Advanced materials  
- **Gold/Yellow Glow:** Tech Components - Valuable technology

The brighter the glow, the easier it is to spot in space!

