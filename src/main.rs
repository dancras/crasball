use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode};
use ggez::event::{self, EventHandler};
use nalgebra::{Point2, Vector2};

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
                position: Point2::new(200.0, 200.0),
                direction: Vector2::new(1.0, 1.0)
            }
        }
    }
}

// // Returns 1 if the lines intersect, otherwise 0. In addition, if the lines 
// // intersect the intersection point may be stored in the floats i_x and i_y.
// char get_line_intersection(float p0_x, float p0_y, float p1_x, float p1_y, 
//     float p2_x, float p2_y, float p3_x, float p3_y, float *i_x, float *i_y)
// {
//     float s1_x, s1_y, s2_x, s2_y;
//     s1_x = p1_x - p0_x;     s1_y = p1_y - p0_y;
//     s2_x = p3_x - p2_x;     s2_y = p3_y - p2_y;

//     float s, t;
//     s = (-s1_y * (p0_x - p2_x) + s1_x * (p0_y - p2_y)) / (-s2_x * s1_y + s1_x * s2_y);
//     t = ( s2_x * (p0_y - p2_y) - s2_y * (p0_x - p2_x)) / (-s2_x * s1_y + s1_x * s2_y);

//     if (s >= 0 && s <= 1 && t >= 0 && t <= 1)
//     {
//         // Collision detected
//         if (i_x != NULL)
//             *i_x = p0_x + (t * s1_x);
//         if (i_y != NULL)
//             *i_y = p0_y + (t * s1_y);
//         return 1;
//     }

//     return 0; // No collision
// }

fn find_intersection(
    a1: Point2<f32>, a2: Point2<f32>, b1: Point2<f32>, b2: Point2<f32>
) -> (bool, Point2<f32>)
{
    let sa = *(a2 - a1);
    let sb = *(b2 - b1);

    let d1 = *(a1 - b1);

    let s = (-sa.y * d1.x + sa.x * d1.y) / (-sb.x * sa.y + sa.x * sb.y);
    let t = (sb.x * d1.y - sb.y * d1.x) / (-sb.x * sa.y + sa.x * sb.y);

    if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
        (true, a1 + ((a2 - a1) * t))
    } else {
        (false, Point2::origin())
    }
}

impl EventHandler for CrasballGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let newpos: Point2<f32>;

        let (intersects, point) = find_intersection(
            self.ball.position, self.ball.position + self.ball.direction,
            Point2::new(0.0, 600.0), Point2::new(800.0, 600.0)
        );

        if intersects {
            newpos = Point2::new(200.0, 200.0);
        } else {
            newpos = self.ball.position + self.ball.direction;
        }

        self.ball = Ball {
            position: newpos,
            ..self.ball
        };

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