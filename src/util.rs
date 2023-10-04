#![macro_use]

use crate::state;

pub const PLAYER_SPEED: f32 = 0.05;

pub fn size_of_slice<T: Sized>(slice: &[T]) -> usize {
    std::mem::size_of::<T>() * slice.len()
}

#[macro_export]
macro_rules! any {
    ($x:expr, $($y:expr),+ $(,)?) => {
        {
            false $(|| $x == $y)+
        }
    };
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use crate::state;

    #[test]
    fn any_with_game_state() {
        let game_state = state::GameState::GameOver;
        assert!(any!(game_state, state::GameState::GameOver));

        assert!(!any!(game_state, state::GameState::MainMenu));
        assert!(!any!(game_state, state::GameState::Playing));
        assert!(!any!(game_state, state::GameState::Quiting));

        assert!(any!(
            game_state,
            state::GameState::MainMenu,
            state::GameState::Playing,
            state::GameState::GameOver,
            state::GameState::Quiting,
        ));
    }
}
