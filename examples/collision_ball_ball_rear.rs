use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics;
use ggez::conf::{WindowMode};
use ggez::event::{self, EventHandler};
use nalgebra::{Point2, Vector2};
use crasball::game::{Ball,GameState};

const SCREEN_SIZE: (f32, f32) = (400.0, 400.0);

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("dancras/crasball/example", "dancras")
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("aieee, could not create ggez context!");

    let mut example = CrasballGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut example) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct CrasballGame {
    state: GameState
}

impl CrasballGame {
    pub fn new(_ctx: &mut Context) -> CrasballGame {
        // Load/create resources such as images here.
        CrasballGame {
            state: GameState {
                balls: vec![
                    Ball {
                        radius: 20.0,
                        position: Point2::new(100.0, 200.0),
                        movement: Vector2::new(100.0, 0.0)
                    },
                    Ball {
                        radius: 20.0,
                        position: Point2::new(141.0, 200.0),
                        movement: Vector2::new(
                            (5000.0 as f32).sqrt(),
                            (5000.0 as f32).sqrt()
                        )
                    }
                ],
                walls: Vec::new()
            }
        }
    }
}

impl EventHandler for CrasballGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.state.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        self.state.draw(ctx)?;

        graphics::present(ctx)
    }
}
