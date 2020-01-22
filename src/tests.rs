mod fixture_parser;

use nalgebra::{Point2};

use fixture_parser::{parse_live_area};
use crate::game::{LiveArea};

// does not add an edge that is within another edge
// combines edges that share point and direction
// NOTE edges should be INT


// https://math.stackexchange.com/questions/15815/how-to-union-many-polygons-efficiently

fn simplest_live_area() -> LiveArea {
    parse_live_area("
= = = = = = =
=           =
=           =
=     o     =
=           =
=           =
= = = = = = =
")
}

#[test]
fn test_add_partial_wall_to_down_facing_edge() {

    let initial = simplest_live_area();

    let output = initial.add_wall(
        Point2::new(40, 0),
        Point2::new(60, 0),
        Point2::new(60, 20),
        Point2::new(40, 20),
    );

    let expected = vec![
        parse_live_area("
= = = = = = =
=     =     =
=           =
=     o     =
=           =
=           =
= = = = = = =
")
    ];

    assert_eq!(output, expected);

}

#[test]
fn test_add_partial_wall_to_left_facing_edge() {

    let initial = simplest_live_area();

    let output = initial.add_wall(
        Point2::new(80, 40),
        Point2::new(100, 40),
        Point2::new(100, 60),
        Point2::new(80, 60),
    );

    let expected = vec![
        parse_live_area("
= = = = = = =
=           =
=           =
=     o   = =
=           =
=           =
= = = = = = =
")
    ];

    assert_eq!(output, expected);

}

#[test]
fn test_add_partial_wall_to_up_facing_edge() {

    let initial = simplest_live_area();

    let output = initial.add_wall(
        Point2::new(40, 80),
        Point2::new(60, 80),
        Point2::new(60, 100),
        Point2::new(40, 100),
    );

    let expected = vec![
        parse_live_area("
= = = = = = =
=           =
=           =
=     o     =
=           =
=     =     =
= = = = = = =
")
    ];

    assert_eq!(output, expected);

}

#[test]
fn test_wall_off_section() {

    let initial = simplest_live_area();

    let output = initial.add_wall(
        Point2::new(60, 0),
        Point2::new(80, 0),
        Point2::new(80, 100),
        Point2::new(60, 100),
    );

    let expected = vec![
        parse_live_area("
= = = = =
=       =
=       =
=     o =
=       =
=       =
= = = = =
")
    ];

    assert_eq!(output, expected);

}
