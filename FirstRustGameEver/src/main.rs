mod game_mode;
mod state;
mod player;
mod obstacle;

use bracket_lib::prelude::*;
use crate::game_mode::GameMode;
use crate::state::State;

// Implementing the GameState trait for our State struct
// This trait is provided by the bracket-lib crate
// It requires us to implement a tick function that will be called every frame
// We will use this function to update our game state and render the game
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
        // ctx.cls();
        // ctx.print(2, 2, "Hello, Bracket World");
    }
}

fn main() ->BError {
    let context = BTermBuilder::vga80x50()
        .with_title("Bracket Terminal Example - Hello Minimal Bracket World")
        .build()?;

    main_loop(context, State::new())
}

