mod fixture_parser;

use nalgebra::{Point2, Vector2};

use crate::game::{Ball, Edge, Facing, LiveArea};
use fixture_parser::{parse_live_area};

#[test]
fn test_add() {
    assert_eq!(
        vec![
            Edge {
                a: Point2::new(0, 0),
                b: Point2::new(100, 0),
                n: Facing::Down
            }
        ],
        vec![
            Edge {
                a: Point2::new(0, 0),
                b: Point2::new(100, 0),
                n: Facing::Down
            }
        ]
    );
}

// does not add an edge that is within another edge
// combines edges that share point and direction
// NOTE edges should be INT


// https://math.stackexchange.com/questions/15815/how-to-union-many-polygons-efficiently

// LiveArea
//


// convert the string into a grid array
// iterate and collect a list of vertexes
//
// v = | = v = | = v = | = v =
// =   |   =   |   v   | v v
//     |       |       | =

// it will probably be super tough to convert these to edges actually..

// maybe better to "search" for the starting point and trace the edges

// a vertex cell has
//  a wall has a joining wall in 2 axis and is not completely surrounded
//

// = = = = = = =
// =     *     =
// =           =
// =           =
// =        o  =
// =           =
// = = = = = = =

// = = = = = = =
// =     =     =
// =           =
// =           =
// =        o  =
// =           =
// = = = = = = =

// = = = = = = =
// =     =     =
// =     =     =
// =     =     =
// =  o  *  o  =
// =     *     =
// = = = = = = =

// = = = =
// =     =
// =     =
// =     =
// =  o  =
// =     =
// = = = =

//       = = = =
//       =     =
//       =     =
//       =     =
//       =  o  =
//       =     =
//       = = = =


#[test]
fn test_add_partial_wall_to_down_facing_edge() {

    let initial = parse_live_area("
= = = = = = =
=           =
=           =
=     o     =
=           =
=           =
= = = = = = =
");

    let output = initial.add_wall(
        Point2::new(40, 0),
        Point2::new(60, 0),
        Point2::new(60, 20),
        Point2::new(40, 20),
    );

    let expected = vec![
        LiveArea {
            balls: vec![
                Ball {
                    radius: 20.0,
                    position: Point2::new(50.0, 50.0),
                    movement: Vector2::new(0.0, 0.0)
                }
            ],
            edges: vec![
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
                    b: Point2::new(60, 20),
                    n: Facing::Down
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
            ]
        }
    ];

    assert_eq!(output, expected);

}

#[test]
fn test_add_partial_wall_to_left_facing_edge() {

    let initial = LiveArea {
        balls: vec![
            Ball {
                radius: 20.0,
                position: Point2::new(50.0, 50.0),
                movement: Vector2::new(0.0, 0.0)
            }
        ],
        edges: vec![
            Edge {
                a: Point2::new(0, 0),
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
        ]
    };

    let output = initial.add_wall(
        Point2::new(80, 40),
        Point2::new(100, 40),
        Point2::new(100, 60),
        Point2::new(80, 60),
    );

    let expected = vec![
        LiveArea {
            balls: vec![
                Ball {
                    radius: 20.0,
                    position: Point2::new(50.0, 50.0),
                    movement: Vector2::new(0.0, 0.0)
                }
            ],
            edges: vec![
                Edge {
                    a: Point2::new(0, 0),
                    b: Point2::new(100, 0),
                    n: Facing::Down
                },
                Edge {
                    a: Point2::new(100, 0),
                    b: Point2::new(100, 40),
                    n: Facing::Left
                },
                Edge {
                    a: Point2::new(100, 40),
                    b: Point2::new(80, 40),
                    n: Facing::Up
                },
                Edge {
                    a: Point2::new(80, 40),
                    b: Point2::new(80, 60),
                    n: Facing::Left
                },
                Edge {
                    a: Point2::new(80, 60),
                    b: Point2::new(100, 60),
                    n: Facing::Down
                },
                Edge {
                    a: Point2::new(100, 60),
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
            ]
        }
    ];

    assert_eq!(output, expected);

}

#[test]
fn test_add_partial_wall_to_up_facing_edge() {

    let initial = LiveArea {
        balls: vec![
            Ball {
                radius: 20.0,
                position: Point2::new(50.0, 50.0),
                movement: Vector2::new(0.0, 0.0)
            }
        ],
        edges: vec![
            Edge {
                a: Point2::new(0, 0),
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
        ]
    };

    let output = initial.add_wall(
        Point2::new(40, 80),
        Point2::new(60, 80),
        Point2::new(60, 100),
        Point2::new(40, 100),
    );

    let expected = vec![
        LiveArea {
            balls: vec![
                Ball {
                    radius: 20.0,
                    position: Point2::new(50.0, 50.0),
                    movement: Vector2::new(0.0, 0.0)
                }
            ],
            edges: vec![
                Edge {
                    a: Point2::new(0, 0),
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
                    b: Point2::new(60, 100),
                    n: Facing::Up
                },
                Edge {
                    a: Point2::new(60, 100),
                    b: Point2::new(60, 80),
                    n: Facing::Right
                },
                Edge {
                    a: Point2::new(60, 80),
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
                    b: Point2::new(0, 100),
                    n: Facing::Up
                },
                Edge {
                    a: Point2::new(0, 100),
                    b: Point2::new(0, 0),
                    n: Facing::Right
                }
            ]
        }
    ];

    assert_eq!(output, expected);

}
