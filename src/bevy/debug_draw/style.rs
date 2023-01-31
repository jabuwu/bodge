use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct DebugDrawStyle {
    pub visible: bool,
    pub color: Color,
    pub outline: bool,
    pub thickness: f32,
    pub segments: u8,
    pub depth: f32,
}

impl Default for DebugDrawStyle {
    fn default() -> DebugDrawStyle {
        DebugDrawStyle {
            visible: true,
            color: Color::BLACK,
            outline: false,
            thickness: 1.,
            segments: 64,
            depth: 0.,
        }
    }
}

impl DebugDrawStyle {
    pub fn new(color: Color) -> DebugDrawStyle {
        DebugDrawStyle {
            color,
            ..Default::default()
        }
    }

    pub fn new_outline(color: Color) -> DebugDrawStyle {
        DebugDrawStyle {
            color,
            outline: true,
            ..Default::default()
        }
    }

    pub fn with_visible(&self, visible: bool) -> DebugDrawStyle {
        DebugDrawStyle { visible, ..*self }
    }

    pub fn with_color(&self, color: Color) -> DebugDrawStyle {
        DebugDrawStyle { color, ..*self }
    }

    pub fn with_outline(&self, outline: bool) -> DebugDrawStyle {
        DebugDrawStyle { outline, ..*self }
    }

    pub fn with_thickness(&self, thickness: f32) -> DebugDrawStyle {
        DebugDrawStyle { thickness, ..*self }
    }

    pub fn with_segments(&self, segments: u8) -> DebugDrawStyle {
        DebugDrawStyle { segments, ..*self }
    }

    pub fn with_depth(&self, depth: f32) -> DebugDrawStyle {
        DebugDrawStyle { depth, ..*self }
    }
}
