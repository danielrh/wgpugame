#![macro_use]

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
    use crate::game;

    #[test]
    fn any_with_game_state() {
        let game_state = game::GameState::GameOver;
        assert!(any!(game_state, game::GameState::GameOver));

        assert!(!any!(game_state, game::GameState::MainMenu));
        assert!(!any!(game_state, game::GameState::Playing));
        assert!(!any!(game_state, game::GameState::Quiting));

        assert!(any!(
            game_state,
            game::GameState::MainMenu,
            game::GameState::Playing,
            game::GameState::GameOver,
            game::GameState::Quiting,
        ));
    }
}
