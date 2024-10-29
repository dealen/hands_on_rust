use bracket_lib::prelude::*;

fn main() {
    println!("Hello, world!");
}


struct State {}

// Implementing the GameState trait for our State struct
// This trait is provided by the bracket-lib crate
// It requires us to implement a tick function that will be called every frame
// We will use this function to update our game state and render the game
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket World");
    }
}


// page 51