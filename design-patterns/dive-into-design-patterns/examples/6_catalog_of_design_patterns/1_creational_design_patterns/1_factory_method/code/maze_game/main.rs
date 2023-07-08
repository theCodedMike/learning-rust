use crate::magic_maze::MagicMaze;
use crate::ordinary_maze::OrdinaryMaze;

mod game;
mod magic_maze;
mod ordinary_maze;

/// cargo r --example factory-method-maze-game
fn main() {
    // Option 1: The game starts with an ordinary maze.
    let ordinary_maze = OrdinaryMaze::new();
    game::run(ordinary_maze);

    // Option 2: The game starts with a magic maze.
    let magic_maze = MagicMaze::new();
    game::run(magic_maze);
}
