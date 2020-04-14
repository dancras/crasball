# CrasBall

WIP clone of the game JezzBall which shipped with windows a long time ago. The project is purely for self-education.


## Setup

VSCode extensions:

- CodeLLDB
- EditorConfig
- Rust language support

`cargo run` to run the game

`cargo test` to run the tests


## Implementation notes

Rather than use a flood fill grid algorithm the game is modelled as a collection of live areas, polygon shapes which still contain balls, and handle a new wall by updating themselves or splitting themselves into more live areas as needed by following a step by step algorithm through the points of the polygon.

The tests are represented as ASCII art game state examples which are very useful and easy to return to after a long break.
