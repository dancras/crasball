use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode};
use ggez::event::{self, EventHandler};
use ggez::input::mouse::{self, MouseButton};
use ggez::timer;
use nalgebra::{Point2, Vector2};
use rand::{random};
use std::f32::consts::{PI};
use crasball::game::{Ball,GameState, Edge};

const DESIRED_FPS: u32 = 60;
const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("dancras/crasball", "dancras")
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = CrasballGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

enum CursorDirection {
    Horizontal,
    Vertical
}

struct CrasballGame {
    cursor_direction: CursorDirection,
    state: GameState
}

impl CrasballGame {
    pub fn new(ctx: &mut Context) -> CrasballGame {
        // Load/create resources such as images here.
        mouse::set_cursor_type(ctx, mouse::MouseCursor::NsResize);

        CrasballGame {
            cursor_direction: CursorDirection::Vertical,
            state: GameState {
                balls: vec![
                    Ball {
                        radius: 20.0,
                        position: random_ball_position(20.0),
                        movement: random_ball_movement(100.0)
                    },
                    Ball {
                        radius: 20.0,
                        position: Point2::new(350.0, 255.0),
                        movement: Vector2::new(
                            (5000.0 as f32).sqrt(),
                            (5000.0 as f32).sqrt()
                        )
                    },
                    Ball {
                        radius: 20.0,
                        position: random_ball_position(20.0),
                        movement: random_ball_movement(100.0)
                    },
                    Ball {
                        radius: 20.0,
                        position: random_ball_position(20.0),
                        movement: random_ball_movement(100.0)
                    }
                ],
                edges: vec![
                    Edge {
                        a: Point2::new(0.0, 0.0),
                        b: Point2::new(800.0, 0.0),
                        n: Vector2::new(0.0, 1.0)
                    },
                    Edge {
                        a: Point2::new(0.0, 600.0),
                        b: Point2::new(800.0, 600.0),
                        n: Vector2::new(0.0, -1.0)
                    },
                    Edge {
                        a: Point2::new(800.0, 0.0),
                        b: Point2::new(800.0, 600.0),
                        n: Vector2::new(-1.0, 0.0)
                    },
                    Edge {
                        a: Point2::new(0.0, 0.0),
                        b: Point2::new(0.0, 600.0),
                        n: Vector2::new(1.0, 0.0)
                    },

                    // Test edge
                    Edge {
                        a: Point2::new(390.0, 300.0),
                        b: Point2::new(410.0, 300.0),
                        n: Vector2::new(0.0, -1.0)
                    },
                    Edge {
                        a: Point2::new(410.0, 300.0),
                        b: Point2::new(410.0, 600.0),
                        n: Vector2::new(1.0, 0.0)
                    },
                    Edge {
                        a: Point2::new(390.0, 300.0),
                        b: Point2::new(390.0, 600.0),
                        n: Vector2::new(-1.0, 0.0)
                    }
                ]
            }
        }
    }
}

fn random_ball_position(radius: f32) -> Point2<f32> {

    Point2::new(
        radius + random::<f32>() * (SCREEN_SIZE.0 - 2.0 * radius),
        radius + random::<f32>() * (SCREEN_SIZE.1 - 2.0 * radius)
    )
}

fn random_ball_movement(velocity: f32) -> Vector2<f32> {
    let angle = 2.0 * PI * random::<f32>();
    let base_movement = Vector2::new(angle.sin(), angle.cos());
    let base_magnitude = base_movement.norm();

    base_movement * (velocity / base_magnitude)
}

impl EventHandler for CrasballGame {

    fn mouse_button_down_event(
        &mut self, ctx: &mut Context, button: MouseButton, _x: f32, _y: f32
    ) {
        if let MouseButton::Right = button {
            if let CursorDirection::Vertical = self.cursor_direction {
                self.cursor_direction = CursorDirection::Horizontal;
                mouse::set_cursor_type(ctx, mouse::MouseCursor::NsResize);
            } else {
                self.cursor_direction = CursorDirection::Vertical;
                mouse::set_cursor_type(ctx, mouse::MouseCursor::EwResize);
            }
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let delta = 1.0 / (DESIRED_FPS as f32);
            self.state.update(delta);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        self.state.draw(ctx)?;

        graphics::present(ctx)
    }
}
