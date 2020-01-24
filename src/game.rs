use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};
use nalgebra::{convert, Point2, Vector2};

#[derive(Clone,Copy,Debug,PartialEq)]
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

fn is_point_on_edge(p: Point2<i16>, e: &Edge) -> bool {
    p >= e.a && p <= e.b ||
    p >= e.b && p <= e.a
}

fn is_point_on_any_edge(p: Point2<i16>, edges: &[Edge]) -> (bool, usize) {
    for (i, edge) in edges.iter().enumerate() {
        if is_point_on_edge(p, edge) {
            return (true, i);
        }
    }

    (false, 0)
}

impl LiveArea {

    fn ball_is_inside(self, ball: Ball) -> bool {
        false
    }

    pub fn add_wall(
        self,
        a: Point2<i16>,
        b: Point2<i16>,
        c: Point2<i16>,
        d: Point2<i16>
    ) -> Vec<LiveArea> {
        let mut current_area = LiveArea {
            balls: Vec::default(),
            edges: Vec::default()
        };

        let new_points = [a, b, c, d];

        let mut ignore_until = 0;
        for j in 0..self.edges.len() {

            if ignore_until > j {
                continue;
            }

            let edge = self.edges[j];
            let rest = &self.edges[(j+1)..];

            let i = match edge.n {
                Facing::Down => 0,
                Facing::Left => 1,
                Facing::Up => 2,
                Facing::Right => 3,
            };

            if is_point_on_edge(new_points[i], &edge) {
                current_area.edges.push(
                    Edge {
                        a: edge.a,
                        b: new_points[i],
                        n: edge.n
                    }
                );

                current_area.edges.push(
                    Edge {
                        a: new_points[i],
                        b: new_points[(i + 3) % 4],
                        n: edge.n.clockwise()
                    }
                );

                let (connects_edges, connect_i) = is_point_on_any_edge(new_points[(i + 3) % 4], rest);
                if connects_edges {

                    // iterate balls and put those in "current_area" into it
                    // if all balls are in, ignore the other edges
                    // otherwise the other balls need to be added to another area

                    ignore_until = connect_i + 2;

                    current_area.edges.push(
                        Edge {
                            a: new_points[(i + 3) % 4],
                            b: rest[connect_i].b,
                            n: rest[connect_i].n
                        }
                    );

                    for ball in self.balls.clone() {
                        if current_area.ball_is_inside(ball) {
                            current_area.balls.push(ball);
                        }
                    }
                } else {

                    current_area.edges.push(
                        Edge {
                            a: new_points[(i + 3) % 4],
                            b: new_points[(i + 2) % 4],
                            n: edge.n
                        }
                    );
                    current_area.edges.push(
                        Edge {
                            a: new_points[(i + 2) % 4],
                            b: new_points[(i + 1) % 4],
                            n: edge.n.anticlockwise()
                        }
                    );
                    current_area.edges.push(
                        Edge {
                            a: new_points[(i + 1) % 4],
                            b: edge.b,
                            n: edge.n
                        }
                    );
                }
            } else {
                current_area.edges.push(edge)
            }
        }

        vec![current_area]
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Edge {
    pub a: Point2<i16>,
    pub b: Point2<i16>,
    pub n: Facing
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Facing {
    Down,
    Left,
    Up,
    Right
}

impl Facing {
    pub fn clockwise(self) -> Self {
        match self {
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Right => Self::Down
        }
    }

    pub fn anticlockwise(self) -> Self {
        match self {
            Self::Down => Self::Right,
            Self::Right => Self::Up,
            Self::Up => Self::Left,
            Self::Left => Self::Down
        }
    }

    // fn opposite(self) -> Self {
    //     match self {
    //         Self::Down => Self::Up,
    //         Self::Up => Self::Down,
    //         Self::Left => Self::Right,
    //         Self::Right => Self::Left
    //     }
    // }
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

                let edge_n: Vector2<f32> = match edge.n {
                    Facing::Down => Vector2::new(0.0, 1.0),
                    Facing::Left => Vector2::new(-1.0, 0.0),
                    Facing::Up => Vector2::new(0.0, -1.0),
                    Facing::Right => Vector2::new(1.0, 0.0)
                };

                let distance_to_a = (edge_a - newpos).norm();

                if distance_to_a < ball.radius {

                    let overshoot = ball.radius - distance_to_a;
                    let remaining = overshoot / to_move.norm();

                    newpos = newpos - to_move * remaining;

                    next_move = reflect_vector(ball.movement, newpos - edge_a);

                    newpos = newpos + next_move * remaining * delta;

                    break;

                }

                let offset = edge_n * -ball.radius;
                let (intersects, offset_intersect_point) = find_intersection(
                    ball.position + offset, ball.position + to_move + offset,
                    edge_a, edge_b
                );
                if intersects {
                    let intersect_point = offset_intersect_point - offset;

                    next_move = reflect_vector(ball.movement, edge_n);

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
