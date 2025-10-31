# Cyberpunk UI Visual Design Guide

## Quick Reference for the New UI Aesthetic

### Color Scheme at a Glance

| Element | Color | Hex Equivalent | Usage |
|---------|-------|----------------|-------|
| Health/Hull | Neon Red | `#FF1A4D` | Hull integrity bars |
| Shields | Neon Cyan | `#00E6FF` | Shield bars, pause menu |
| Energy | Neon Green | `#00FF4D` | Power bars, success states |
| Weapons/Heat | Neon Orange | `#FF6600` | Weapon heat, warnings |
| Critical/Danger | Pure Red | `#FF0000` | Game over, critical states |
| Minerals | Neon Magenta | `#FF00CC` | Resource display |
| Controls | Electric Purple | `#9900FF` | Control panels, UI accents |
| Warnings | Neon Yellow | `#FFF200` | Notifications, reload states |

### Typography Patterns

```
Main Titles:        "◢ SPACE COMBAT ◣"
Section Headers:    "// SYSTEM PAUSED //"
Commands:           ">> PRESS [ENTER] TO INITIALIZE <<"
Labels:             "// HULL INTEGRITY"
Values:             "[ AMMO: 10/50 ]"
Resources:          "[■] SCRAP: 150"
```

### UI Element Styles

#### Progress Bars
```
┌─────────────────────────────┐  ← Thick neon border (3px)
│███████████░░░░░░░░░░░  75% │  ← Dark background (α=0.7)
└─────────────────────────────┘  ← Filled with solid neon color
```

#### Buttons
```
┌───────────────────────────────┐
│  >> BUTTON TEXT HERE         │  ← Pulsing border
└───────────────────────────────┘  ← Dark semi-transparent bg
```

#### Panels
```
╔═══════════════════════════════╗  ← Double-line visual effect
║  // PANEL HEADER //           ║  ← Pulsing neon border
║                               ║  ← Very dark bg (α=0.9)
║  Content goes here            ║
╚═══════════════════════════════╝
```

### Animation Timings

| Effect | Speed | Pattern | Usage |
|--------|-------|---------|-------|
| Border Pulse | 0.8-2.0 Hz | Sine wave | Panels, important elements |
| Text Glitch | Every 1-5s | Random offset | Titles, headers |
| Warning Pulse | 3.0 Hz | Fast sine | Low health warning |
| Fade In | 0.3s | Linear | Panel appearance |

### State-Based Coloring

#### Health Bar Colors
- **100-75%**: Solid neon red
- **75-25%**: Same red (no change)
- **<25%**: Pulsing red border for warning

#### Ammo Display Colors
- **Full/High**: White/cyan
- **<25%**: Warning yellow
- **Empty**: Danger red

#### Shield States
- **Active**: Pulsing cyan border (slow)
- **Recharging**: Cyan fill animation
- **Depleted**: Dim/hidden

### In-Game HUD Layout

```
┌─ Screen ──────────────────────────────────────┐
│ // HULL INTEGRITY                             │
│ ┌────────────── 85% ─────────────┐            │
│ │███████████████████░░░░░         │            │
│ └─────────────────────────────────┘            │
│                                                │
│ // SHIELD MATRIX                               │
│ ┌────────────── 100% ────────────┐  (pulsing) │
│ │█████████████████████████████   │            │
│ └─────────────────────────────────┘            │
│                                                │
│ // POWER CORE                                  │
│ ┌────────────── 60% ─────────────┐            │
│ │█████████████████░░░░░░░░░░░     │            │
│ └─────────────────────────────────┘            │
│                                                │
│ >> WEAPON: LASER                               │
│ // HEAT LEVEL                                  │
│ ┌────────────────────────────────┐            │
│ [ AMMO: 50/200 ]                               │
│                                                │
│ // INVENTORY                                   │
│ [■] SCRAP: 150                                 │
│ [●] CORES: 45                                  │
│ [◆] MINERALS: 20                               │
│ [▲] TECH: 10                                   │
│                                                │
│ >> UPGRADES AVAILABLE (U)  (pulsing)          │
│                                                │
│                         Center of screen       │
│                           Crosshair            │
│                                                │
│                                                │
│                                                │
└────────────────────────────────────────────────┘
```

### Menu Hierarchy

```
MAIN MENU
├─ Title (glitching, cyan)
│  "◢ SPACE COMBAT ◣"
├─ Subtitle (pulsing, magenta)
│  "// NEURAL INTERFACE ACTIVE"
├─ Start Prompt (pulsing, green)
│  ">> PRESS [ENTER] TO INITIALIZE <<"
└─ Control Panel (purple border)
   ├─ Header: "// CONTROL MATRIX //"
   └─ Control list

PAUSE MENU
├─ Panel (pulsing cyan border)
│  ├─ Title (glitching)
│  │  "// SYSTEM PAUSED //"
│  ├─ Button: Resume (green)
│  ├─ Button: Save (cyan)
│  └─ Button: Exit (orange)

GAME OVER
├─ Panel (pulsing red border)
│  ├─ Title (intense glitch, red)
│  │  "// CRITICAL FAILURE //"
│  ├─ Status text
│  │  "HULL INTEGRITY: 0%"
│  ├─ Button: Respawn (green)
│  ├─ Button: Restore (cyan, if save exists)
│  └─ Button: Exit (orange)
```

### Icon Legend

| Icon | Meaning | Usage |
|------|---------|-------|
| `//` | Comment/Label | Section headers |
| `>>` | Command/Action | Prompts, active elements |
| `[ ]` | Value container | Ammo, bracketed info |
| `◢ ◣` | Decorative | Title flourishes |
| `[■]` | Solid square | Scrap metal resource |
| `[●]` | Circle | Energy cores resource |
| `[◆]` | Diamond | Rare minerals resource |
| `[▲]` | Triangle | Tech components resource |

### Responsive Behaviors

#### Button Interactions
1. **Idle**: Dark background, colored border (pulsing)
2. **Hover**: Slightly brighter background
3. **Pressed**: Brief flash, then action
4. **Disabled**: Dimmed, gray colors

#### Panel Entrance
1. Fade in background (0.3s)
2. Border appears with pulse animation
3. Content fades in slightly after panel

#### Text Effects
- **Glitch**: Random horizontal offset + color shift
- **Pulse**: Text color or border oscillates via sine wave
- **Fade**: Opacity transition for smooth appearance

### Accessibility Notes

- **High Contrast**: Neon colors on dark backgrounds ensure readability
- **Clear Hierarchy**: Important info uses larger text and brighter colors
- **Animation Control**: Can be disabled for users sensitive to motion
- **Color Blind Modes**: Future enhancement to adjust palette
- **Text Size**: All text sized for readability at 1080p+

### Technical Implementation Notes

#### Layering (Z-Index)
- Background: -10
- Base UI: 0
- Panels: 10
- Overlays: 20
- Tooltips: 30
- Modals: 40
- Reticule: 100

#### Border Radius
- Most elements: 2px (sharp, cyberpunk aesthetic)
- Health bars: 2px
- Buttons: 2px
- Panels: 4px for slight softness

#### Opacity Levels
- Panel backgrounds: 0.85-0.95 (very opaque)
- Bar backgrounds: 0.7 (more transparent to show glow)
- Overlays: 0.9 (slightly transparent for context)

---

## Development Quick Reference

### Adding a New Neon Element
```rust
use crate::systems::ui_theme::{colors, borders, PanelConfig};
use crate::systems::ui_animations::PulseAnimation;

// Create a pulsing panel
parent.spawn((
    PanelConfig::new()
        .with_border_color(colors::NEON_CYAN)
        .dark()
        .build(),
    PulseAnimation::new(1.5, colors::NEON_CYAN).with_range(0.7, 1.0),
));

// Add glitching text
parent.spawn((
    TextBundle::from_section(
        ">> YOUR TEXT HERE",
        TextStyle {
            font_size: 24.0,
            color: colors::NEON_GREEN,
            ..default()
        },
    ),
    GlitchEffect {
        interval: 3.0,
        duration: 0.1,
        intensity: 0.8,
        ..default()
    },
));
```

### Color Selection Guide
- **Success/Go**: NEON_GREEN
- **Info/Primary**: NEON_CYAN
- **Warning**: NEON_YELLOW or NEON_ORANGE
- **Error/Danger**: DANGER_COLOR (pure red)
- **Special/Accent**: NEON_MAGENTA or ELECTRIC_PURPLE
- **Neutral**: Gray tones (0.7-0.9)

---

**This guide should be used as a reference when extending or modifying the UI to maintain visual consistency.**

