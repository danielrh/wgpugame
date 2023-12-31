use winit::event::{ElementState, VirtualKeyCode};

#[derive(Debug, Default)]
pub struct Input {
    pub p1_up_pressed: bool,
    pub p1_down_pressed: bool,
    pub p1_left_pressed: bool,
    pub p1_right_pressed: bool,
    pub p2_up_pressed: bool,
    pub p2_down_pressed: bool,
    pub p2_left_pressed: bool,
    pub p2_right_pressed: bool,
    pub enter_pressed: bool,
    pub mouse_pressed: [bool; 16],
    pub mouse_click: [bool; 16],
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub mouse_move_x: f64,
    pub mouse_move_y: f64,
}

impl Input {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn update(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        let pressed = state == ElementState::Pressed;
        match key {
            VirtualKeyCode::Up => {
                self.p2_up_pressed = pressed;
                true
            }
            VirtualKeyCode::Left => {
                self.p2_left_pressed = pressed;
                true
            }
            VirtualKeyCode::Right => {
                self.p2_right_pressed = pressed;
                true
            }
            VirtualKeyCode::Down => {
                self.p2_down_pressed = pressed;
                true
            }
            VirtualKeyCode::W => {
                self.p1_up_pressed = pressed;
                true
            }
            VirtualKeyCode::A => {
                self.p1_left_pressed = pressed;
                true
            }
            VirtualKeyCode::D => {
                self.p1_right_pressed = pressed;
                true
            }
            VirtualKeyCode::S => {
                self.p1_down_pressed = pressed;
                true
            }
            VirtualKeyCode::Return => {
                self.enter_pressed = pressed;
                true
            }
            _ => false,
        }
    }

    pub fn ui_up_pressed(&self) -> bool {
        self.p1_up_pressed || self.p2_up_pressed
    }

    pub fn ui_down_pressed(&self) -> bool {
        self.p1_down_pressed || self.p2_down_pressed
    }
}
