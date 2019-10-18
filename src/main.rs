use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode};
use ggez::event::{self, EventHandler};
use nalgebra::{Point2, Vector2};
use rand::{random};
use std::f32::consts::{PI};

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
    movement: Vector2<f32>
}

struct CrasballGame {
    balls: Vec<Ball>,
    walls: Vec<Wall>
}

struct Wall {
    a: Point2<f32>,
    b: Point2<f32>,
    n: Vector2<f32>
}

impl CrasballGame {
    pub fn new(_ctx: &mut Context) -> CrasballGame {
        // Load/create resources such as images here.
        CrasballGame {
            balls: vec![
                Ball {
                    radius: 20.0,
                    position: Point2::new(200.0, 200.0),
                    movement: random_ball_movement()
                },
                Ball {
                    radius: 20.0,
                    position: Point2::new(200.0, 200.0),
                    movement: random_ball_movement()
                },
            ],
            walls: vec![
                Wall {
                    a: Point2::new(0.0, 0.0),
                    b: Point2::new(800.0, 0.0),
                    n: Vector2::new(0.0, 1.0)
                },
                Wall {
                    a: Point2::new(0.0, 600.0),
                    b: Point2::new(800.0, 600.0),
                    n: Vector2::new(0.0, -1.0)
                },
                Wall {
                    a: Point2::new(800.0, 0.0),
                    b: Point2::new(800.0, 600.0),
                    n: Vector2::new(-1.0, 0.0)
                },
                Wall {
                    a: Point2::new(0.0, 0.0),
                    b: Point2::new(0.0, 600.0),
                    n: Vector2::new(1.0, 0.0)
                }
            ]
        }
    }
}

fn random_ball_movement() -> Vector2<f32> {
    let angle = 2.0 * PI * random::<f32>();
    Vector2::new(angle.sin(), angle.cos())
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

fn reflect_vector(i: Vector2<f32>, n: Vector2<f32>) -> Vector2<f32> {
    i - n * (n.dot(&i) * 2.0)
}

impl EventHandler for CrasballGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        for ball in self.balls.iter_mut() {

            let mut newpos = ball.position + ball.movement;
            let mut next_move = ball.movement;

            for wall in self.walls.iter() {
                let offset = wall.n * -ball.radius;
                let (intersects, offset_intersect_point) = find_intersection(
                    ball.position + offset, ball.position + ball.movement + offset,
                    wall.a, wall.b
                );

                if intersects {
                    let intersect_point = offset_intersect_point - offset;

                    next_move = reflect_vector(ball.movement, wall.n);

                    let travelled = intersect_point - ball.position;
                    let remaining = travelled.norm() / ball.movement.norm();

                    newpos = intersect_point + next_move * remaining;

                    break;
                }
            }

            ball.position = newpos;
            ball.movement = next_move;

        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        for ball in self.balls.iter() {

            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                ball.position,
                ball.radius,
                0.5,
                graphics::BLACK,
            )?;

            graphics::draw(ctx, &circle, (Point2::new(0.0, 0.0),))?;

        }

        graphics::present(ctx)
    }
}
