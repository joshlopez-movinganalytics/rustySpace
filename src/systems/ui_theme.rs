use bevy::prelude::*;

/// Cyberpunk/Neon color palette
pub mod colors {
    use bevy::prelude::*;
    
    // Primary neon colors
    pub const NEON_CYAN: Color = Color::srgb(0.0, 0.9, 1.0);
    pub const NEON_MAGENTA: Color = Color::srgb(1.0, 0.0, 0.8);
    pub const ELECTRIC_PURPLE: Color = Color::srgb(0.6, 0.0, 1.0);
    pub const HOT_PINK: Color = Color::srgb(1.0, 0.1, 0.5);
    pub const NEON_GREEN: Color = Color::srgb(0.0, 1.0, 0.3);
    pub const NEON_ORANGE: Color = Color::srgb(1.0, 0.4, 0.0);
    pub const NEON_YELLOW: Color = Color::srgb(1.0, 0.95, 0.0);
    
    // Dark backgrounds with transparency
    pub const PANEL_BG: Color = Color::srgba(0.05, 0.0, 0.15, 0.85);
    pub const PANEL_BG_DARK: Color = Color::srgba(0.02, 0.0, 0.08, 0.9);
    pub const PANEL_BG_DARKER: Color = Color::srgba(0.01, 0.0, 0.05, 0.95);
    
    // UI element colors
    pub const BUTTON_BG: Color = Color::srgba(0.1, 0.0, 0.2, 0.8);
    pub const BUTTON_BG_HOVER: Color = Color::srgba(0.15, 0.0, 0.3, 0.9);
    pub const BUTTON_BG_PRESSED: Color = Color::srgba(0.2, 0.0, 0.4, 1.0);
    pub const BUTTON_BG_DISABLED: Color = Color::srgba(0.05, 0.05, 0.05, 0.6);
    
    // Status colors
    pub const HEALTH_COLOR: Color = Color::srgb(1.0, 0.1, 0.3);
    pub const SHIELD_COLOR: Color = NEON_CYAN;
    pub const ENERGY_COLOR: Color = NEON_GREEN;
    pub const WARNING_COLOR: Color = NEON_YELLOW;
    pub const DANGER_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
    
    // Resource colors
    pub const SCRAP_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
    pub const ENERGY_CORE_COLOR: Color = NEON_CYAN;
    pub const MINERAL_COLOR: Color = NEON_MAGENTA;
    pub const TECH_COLOR: Color = NEON_ORANGE;
}

/// Typography styles with glowing effects
pub mod typography {
    use bevy::prelude::*;
    use super::colors;
    
    pub fn title_style() -> TextStyle {
        TextStyle {
            font_size: 60.0,
            color: colors::NEON_CYAN,
            ..default()
        }
    }
    
    pub fn subtitle_style() -> TextStyle {
        TextStyle {
            font_size: 36.0,
            color: colors::NEON_MAGENTA,
            ..default()
        }
    }
    
    pub fn heading_style() -> TextStyle {
        TextStyle {
            font_size: 28.0,
            color: colors::ELECTRIC_PURPLE,
            ..default()
        }
    }
    
    pub fn body_style() -> TextStyle {
        TextStyle {
            font_size: 18.0,
            color: Color::srgb(0.9, 0.9, 1.0),
            ..default()
        }
    }
    
    pub fn small_style() -> TextStyle {
        TextStyle {
            font_size: 14.0,
            color: Color::srgb(0.7, 0.7, 0.8),
            ..default()
        }
    }
    
    pub fn label_style() -> TextStyle {
        TextStyle {
            font_size: 16.0,
            color: colors::NEON_CYAN,
            ..default()
        }
    }
    
    pub fn value_style() -> TextStyle {
        TextStyle {
            font_size: 16.0,
            color: Color::WHITE,
            ..default()
        }
    }
}

/// Border styles for neon effects
pub mod borders {
    use bevy::prelude::*;
    use super::colors;
    
    pub const THIN_BORDER: UiRect = UiRect::all(Val::Px(1.0));
    pub const MEDIUM_BORDER: UiRect = UiRect::all(Val::Px(2.0));
    pub const THICK_BORDER: UiRect = UiRect::all(Val::Px(3.0));
    pub const EXTRA_THICK_BORDER: UiRect = UiRect::all(Val::Px(4.0));
    
    pub fn neon_border_color(color: Color) -> BorderColor {
        color.into()
    }
    
    pub fn default_border() -> (UiRect, BorderColor) {
        (MEDIUM_BORDER, colors::NEON_CYAN.into())
    }
    
    pub fn glow_border() -> (UiRect, BorderColor) {
        (THICK_BORDER, colors::NEON_MAGENTA.into())
    }
}

/// Panel creation helpers
pub struct PanelConfig {
    pub width: Val,
    pub height: Val,
    pub padding: UiRect,
    pub background_color: Color,
    pub border: UiRect,
    pub border_color: Color,
    pub border_radius: BorderRadius,
}

impl Default for PanelConfig {
    fn default() -> Self {
        Self {
            width: Val::Auto,
            height: Val::Auto,
            padding: UiRect::all(Val::Px(15.0)),
            background_color: colors::PANEL_BG,
            border: borders::MEDIUM_BORDER,
            border_color: colors::NEON_CYAN,
            border_radius: BorderRadius::all(Val::Px(4.0)),
        }
    }
}

impl PanelConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_width(mut self, width: Val) -> Self {
        self.width = width;
        self
    }
    
    pub fn with_height(mut self, height: Val) -> Self {
        self.height = height;
        self
    }
    
    pub fn with_padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }
    
    pub fn with_background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }
    
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }
    
    pub fn with_border_thickness(mut self, border: UiRect) -> Self {
        self.border = border;
        self
    }
    
    pub fn dark(mut self) -> Self {
        self.background_color = colors::PANEL_BG_DARK;
        self
    }
    
    pub fn darker(mut self) -> Self {
        self.background_color = colors::PANEL_BG_DARKER;
        self
    }
    
    pub fn build(self) -> NodeBundle {
        NodeBundle {
            style: Style {
                width: self.width,
                height: self.height,
                padding: self.padding,
                border: self.border,
                ..default()
            },
            background_color: self.background_color.into(),
            border_color: self.border_color.into(),
            border_radius: self.border_radius,
            ..default()
        }
    }
}

/// Button creation helpers
pub struct ButtonConfig {
    pub padding: UiRect,
    pub background_color: Color,
    pub border: UiRect,
    pub border_color: Color,
    pub border_radius: BorderRadius,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            padding: UiRect::all(Val::Px(15.0)),
            background_color: colors::BUTTON_BG,
            border: borders::MEDIUM_BORDER,
            border_color: colors::NEON_CYAN,
            border_radius: BorderRadius::all(Val::Px(2.0)),
        }
    }
}

impl ButtonConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_padding(mut self, padding: UiRect) -> Self {
        self.padding = padding;
        self
    }
    
    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }
    
    pub fn build(self) -> ButtonBundle {
        ButtonBundle {
            style: Style {
                padding: self.padding,
                border: self.border,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: self.background_color.into(),
            border_color: self.border_color.into(),
            border_radius: self.border_radius,
            ..default()
        }
    }
}

/// Helper for creating holographic bar containers
pub fn create_bar_container(width: f32) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Px(width),
            height: Val::Px(20.0),
            border: borders::MEDIUM_BORDER,
            ..default()
        },
        background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
        border_color: colors::NEON_CYAN.into(),
        border_radius: BorderRadius::all(Val::Px(2.0)),
        ..default()
    }
}

/// Helper for creating holographic bar fill
pub fn create_bar_fill(color: Color, percentage: f32) -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(percentage * 100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: color.into(),
        ..default()
    }
}

/// Animation timing constants
pub mod timing {
    pub const PULSE_SPEED: f32 = 2.0; // Cycles per second
    pub const GLOW_SPEED: f32 = 1.5;
    pub const SCANLINE_SPEED: f32 = 100.0; // Pixels per second
    pub const GLITCH_INTERVAL: f32 = 3.0; // Seconds between glitches
    pub const FADE_DURATION: f32 = 0.3; // Seconds
    pub const SLIDE_DURATION: f32 = 0.4; // Seconds
}

/// Z-index layers for UI depth
pub mod layers {
    use bevy::prelude::*;
    
    pub const BACKGROUND: ZIndex = ZIndex::Global(-10);
    pub const BASE: ZIndex = ZIndex::Global(0);
    pub const PANELS: ZIndex = ZIndex::Global(10);
    pub const OVERLAYS: ZIndex = ZIndex::Global(20);
    pub const TOOLTIPS: ZIndex = ZIndex::Global(30);
    pub const MODALS: ZIndex = ZIndex::Global(40);
    pub const RETICULE: ZIndex = ZIndex::Global(100);
}

/// Helper to create glowing text with shadow effect
pub fn create_glowing_text(text: &str, style: TextStyle) -> TextBundle {
    TextBundle::from_section(text, style)
}

/// Helper to create uppercase text (for cyberpunk aesthetic)
pub fn uppercase(text: &str) -> String {
    text.to_uppercase()
}

