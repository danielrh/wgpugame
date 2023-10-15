use super::render::Render;
use super::state::Text;
use super::system::System;
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
    pub title_text: Text,
    pub play_button: Text,
    pub quit_button: Text,
    pub player1_score: Text,
    pub player2_score: Text,
    pub win_text: Text,
    pub game_state: GameState,
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
            title_text: Text {
                position: (20.0, 20.0).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                text: String::from("PONG"),
                size: 64.0,
                ..Default::default()
            },
            play_button: Text {
                position: (40.0, 100.0).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                text: String::from("Play"),
                size: 32.0,
                centered: false,
                ..Default::default()
            },
            quit_button: Text {
                position: (40.0, 160.0).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                text: String::from("Quit"),
                size: 32.0,
                ..Default::default()
            },
            player1_score: Text {
                position: (render.width() * 0.25, 20.0).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                text: String::from("0"),
                size: 32.0,
                ..Default::default()
            },
            player2_score: Text {
                position: (render.width() * 0.75, 20.0).into(),
                color: (1.0, 1.0, 1.0, 1.0).into(),
                text: String::from("0"),
                size: 32.0,
                ..Default::default()
            },
            win_text: Text {
                position: (render.width() * 0.5, render.height() * 0.5).into(),
                bounds: (render.width(), UNBOUNDED_F32).into(),
                size: 32.0,
                centered: true,
                ..Default::default()
            },
            game_state: GameState::MainMenu,
        }
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

        // normalize players
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

        if state.player1.score > 2 || state.player2.score > 2 {
            log::info!("Gameover");
            state.game_state = GameState::GameOver;
        }
    }
}
