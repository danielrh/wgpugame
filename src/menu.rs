use super::game::UNBOUNDED_F32;
use super::state::Text;
pub struct Menu {
    pub title_text: Text,
    pub play_button: Text,
    pub quit_button: Text,
    pub win_text: Text,
}
impl Default for Menu {
    fn default() -> Self {
        Menu {
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
            win_text: Text {
                position: (160.0, 120.0).into(),
                bounds: (320.0, UNBOUNDED_F32).into(),
                size: 32.0,
                centered: true,
                ..Default::default()
            },
        }
    }
}
