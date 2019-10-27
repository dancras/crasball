use nalgebra::{Point2, Vector2};

use crate::game::{Ball, Edge, LiveArea};

#[test]
fn test_add() {
    assert_eq!(
        vec![
            Edge {
                a: Point2::new(0, 0),
                b: Point2::new(100, 0),
                n: Vector2::new(100.0, 100.0)
            }
        ],
        vec![
            Edge {
                a: Point2::new(0, 0),
                b: Point2::new(100, 0),
                n: Vector2::new(100.0, 100.0)
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

#[test]
fn test_add_partial_wall_to_down_facing_edge() {

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
                n: Vector2::new(0.0, 1.0)
            },
            Edge {
                a: Point2::new(100, 0),
                b: Point2::new(100, 100),
                n: Vector2::new(-1.0, 0.0)
            },
            Edge {
                a: Point2::new(100, 100),
                b: Point2::new(0, 100),
                n: Vector2::new(0.0, -1.0)
            },
            Edge {
                a: Point2::new(0, 100),
                b: Point2::new(0, 0),
                n: Vector2::new(1.0, 0.0)
            }
        ]
    };

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
                    n: Vector2::new(0.0, 1.0)
                },
                Edge {
                    a: Point2::new(40, 0),
                    b: Point2::new(40, 20),
                    n: Vector2::new(-1.0, 0.0)
                },
                Edge {
                    a: Point2::new(40, 20),
                    b: Point2::new(60, 20),
                    n: Vector2::new(0.0, 1.0)
                },
                Edge {
                    a: Point2::new(60, 20),
                    b: Point2::new(60, 0),
                    n: Vector2::new(1.0, 0.0)
                },
                Edge {
                    a: Point2::new(60, 0),
                    b: Point2::new(100, 0),
                    n: Vector2::new(0.0, 1.0)
                },
                Edge {
                    a: Point2::new(100, 0),
                    b: Point2::new(100, 100),
                    n: Vector2::new(-1.0, 0.0)
                },
                Edge {
                    a: Point2::new(100, 100),
                    b: Point2::new(0, 100),
                    n: Vector2::new(0.0, -1.0)
                },
                Edge {
                    a: Point2::new(0, 100),
                    b: Point2::new(0, 0),
                    n: Vector2::new(1.0, 0.0)
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
                n: Vector2::new(0.0, 1.0)
            },
            Edge {
                a: Point2::new(100, 0),
                b: Point2::new(100, 100),
                n: Vector2::new(-1.0, 0.0)
            },
            Edge {
                a: Point2::new(100, 100),
                b: Point2::new(0, 100),
                n: Vector2::new(0.0, -1.0)
            },
            Edge {
                a: Point2::new(0, 100),
                b: Point2::new(0, 0),
                n: Vector2::new(1.0, 0.0)
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
                    n: Vector2::new(0.0, 1.0)
                },
                Edge {
                    a: Point2::new(100, 0),
                    b: Point2::new(100, 40),
                    n: Vector2::new(-1.0, 0.0)
                },
                Edge {
                    a: Point2::new(100, 40),
                    b: Point2::new(80, 40),
                    n: Vector2::new(0.0, -1.0)
                },
                Edge {
                    a: Point2::new(80, 40),
                    b: Point2::new(80, 60),
                    n: Vector2::new(-1.0, 0.0)
                },
                Edge {
                    a: Point2::new(80, 60),
                    b: Point2::new(100, 60),
                    n: Vector2::new(0.0, 1.0)
                },
                Edge {
                    a: Point2::new(100, 60),
                    b: Point2::new(100, 100),
                    n: Vector2::new(-1.0, 0.0)
                },
                Edge {
                    a: Point2::new(100, 100),
                    b: Point2::new(0, 100),
                    n: Vector2::new(0.0, -1.0)
                },
                Edge {
                    a: Point2::new(0, 100),
                    b: Point2::new(0, 0),
                    n: Vector2::new(1.0, 0.0)
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
                n: Vector2::new(0.0, 1.0)
            },
            Edge {
                a: Point2::new(100, 0),
                b: Point2::new(100, 100),
                n: Vector2::new(-1.0, 0.0)
            },
            Edge {
                a: Point2::new(100, 100),
                b: Point2::new(0, 100),
                n: Vector2::new(0.0, -1.0)
            },
            Edge {
                a: Point2::new(0, 100),
                b: Point2::new(0, 0),
                n: Vector2::new(1.0, 0.0)
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
                    n: Vector2::new(0.0, 1.0)
                },
                Edge {
                    a: Point2::new(100, 0),
                    b: Point2::new(100, 100),
                    n: Vector2::new(-1.0, 0.0)
                },
                Edge {
                    a: Point2::new(100, 100),
                    b: Point2::new(60, 100),
                    n: Vector2::new(0.0, -1.0)
                },
                Edge {
                    a: Point2::new(60, 100),
                    b: Point2::new(60, 80),
                    n: Vector2::new(1.0, 0.0)
                },
                Edge {
                    a: Point2::new(60, 80),
                    b: Point2::new(40, 80),
                    n: Vector2::new(0.0, -1.0)
                },
                Edge {
                    a: Point2::new(40, 80),
                    b: Point2::new(40, 100),
                    n: Vector2::new(-1.0, 0.0)
                },
                Edge {
                    a: Point2::new(40, 100),
                    b: Point2::new(0, 100),
                    n: Vector2::new(0.0, -1.0)
                },
                Edge {
                    a: Point2::new(0, 100),
                    b: Point2::new(0, 0),
                    n: Vector2::new(1.0, 0.0)
                }
            ]
        }
    ];

    assert_eq!(output, expected);

}
