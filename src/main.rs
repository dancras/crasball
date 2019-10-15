use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode};
use ggez::event::{self, EventHandler};
use nalgebra::{Point2, Vector2};
use nalgebra::geometry::{Translation2};

const SCREEN_SIZE: (f32, f32) = (400.0, 400.0);

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

struct Ball {
    radius: f32,
    position: Point2<f32>,
    direction: Vector2<f32>
}

struct CrasballGame {
    ball: Ball
}

impl CrasballGame {
    pub fn new(_ctx: &mut Context) -> CrasballGame {
        // Load/create resources such as images here.
        CrasballGame {
            ball: Ball {
                radius: 50.0,
                position: Point2::new(0.0, 200.0),
                direction: Vector2::new(1.0, 1.0)
            }
        }
    }
}

impl EventHandler for CrasballGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        let t = Translation2::from(self.ball.direction);

        self.ball = Ball {
            position: t.transform_point(&self.ball.position),
            ..self.ball
        };
        // self.pos_x = self.pos_x % SCREEN_SIZE.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.ball.position,
            self.ball.radius,
            0.1,
            graphics::BLACK,
        )?;

        graphics::draw(ctx, &circle, (Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)
    }
}