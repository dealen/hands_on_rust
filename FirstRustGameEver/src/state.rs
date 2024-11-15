use bracket_lib::prelude::*;
use crate::game_mode::GameMode;
use crate::obstacle::Obstacle;
use crate::player::Player;

const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
const FRAME_DURATION : f32 = 75.0;

pub struct State {
    pub player: Player,
    pub obstacle: Obstacle,
    pub frame_time: f32,
    pub mode: GameMode,
    pub score: i32
}
// page 51

impl State {
    pub fn new() -> State {
        State {
            player: Player::new(5, 25),
            obstacle: Obstacle::new(SCREEN_WIDTH, 0, SCREEN_HEIGHT),
            frame_time: 0.0,
            mode: GameMode::Menu,
            score: 0
        }
    }

    pub fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);
        ctx.print(0,0, "Press SPACE to flap!");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(
                self.player.x + SCREEN_WIDTH, self.score, SCREEN_WIDTH
            );
        }

        if self.player.y > SCREEN_HEIGHT ||
            self.obstacle.hit_obstacle(&self.player)
        {
            self.mode = GameMode::End;
        }
    }

    pub fn restart (&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0, SCREEN_HEIGHT);

        self.mode = GameMode::Playing;
    }

    pub fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Minimal Bracket Roguelike");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    pub fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points!", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}