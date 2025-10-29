# Pause, Restart, and Save/Load Features

This document describes the newly implemented pause, restart, and save/load features for the Space Combat Game.

## Overview

Three major features have been added to improve game flow and player experience:
1. **Pause Menu** - Pause the game at any time
2. **Game Over Screen** - Options to restart or load saved game after death
3. **Save/Load System** - Persistent save files to continue your progress

## Features

### 1. Pause Menu (ESC Key)

Press `ESC` during gameplay to pause the game and access the pause menu.

**Pause Menu Options:**
- **Resume (ESC)** - Resume the game immediately
- **Save Game** - Save your current progress to disk
- **Exit to Main Menu** - Return to the main menu

**Implementation Details:**
- New `GameState::Paused` state added
- Cursor is unlocked when paused
- All game systems are paused (no updates to enemies, projectiles, etc.)
- Pause menu UI with semi-transparent overlay

### 2. Game Over Screen

When your ship is destroyed, the game transitions to a game over screen with options:

**Game Over Options:**
- **Restart (New Game)** - Start a fresh game with default resources and upgrades
- **Load Saved Game** - Load your most recent save file (only shown if a save exists)
- **Main Menu** - Return to the main menu

**Implementation Details:**
- New `GameState::GameOver` state added
- Player death triggers automatic transition to game over screen
- All enemies and projectiles are cleared when restarting or loading
- Resources and upgrades are properly restored when loading

### 3. Save/Load System

**Save File Location:**
- Saved to: `~/.space_combat_game/save.json`
- Automatically creates directory if it doesn't exist
- JSON format for easy debugging and inspection

**What Gets Saved:**
- Player position and rotation
- Current health, shields, and energy
- Maximum health, shields, and energy
- Complete inventory (scrap metal, energy cores, minerals, tech components)
- All purchased upgrades

**How to Save:**
1. Press `ESC` to open the pause menu
2. Click "Save Game" button
3. A confirmation message will appear in the console

**How to Load:**
1. Die and reach the game over screen
2. Click "Load Saved Game" button (if a save exists)
3. Game will restore to your saved state

### 4. Restart System

**What Gets Reset:**
- All enemies are despawned
- All projectiles are cleared
- Player ship respawns at origin (0, 0, 0)
- Inventory reset to starting values:
  - Scrap Metal: 100
  - Energy Cores: 50
  - Rare Minerals: 25
  - Tech Components: 10
- All upgrades are reset to none
- Health, shields, and energy reset to 100

**How to Restart:**
1. Die and reach the game over screen
2. Click "Restart (New Game)" button
3. Game starts fresh with default values

## Technical Implementation

### New Files Added:
- `src/systems/save_load.rs` - Save/load functionality
- `PAUSE_RESTART_SAVE_FEATURES.md` - This documentation

### Modified Files:
- `src/resources/game_state.rs` - Added `GameOver` state
- `src/systems/ui.rs` - Added pause menu, game over menu, and associated systems
- `src/systems/combat.rs` - Added game over transition on player death
- `src/systems/spawning.rs` - Added restart and load game handlers
- `src/systems/mod.rs` - Added save_load module
- `src/main.rs` - Wired up all new systems and state transitions
- `Cargo.toml` - Added `serde_json` and `dirs` dependencies

### Key Systems:
- `check_pause_key` - Detects ESC key press in InGame or Paused states
- `setup_pause_menu` / `cleanup_pause_menu` - Manage pause menu UI
- `pause_menu_system` - Handle pause menu button interactions
- `setup_game_over_menu` / `cleanup_game_over_menu` - Manage game over UI
- `game_over_menu_system` - Handle game over button interactions
- `handle_restart_game` - Reset and restart the game
- `handle_load_game` - Load saved game state
- `save_game` - Save current game state to disk
- `load_game` - Load game state from disk

### State Transitions:
```
MainMenu
  └─> InGame (press Enter)
       ├─> Paused (press ESC)
       │    ├─> InGame (Resume or press ESC)
       │    └─> MainMenu (Exit)
       ├─> Upgrade (press U)
       │    └─> InGame (press ESC)
       └─> GameOver (player dies)
            ├─> InGame + Restart (Restart button)
            ├─> InGame + Load (Load Save button)
            └─> MainMenu (Main Menu button)
```

## User Experience Improvements

1. **No Lost Progress** - Save your game before risky battles
2. **Quick Testing** - Restart immediately after death to try again
3. **Flexible Gameplay** - Pause at any time for breaks
4. **Safe Progression** - Load your save if a restart isn't what you want
5. **Clear Feedback** - Console messages confirm all save/load operations

## Future Enhancements (Ideas)

- Multiple save slots
- Auto-save on upgrade purchase
- Save game statistics (enemies killed, time played, etc.)
- Quick save/load hotkeys (F5/F9)
- Save file versioning for compatibility
- Cloud save integration

## Controls Summary

- `ESC` - Pause game (during gameplay) or Resume (when paused)
- `U` - Open upgrade menu (during gameplay)
- Mouse buttons and number keys work in all menus

## Troubleshooting

**Save file not found:**
- The save file is only created after you save at least once
- Check `~/.space_combat_game/save.json` exists

**Load button not appearing:**
- The load button only appears if a save file exists
- Save at least once to see this option

**Game state issues after loading:**
- If you experience issues, try restarting instead
- Check console for error messages
- The game will fall back to restart if load fails

## Console Messages

The game logs important events to help debug issues:
- `[UI System] Game paused`
- `[UI System] Game saved successfully!`
- `[UI System] Restarting game...`
- `[UI System] Loading saved game...`
- `[Combat System] Player died! Game Over`
- `[Spawning System] Game restarted successfully`
- `[Save/Load System] Game saved to [path]`

All systems use descriptive prefixes to make debugging easier.

