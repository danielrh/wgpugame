use super::menu::Menu;
use super::render::Render;
use super::state::Text;
use super::system::System;
use crate::render::{draw_text, QuadBufferBuilder, Color};
pub const UNBOUNDED_F32: f32 = std::f32::INFINITY;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GameState {
    MainMenu,
    Playing,
    GameOver,
    Quiting,
}

pub struct State {
    pub player1: Player,
    pub player2: Player,
    pub player1_score: Text,
    pub player2_score: Text,
    pub game_state: GameState,
    pub menu: Menu,
}

impl State {
    pub fn new(render: &Render) -> State {
        State {
            player1: Player {
                position: (-0.8, 0.0).into(),
                size: (0.05, 0.4).into(),
                score: 0,
                visible: true,
            },
            player2: Player {
                position: (0.8, 0.0).into(),
                size: (0.05, 0.4).into(),
                score: 0,
                visible: true,
            },
            player1_score: Text {
                position: (render.width() * 0.25, 20.0).into(),
                color: Color::new(255, 255, 0),
                text: String::from("0"),
                size: 32.0,
                ..Default::default()
            },
            player2_score: Text {
                position: (render.width() * 0.75, 20.0).into(),
                color: Color::new(0, 255, 255),
                text: String::from("0"),
                size: 32.0,
                ..Default::default()
            },
            game_state: GameState::MainMenu,
            menu: Menu::default(),
        }
    }
    pub fn draw(&self, glyph_brush: &mut wgpu_glyph::GlyphBrush<()>) -> QuadBufferBuilder {
        if self.player1_score.visible {
            draw_text(&self.player1_score, glyph_brush);
        }
        if self.player2_score.visible {
            draw_text(&self.player2_score, glyph_brush);
        }
        QuadBufferBuilder::new()
            .push_quad2d(self.player1.position, self.player1.size, Color::new(255,255,0))
            .push_quad2d(self.player2.position, self.player2.size, Color::new(0,255,255))
    }

    pub fn resize(&mut self, width: f32, _height: f32) {
        self.player1_score.position = (width * 0.25, 20.0).into();
        self.player2_score.position = (width * 0.75, 20.0).into()
    }
}

pub const PLAYER_SPEED: f32 = 0.05;

#[derive(Debug)]
pub struct Player {
    pub position: cgmath::Vector2<f32>,
    pub size: cgmath::Vector2<f32>,
    pub score: u32,
    pub visible: bool,
}


pub struct PlaySystem;
impl System for PlaySystem {
    fn update_state(
        &self,
        input: &crate::input::Input,
        state: &mut State,
        _events: &mut Vec<crate::state::Event>,
    ) {
        // move the players
        if input.p1_up_pressed {
            state.player1.position.y += PLAYER_SPEED;
        }
        if input.p1_down_pressed {
            state.player1.position.y -= PLAYER_SPEED;
        }
        if input.p2_up_pressed {
            state.player2.position.y += PLAYER_SPEED;
        }
        if input.p2_down_pressed {
            state.player2.position.y -= PLAYER_SPEED;
        }

        if input.p1_right_pressed {
            state.player1.position.x += PLAYER_SPEED;
        }
        if input.p1_left_pressed {
            state.player1.position.x -= PLAYER_SPEED;
        }
        if input.p2_right_pressed {
            state.player2.position.x += PLAYER_SPEED;
        }
        if input.p2_left_pressed {
            state.player2.position.x -= PLAYER_SPEED;
        }

        // Never allow players to go past upper or lower edge of screen.
        if state.player1.position.y > 1.0 - state.player1.size.y * 0.5 {
            state.player1.position.y = 1.0 - state.player1.size.y * 0.5;
        } else if state.player1.position.y < state.player1.size.y * 0.5 - 1.0 {
            state.player1.position.y = state.player1.size.y * 0.5 - 1.0;
        }
        if state.player2.position.y > 1.0 - state.player1.size.y * 0.5 {
            state.player2.position.y = 1.0 - state.player1.size.y * 0.5;
        } else if state.player2.position.y < state.player1.size.y * 0.5 - 1.0 {
            state.player2.position.y = state.player1.size.y * 0.5 - 1.0;
        }

        // Silly: add score a bit and subtract it like GladOS
        state.player1.score += 1;
        state.player1.score %= 3;

        // Copy current score to players
        state.player1_score.text = state.player1.score.to_string();
        state.player2_score.text = state.player2.score.to_string();

        if state.player1.score > 2 || state.player2.score > 2 {
            log::info!("Gameover");
            state.game_state = GameState::GameOver;
        }
    }
}
