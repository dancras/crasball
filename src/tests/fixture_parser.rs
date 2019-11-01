use std::cmp;
use nalgebra::{Point2};

use crate::game::{Edge, Facing};

#[derive(Clone,Copy,Debug,PartialEq)]
enum CellSymbol {
    Wall,
    NewWall,
    Ball,
    Empty
}

type SymbolGrid = Vec<Vec<CellSymbol>>;

fn parse_to_array(fixture: &str) -> SymbolGrid {

    let mut output = Vec::new();

    let longest_line = fixture.lines().fold(0, |a, v| cmp::max(a, v.len()));
    let cells_in_row = (longest_line + 1) / 2;

    for l in fixture.lines() {

        if l.len() == 0 {
            continue;
        }

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

fn create_edge(x1: usize, y1: usize, x2: usize, y2: usize, facing: Facing) -> Edge {

    let mods = match facing {
        Facing::Down => (0, 0, -20, 0),
        Facing::Left => (-20, 0, -20, -20),
        Facing::Up => (-20, -20, 0, -20),
        Facing::Right => (0, -20, 0, 0)
    };

    let (ax, ay, bx, by) = (
        x1 as i16 * 20 + mods.0,
        y1 as i16 * 20 + mods.1,
        x2 as i16 * 20 + mods.2,
        y2 as i16 * 20 + mods.3
    );

    Edge {
        a: Point2::new(ax, ay),
        b: Point2::new(bx, by),
        n: facing
    }

}

fn find_edges(grid: SymbolGrid) -> Vec<Edge> {

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

    let mut vx = 1;
    let mut vy = 0;

    let first_edge_x = x;
    let first_edge_y = y;

    let mut edge_start_x = x;
    let mut edge_start_y = y;

    let mut edge_facing = Facing::Down;

    x = x + 1;

    loop {

        let next_x = x as i16 + vx;
        let next_y = y as i16 + vy;

        let mut next_cell_out_of_bounds = false;
        let mut next_cell_not_wall = false;

        if next_y < 0 || grid.len() <= next_y as usize
            || next_x < 0 || grid[next_y as usize].len() <= next_x as usize {
            next_cell_out_of_bounds = true;
        } else {
            let next_cell = grid[next_y as usize][next_x as usize];

            if let CellSymbol::Wall = next_cell {
                // Do nothing
            } else {
                next_cell_not_wall = true;
            }
        }

        if next_cell_out_of_bounds || next_cell_not_wall {
            edges.push(
                create_edge(edge_start_x, edge_start_y, x, y, edge_facing)
            );

            if x == first_edge_x && y == first_edge_y {
                break;
            }

            edge_start_x = x;
            edge_start_y = y;

            edge_facing = edge_facing.clockwise();

            let new_v = match edge_facing {
                Facing::Down => (1, 0),
                Facing::Left => (0, 1),
                Facing::Up => (-1, 0),
                Facing::Right => (0, -1)
            };

            vx = new_v.0;
            vy = new_v.1;

        }

        x = (x as i16 + vx) as usize;
        y = (y as i16 + vy) as usize;

    }

// 1 = 2   8 = 9
// =   =   =   =
// =   = 6 7   =
// =   3 4     =
// =     5  o  =
// =           =
// b = = = = = a

// 1 = = ? = = v
// =     =     =
// =   4 ? 9   =
// =   5 6     =
// =     7  o  =
// =           =
// v = = = = = v

    edges
}

#[test]
fn test_find_edges_simple_perimeter() {

    let grid = vec![
        vec![CellSymbol::Wall, CellSymbol::Wall, CellSymbol::Wall],
        vec![CellSymbol::Wall, CellSymbol::Empty, CellSymbol::Wall],
        vec![CellSymbol::Wall, CellSymbol::Wall, CellSymbol::Wall],
    ];

    assert_eq!(find_edges(grid), [
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

// #[test]
// fn test_find_edges_complex_geometry() {

//     let grid = parse_to_array("
// = = = = = = =
// =     =     =
// =   = = =   =
// =   = =     =
// =     =     =
// =           =
// = = = = = = =
// ");

//     assert_eq!(find_edges(grid), [
//         Edge {
//             a: Point2::new(0, 0),
//             b: Point2::new(40, 0),
//             n: Facing::Down
//         },
//         Edge {
//             a: Point2::new(40, 0),
//             b: Point2::new(40, 40),
//             n: Facing::Left
//         },
//         Edge {
//             a: Point2::new(40, 40),
//             b: Point2::new(0, 40),
//             n: Facing::Up
//         },
//         Edge {
//             a: Point2::new(0, 40),
//             b: Point2::new(0, 0),
//             n: Facing::Right
//         }
//     ]);

// }
