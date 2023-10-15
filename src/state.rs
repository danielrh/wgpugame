use crate::render::Color;
#[derive(Debug)]
pub struct Text {
    pub position: cgmath::Vector2<f32>,
    pub bounds: cgmath::Vector2<f32>,
    pub color: Color,
    pub text: String,
    pub size: f32,
    pub visible: bool,
    pub focused: bool,
    pub centered: bool,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0).into(),
            bounds: (super::game::UNBOUNDED_F32, super::game::UNBOUNDED_F32).into(),
            color: Color::new(255, 255, 255),
            text: String::new(),
            size: 16.0,
            visible: false,
            focused: false,
            centered: false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Event {
    ButtonPressed,
    FocusChanged,
    #[allow(dead_code)]
    Score(u32),
    Resize(f32, f32),
}
