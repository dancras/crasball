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

fn find_edges(grid: SymbolGrid) -> Vec<Edge> {
    // find starting corner
    // follow it to the end,
    // then follow the next one
    Vec::new()
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
            b: Point2::new(40, 0),
            n: Facing::Down
        },
        Edge {
            a: Point2::new(40, 0),
            b: Point2::new(40, 40),
            n: Facing::Left
        },
        Edge {
            a: Point2::new(40, 40),
            b: Point2::new(0, 40),
            n: Facing::Up
        },
        Edge {
            a: Point2::new(0, 40),
            b: Point2::new(0, 0),
            n: Facing::Right
        }
    ]);

}
