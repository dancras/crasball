use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use nalgebra::{convert, Point2, Vector2};
use nalgebra::base::coordinates::XY;

#[derive(Debug,PartialEq)]
pub struct Ball {
    pub radius: f32,
    pub position: Point2<f32>,
    pub movement: Vector2<f32>
}

pub struct GameState {
    pub balls: Vec<Ball>,
    pub edges: Vec<Edge>
}

#[derive(Debug,PartialEq)]
pub struct LiveArea {
    pub balls: Vec<Ball>,
    pub edges: Vec<Edge>
}

// fn is_point_on_edge(p: Point2<i16>, e: Edge) -> bool {
//     let v1 = p - e.a;
//     let v2 = e.b - e.a;

//     v1.cross(&v2) == 0.0
// }

impl LiveArea {
    pub fn add_wall(
        self,
        a: Point2<i16>,
        b: Point2<i16>,
        c: Point2<i16>,
        d: Point2<i16>
    ) -> Vec<LiveArea> {
        let mut current_area = LiveArea {
            balls: self.balls,
            edges: Vec::default()
        };

        for edge in self.edges {

            let XY {x: ax, y: ay} = *edge.a;
            let XY {x: bx, y: by} = *edge.b;

            if ay == by && ay == a.xy().y {
                current_area.edges.push(
                    Edge {
                        a: edge.a,
                        b: a,
                        n: edge.n
                    }
                );
                current_area.edges.push(
                    Edge {
                        a: a,
                        b: d,
                        n: Vector2::new(-1.0, 0.0)
                    }
                );
                current_area.edges.push(
                    Edge {
                        a: d,
                        b: c,
                        n: edge.n
                    }
                );
                current_area.edges.push(
                    Edge {
                        a: c,
                        b: b,
                        n: Vector2::new(1.0, 0.0)
                    }
                );
                current_area.edges.push(
                    Edge {
                        a: b,
                        b: edge.b,
                        n: edge.n
                    }
                );
            } else if ax == bx && ax == b.xy().x {
                current_area.edges.push(
                    Edge {
                        a: edge.a,
                        b: b,
                        n: edge.n
                    }
                );
                current_area.edges.push(
                    Edge {
                        a: b,
                        b: a,
                        n: Vector2::new(0.0, -1.0)
                    }
                );
                current_area.edges.push(
                    Edge {
                        a: a,
                        b: d,
                        n: edge.n
                    }
                );
                current_area.edges.push(
                    Edge {
                        a: d,
                        b: c,
                        n: Vector2::new(0.0, 1.0)
                    }
                );
                current_area.edges.push(
                    Edge {
                        a: c,
                        b: edge.b,
                        n: edge.n
                    }
                );
            } else if ay == by && ay == c.xy().y {
                current_area.edges.push(
                    Edge {
                        a: edge.a,
                        b: c,
                        n: edge.n
                    }
                );
                current_area.edges.push(
                    Edge {
                        a: c,
                        b: b,
                        n: Vector2::new(1.0, 0.0)
                    }
                );
                current_area.edges.push(
                    Edge {
                        a: b,
                        b: a,
                        n: edge.n
                    }
                );
                current_area.edges.push(
                    Edge {
                        a: a,
                        b: d,
                        n: Vector2::new(-1.0, 0.0)
                    }
                );
                current_area.edges.push(
                    Edge {
                        a: d,
                        b: edge.b,
                        n: edge.n
                    }
                );
            } else {
                current_area.edges.push(edge)
            }
        }

        vec![current_area]
    }
}

#[derive(Debug,PartialEq)]
pub struct Edge {
    pub a: Point2<i16>,
    pub b: Point2<i16>,
    pub n: Vector2<f32>
}

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
    let raw_vector = i - n * (n.dot(&i) * 2.0);
    let raw_magnitude = raw_vector.norm();

    raw_vector * (100.0 / raw_magnitude)
}

fn elastic_collision_vector(
    v1: Vector2<f32>, x1: Point2<f32>,
    v2: Vector2<f32>, x2: Point2<f32>
) -> Vector2<f32> {
    let x2_to_x1 = x1 - x2;
    let raw_vector = v1 - ((v1 - v2).dot(&x2_to_x1) / x2_to_x1.norm().powi(2)) * x2_to_x1;
    let raw_magnitude = raw_vector.norm();

    raw_vector * (100.0 / raw_magnitude)
}

impl GameState {
    pub fn update(&mut self, delta: f32) {
        let mut moved_balls: Vec<&mut Ball> = Vec::new();

        for ball in &mut self.balls {

            let to_move = ball.movement * delta;
            let mut newpos = ball.position + to_move;
            let mut next_move = ball.movement;

            for edge in self.edges.iter() {

                let edge_a: Point2<f32> = convert(edge.a);
                let edge_b: Point2<f32> = convert(edge.b);

                let distance_to_a = (edge_a - newpos).norm();

                if distance_to_a < ball.radius {

                    let overshoot = ball.radius - distance_to_a;
                    let remaining = overshoot / to_move.norm();

                    newpos = newpos - to_move * remaining;

                    next_move = reflect_vector(ball.movement, newpos - edge_a);

                    newpos = newpos + next_move * remaining * delta;

                    break;

                }

                let offset = edge.n * -ball.radius;
                let (intersects, offset_intersect_point) = find_intersection(
                    ball.position + offset, ball.position + to_move + offset,
                    edge_a, edge_b
                );
                if intersects {
                    let intersect_point = offset_intersect_point - offset;

                    next_move = reflect_vector(ball.movement, edge.n);

                    let travelled = intersect_point - ball.position;
                    let remaining = travelled.norm() / to_move.norm();

                    newpos = intersect_point + next_move * remaining * delta;

                    break;
                }
            }

            ball.position = newpos;
            ball.movement = next_move;

            for b2 in &mut moved_balls {

                let ball_to_b2 = ball.position - b2.position;
                let distance_apart = ball_to_b2.norm();

                if distance_apart < ball.radius + b2.radius {

                    let correction = (ball.radius + b2.radius - distance_apart) / 2.0;

                    let new_ball_movement = elastic_collision_vector(ball.movement, ball.position, b2.movement, b2.position);
                    let new_b2_movement = elastic_collision_vector(b2.movement, b2.position, ball.movement, ball.position);

                    ball.movement = new_ball_movement;
                    ball.position = ball.position + ball.movement * (correction / ball.movement.norm());

                    b2.movement = new_b2_movement;
                    b2.position = b2.position + b2.movement * (correction / b2.movement.norm());
                }

            }

            moved_balls.push(ball);

        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: 390.0,
                y: 300.0,
                w: 20.0,
                h: 300.0
            },
            graphics::BLACK,
        )?;

        graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;

        for ball in self.balls.iter() {

            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                ball.position,
                ball.radius,
                0.5,
                Color::new(1.0, 0.0, 0.0, 1.0),
            )?;

            graphics::draw(ctx, &circle, graphics::DrawParam::default())?;

        }

        Ok(())
    }
}
