use pyo3::prelude::*;

#[pymodule]
mod meta_pytris {
    use crate::modifier::Modifier;
    use crate::GameType;
    use crate::GameplayState;
    use crate::Input;
    use crate::Random;
    use pyo3::prelude::*;

    #[pyclass]
    struct State {
        name: String,
        gameplay: GameplayState<{ Modifier::empty() }>,
    }

    #[pymethods]
    impl State {
        #[new]
        fn new(name: String, frame_counter: u8, level: u8, height: u8) -> Self {
            let random = Random::new();
            let previous_input = Input::empty();
            let game_type = GameType::A;

            let gameplay = GameplayState::<{ Modifier::empty() }>::new_with_modifier(
                &random,
                frame_counter,
                previous_input,
                game_type,
                level,
                height,
            );
            State { name, gameplay }
        }

        fn get_name(&self) -> String {
            format!("my name is {}", self.name)
        }

        fn step(&mut self) {
            println!("x = {}", self.gameplay.current_piece_x);
            println!("y = {}", self.gameplay.current_piece_y);
            println!("playstate = {:#?}", self.gameplay.play_state);
            self.gameplay.step(Input::empty());
        }

        fn playfield(&mut self) -> [u8; 256] {
            self.gameplay.vanilla_tiles()
        }

        #[getter]
        fn dead(&self) -> bool {
            self.gameplay.dead
        }
    }
}
