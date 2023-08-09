use std::collections::{
    HashMap,
};
use crate::utils::{
    Pos,
    See,
    Visited,
    NUM_ROWS,
    NUM_COLS,
    calc_unattacked,
    can_cover,
};

pub enum Placed {
    Success,
    Failure,
}

pub fn place_queens(
    visited: &mut Visited, // Squares that have been visited. Initially empty.
    sees: &HashMap<Pos, See>, // Rows, columns, and diagonals visible by each square
    squares: &[Pos], // Squares left that can be occupied by a queen.
) -> Placed {
    if visited.pos.len() == (NUM_ROWS as usize) {
        return Placed::Success;
    }

    let unattacked: Vec<Pos> = calc_unattacked(squares, visited, sees);
    if can_cover(&unattacked) {
        for pos in unattacked.iter() {
            // Get the (row, col, ldiag, rdiag) characteristics of `pos`.
            if let Some(see) = sees.get(pos) {
                // Temporarily add `pos` and `see` to `visited`
                visited.push(pos, see);

                // And recurse.
                match place_queens(visited, sees, &unattacked) {
                    Placed::Success => {
                        return Placed::Success;
                    },
                    Placed::Failure => {
                        // Not a good board: backtrack
                        visited.pop();
                    },
                }
            }
        }
    }

    Placed::Failure
}

pub fn dump_board(pos: &[Pos])
{
    let mut map = [['.'; NUM_COLS as usize]; NUM_ROWS as usize];
    for p in pos.iter() {
        let (x, y) = (p.0, p.1);
        map[x as usize][y as usize] = 'Q';
    }
    for i in 0..NUM_ROWS {
        for j in 0..NUM_COLS {
            print!("{} ", map[i as usize][j as usize]);
        }
        print!("\n");
    }
}
