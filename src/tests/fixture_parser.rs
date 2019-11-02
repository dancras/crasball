use std::cmp;
use nalgebra::{Point2, Vector2};

use crate::game::{Ball, Edge, Facing, LiveArea};

#[derive(Clone,Copy,Debug,PartialEq)]
enum CellSymbol {
    Wall,
    NewWall,
    Ball,
    Empty
}

type SymbolGrid = Vec<Vec<CellSymbol>>;

pub fn parse_live_area(fixture: &str) -> LiveArea {

    let grid = parse_to_array(fixture);

    LiveArea {
        balls: find_balls(&grid),
        edges: find_edges(&grid)
    }

}

fn parse_to_array(fixture: &str) -> SymbolGrid {

    let mut output = Vec::new();

    let longest_line = fixture.lines().fold(0, |a, v| cmp::max(a, v.len()));
    let cells_in_row = (longest_line + 1) / 2;

    for l in fixture.lines().skip(1) {

        let mut row = vec![CellSymbol::Empty; cells_in_row];

        for (i, c) in l.char_indices() {

            if i % 2 > 0 {
                continue;
            }

            row[i / 2] = match c {
                '=' => CellSymbol::Wall,
                '*' => CellSymbol::NewWall,
                'o' => CellSymbol::Ball,
                ' ' => CellSymbol::Empty,
                _ => panic!("Unknown symbol in fixture")
            };

        }

        output.push(row);

    }

    output

}

#[test]
fn test_parse_to_array() {
    let fixture = "
= * * =
  = o
";

    let result = parse_to_array(fixture);

    assert_eq!(result, [
        [CellSymbol::Wall, CellSymbol::NewWall, CellSymbol::NewWall, CellSymbol::Wall],
        [CellSymbol::Empty, CellSymbol::Wall, CellSymbol::Ball, CellSymbol::Empty]
    ]);
}

#[test]
fn test_parse_to_array_empty_areas() {
    let fixture = "

  =
";

    let result = parse_to_array(fixture);

    assert_eq!(result, [
        [CellSymbol::Empty, CellSymbol::Empty],
        [CellSymbol::Empty, CellSymbol::Wall]
    ]);
}

fn find_balls(grid: &SymbolGrid) -> Vec<Ball> {
    let mut balls = Vec::new();

    for (row_i, row) in grid.iter().enumerate() {
        for (cell_i, cell) in row.iter().enumerate() {

            if CellSymbol::Ball == *cell {

                balls.push(Ball {
                    radius: 20.0,
                    position: Point2::new(
                        cell_i as f32 * 20.0 - 10.0,
                        row_i as f32 * 20.0 - 10.0,
                    ),
                    movement: Vector2::new(0.0, 0.0)
                });

            }

        }
    }

    balls
}


#[test]
fn test_find_balls() {

    let grid = vec![
        vec![CellSymbol::Wall, CellSymbol::Wall, CellSymbol::Wall],
        vec![CellSymbol::Wall, CellSymbol::Ball, CellSymbol::Empty],
        vec![CellSymbol::Wall, CellSymbol::Empty, CellSymbol::Ball]
    ];

    assert_eq!(find_balls(&grid), [
        Ball {
            radius: 20.0,
            position: Point2::new(10.0, 10.0),
            movement: Vector2::new(0.0, 0.0)
        },
        Ball {
            radius: 20.0,
            position: Point2::new(30.0, 30.0),
            movement: Vector2::new(0.0, 0.0)
        }
    ]);

}


fn calculate_edge_coordinate(x: usize, y: usize, facing: Facing, next_facing: Facing) -> Point2<i16> {

    let mut mod_x = 0;
    let mut mod_y = 0;

    if Facing::Left == facing || Facing::Left == next_facing {
        mod_x = -20;
    }

    if Facing::Up == facing || Facing::Up == next_facing {
        mod_y = -20;
    }

    Point2::new(x as i16 * 20 + mod_x, y as i16 * 20 + mod_y)
}

fn get_cell(grid: &SymbolGrid, x: i16, y: i16) -> Option<CellSymbol> {

    if y < 0 || grid.len() <= y as usize
        || x < 0 || grid[y as usize].len() <= x as usize {
        None
    } else {
        Some(grid[y as usize][x as usize])
    }

}

fn velocities_from_facing(facing: Facing) -> (i16, i16) {
    match facing {
        Facing::Down => (1, 0),
        Facing::Left => (0, 1),
        Facing::Up => (-1, 0),
        Facing::Right => (0, -1)
    }
}

// This algorithm searches the grid for a Wall cell, and then follows this wall
// around, back to the start. No handling for bad fixtures.
fn find_edges(grid: &SymbolGrid) -> Vec<Edge> {

    let mut edges: Vec<Edge> = Vec::new();

    let mut x = 0;
    let mut y = 0;

    // First find a wall
    loop {
        let cell = grid[y][x];

        if let CellSymbol::Wall = cell {
            break;
        } else {
            y = y + 1;

            if y >= grid.len() {
                y = 0;
                x = x + 1;
            }
        }
    }

    // To traverse the walls of the grid using "velocities" for each axis
    let mut vx = 1;
    let mut vy = 0;

    // To know when we are back where we started
    let first_edge_x = x;
    let first_edge_y = y;

    // Initialise the first vertex and facing of the first edge
    let mut edge_start = Point2::new(x as i16 * 20, y as i16 * 20);
    let mut edge_facing = Facing::Down;

    // Move from the starting point
    x = x + 1;

    loop {

        let next_move_is_not_wall = match get_cell(&grid, x as i16 + vx, y as i16 + vy) {
            Some(CellSymbol::Wall) => false,
            _ => true
        };

        let clockwise_facing = edge_facing.clockwise();

        let clockwise_v = velocities_from_facing(clockwise_facing);

        let clockwise_move_is_wall = match get_cell(&grid, x as i16 + clockwise_v.0, y as i16 + clockwise_v.1) {
            Some(CellSymbol::Wall) => true,
            _ => false
        };

        if next_move_is_not_wall || clockwise_move_is_wall {

            let next_edge_facing;

            if clockwise_move_is_wall {
                next_edge_facing = clockwise_facing;
            } else {

                let anticlockwise_facing = edge_facing.anticlockwise();

                let anticlockwise_v = velocities_from_facing(anticlockwise_facing);

                let anticlockwise_move_is_wall = match get_cell(&grid, x as i16 + anticlockwise_v.0, y as i16 + anticlockwise_v.1) {
                    Some(CellSymbol::Wall) => true,
                    _ => false
                };

                if anticlockwise_move_is_wall {
                    next_edge_facing = anticlockwise_facing;
                } else {
                    // For the case of backtracking we need to add an additional edge to traverse the current cell
                    let anticlockwise_facing = edge_facing.anticlockwise();
                    let edge_end = calculate_edge_coordinate(x, y, edge_facing, anticlockwise_facing);

                    edges.push(Edge {
                        a: edge_start,
                        b: edge_end,
                        n: edge_facing
                    });

                    edge_start = edge_end;
                    edge_facing = anticlockwise_facing;

                    let opposite_facing = anticlockwise_facing.anticlockwise();

                    next_edge_facing = opposite_facing;

                }
            }

            let edge_end = calculate_edge_coordinate(x, y, edge_facing, next_edge_facing);

            edges.push(Edge {
                a: edge_start,
                b: edge_end,
                n: edge_facing
            });

            if x == first_edge_x && y == first_edge_y {
                break;
            }

            edge_start = edge_end;
            edge_facing = next_edge_facing;

            let next_velocities = velocities_from_facing(edge_facing);
            vx = next_velocities.0;
            vy = next_velocities.1;

        }

        x = (x as i16 + vx) as usize;
        y = (y as i16 + vy) as usize;

    }

    edges
}

#[test]
fn test_find_edges_simple_perimeter() {

    let grid = vec![
        vec![CellSymbol::Wall, CellSymbol::Wall, CellSymbol::Wall],
        vec![CellSymbol::Wall, CellSymbol::Empty, CellSymbol::Wall],
        vec![CellSymbol::Wall, CellSymbol::Wall, CellSymbol::Wall],
    ];

    assert_eq!(find_edges(&grid), [
        Edge {
            a: Point2::new(0, 0),
            b: Point2::new(20, 0),
            n: Facing::Down
        },
        Edge {
            a: Point2::new(20, 0),
            b: Point2::new(20, 20),
            n: Facing::Left
        },
        Edge {
            a: Point2::new(20, 20),
            b: Point2::new(0, 20),
            n: Facing::Up
        },
        Edge {
            a: Point2::new(0, 20),
            b: Point2::new(0, 0),
            n: Facing::Right
        }
    ]);

}

#[test]
fn test_find_edges_complex_geometry() {

    let grid = parse_to_array("
= = = = = = =
=     =     =
=   = = =   =
=   = =     =
=     =     =
=           =
= = = = = = =
");

    assert_eq!(find_edges(&grid), [
        Edge {
            a: Point2::new(0, 0),
            b: Point2::new(40, 0),
            n: Facing::Down
        },
        Edge {
            a: Point2::new(40, 0),
            b: Point2::new(40, 20),
            n: Facing::Left
        },
        Edge {
            a: Point2::new(40, 20),
            b: Point2::new(20, 20),
            n: Facing::Up
        },
        Edge {
            a: Point2::new(20, 20),
            b: Point2::new(20, 60),
            n: Facing::Left
        },
        Edge {
            a: Point2::new(20, 60),
            b: Point2::new(40, 60),
            n: Facing::Down
        },
        Edge {
            a: Point2::new(40, 60),
            b: Point2::new(40, 80),
            n: Facing::Left
        },
        Edge {
            a: Point2::new(40, 80),
            b: Point2::new(60, 80),
            n: Facing::Down
        },
        Edge {
            a: Point2::new(60, 80),
            b: Point2::new(60, 40),
            n: Facing::Right
        },
        Edge {
            a: Point2::new(60, 40),
            b: Point2::new(80, 40),
            n: Facing::Down
        },
        Edge {
            a: Point2::new(80, 40),
            b: Point2::new(80, 20),
            n: Facing::Right
        },
        Edge {
            a: Point2::new(80, 20),
            b: Point2::new(60, 20),
            n: Facing::Up
        },
        Edge {
            a: Point2::new(60, 20),
            b: Point2::new(60, 0),
            n: Facing::Right
        },
        Edge {
            a: Point2::new(60, 0),
            b: Point2::new(100, 0),
            n: Facing::Down
        },
        Edge {
            a: Point2::new(100, 0),
            b: Point2::new(100, 100),
            n: Facing::Left
        },
        Edge {
            a: Point2::new(100, 100),
            b: Point2::new(0, 100),
            n: Facing::Up
        },
        Edge {
            a: Point2::new(0, 100),
            b: Point2::new(0, 0),
            n: Facing::Right
        }
    ]);

}

#[test]
fn test_find_edges_partial_area() {

    let grid = parse_to_array("

    = = = =
  = =     = =
  =         =
  =         =
  =   = = = =
  = = =
");

    assert_eq!(find_edges(&grid), [
        Edge {
            a: Point2::new(20, 40),
            b: Point2::new(40, 40),
            n: Facing::Down
        },
        Edge {
            a: Point2::new(40, 40),
            b: Point2::new(40, 20),
            n: Facing::Right
        },
        Edge {
            a: Point2::new(40, 20),
            b: Point2::new(80, 20),
            n: Facing::Down
        },
        Edge {
            a: Point2::new(80, 20),
            b: Point2::new(80, 40),
            n: Facing::Left
        },
        Edge {
            a: Point2::new(80, 40),
            b: Point2::new(100, 40),
            n: Facing::Down
        },
        Edge {
            a: Point2::new(100, 40),
            b: Point2::new(100, 80),
            n: Facing::Left
        },
        Edge {
            a: Point2::new(100, 80),
            b: Point2::new(40, 80),
            n: Facing::Up
        },
        Edge {
            a: Point2::new(40, 80),
            b: Point2::new(40, 100),
            n: Facing::Left
        },
        Edge {
            a: Point2::new(40, 100),
            b: Point2::new(20, 100),
            n: Facing::Up
        },
        Edge {
            a: Point2::new(20, 100),
            b: Point2::new(20, 40),
            n: Facing::Right
        }
    ]);

}
