use super::menu::Menu;
use super::render::Render;
use super::state::Text;
use super::system::System;
use crate::render::{draw_text, Color, QuadBufferBuilder};
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
    pub player1_score: Text,
    pub game_state: GameState,
    pub menu: Menu,
}

impl State {
    pub fn new(render: &Render) -> State {
        State {
            player1: Player {
                position: (0.0, 0.0).into(),
                size: (0.4, 0.4).into(),
                score: 0,
                visible: true,
            },
 
            player1_score: Text {
                position: (render.width() * 0.5, 20.0).into(),
                color: Color::new(255, 255, 0),
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

        
        QuadBufferBuilder::new()
            .push_quad2d(
                self.player1.position,
                self.player1.size,
                Color::new(255, 255, 0),
            )
            
    }

    pub fn resize(&mut self, width: f32, _height: f32) {
        self.player1_score.position = (width * 0.25, 20.0).into();
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


        // Copy current score to players
        state.player1_score.text = state.player1.score.to_string();


        if state.player1.score > 200 {
            log::info!("win!");
            state.game_state = GameState::GameOver;
        }
    }
}
