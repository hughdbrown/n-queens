use std::collections::{
    HashMap,
};

mod queen;
mod utils;

use queen::{
    place_queens,
    dump_board,
    Placed,
};
use utils::{
    Pos,
    See,
    Visited,
    calc_sees,
    calc_squares,
};

fn main() {
    let mut visited: Visited = Visited::new();
    let sees: HashMap<Pos, See> = calc_sees();
    let squares: Vec<Pos> = calc_squares();

    match place_queens(&mut visited, &sees, &squares) {
        Placed::Success => {
            //println!("{:#?}", visited.pos);
            dump_board(&visited.pos);
        },
        Placed::Failure => {
        },
    }
}
