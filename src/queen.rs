use std::collections::{
    HashMap,
    HashSet,
};
use crate::utils::{
    Pos,
    See,
    Visited,
    NUM_ROWS,
    NUM_COLS,
};

pub enum Placed {
    Success,
    Failure,
}

pub fn place_queens(
    visited: &mut Visited, // Squares that have been visited. Initially empty.
    sees: &HashMap<Pos, See>, // Rows, columns, and diagonals visible by each square
    squares: &mut HashSet<Pos>, // 
) -> Placed {
    if visited.pos.len() == (NUM_ROWS as usize) {
        return Placed::Success;
    }

    let v: Vec<Pos> = squares.clone().iter().map(|x| *x).collect::<Vec::<Pos>>();
    for pos in v.iter() {
        if let Some(see) = sees.get(pos) {
            if visited.can_push(see) {
                visited.push(pos, see);
                squares.remove(pos);
                match place_queens(visited, sees, squares) {
                    Placed::Success => {
                        return Placed::Success;
                    },
                    Placed::Failure => {
                        // Not a good board: backtrack
                        visited.pop();
                        squares.insert(*pos);
                    },
                }
            }
        }
    }

    Placed::Failure
}

pub fn dump_board(pos: &Vec<Pos>)
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
