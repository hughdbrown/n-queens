use std::fmt;
use std::collections::{
    HashMap,
    HashSet,
};

pub const NUM_ROWS: i8 = 8;
pub const NUM_COLS: i8 = NUM_ROWS;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct Pos(pub i8, pub i8);
impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct See {
    row: i8,
    col: i8,
    ldiag: i8,
    rdiag: i8,
}

impl See {
    // A queen in a particular Pos can see any other queen in the same row, column, left-diagonal,
    // or right-diagonal. All squares in the same row or column have the same row or column,
    // obviously. Not so obviously, all squares on the same left-diagonal have the same row -
    // column, and all squares on the same right-diagonal have the same row + column.
    // So we can identify conflicts by seeing if a row, column, left-diagonal or right-diagonal is
    // already occupied.
    fn new(pos: &Pos) -> Self {
        let (r, c) = (pos.0, pos.1);
        See { row: r, col: c, ldiag: r - c, rdiag: r + c }
    }
}

pub struct Visited {
    pub pos: Vec<Pos>,
    pub seen: Vec<See>,
    pub rows: HashSet<i8>,
    pub cols: HashSet<i8>,
    pub ldiags: HashSet<i8>,
    pub rdiags: HashSet<i8>,
}

impl Visited {
    pub fn new() -> Self {
        Visited {
            pos: Vec::new(),
            seen: Vec::new(),
            rows: HashSet::new(),
            cols: HashSet::new(),
            ldiags: HashSet::new(),
            rdiags: HashSet::new(),
        }
    }
    pub fn push(&mut self, pos: &Pos, see: &See) {
        self.pos.push(*pos);
        self.seen.push(*see);
        self.rows.insert(see.row);
        self.cols.insert(see.col);
        self.ldiags.insert(see.ldiag);
        self.rdiags.insert(see.rdiag);
    }
    pub fn pop(&mut self) {
        let end: usize = self.seen.len() - 1;
        let see: See = self.seen[end];
        self.rows.remove(&see.row);
        self.cols.remove(&see.col);
        self.ldiags.remove(&see.ldiag);
        self.rdiags.remove(&see.rdiag);
        self.seen.remove(end);

        let end: usize = self.pos.len() - 1;
        self.pos.remove(end);
    }
    pub fn can_push(&self, see: &See) -> bool {
        ! (
            self.rows.contains(&see.row) || 
            self.cols.contains(&see.col) || 
            self.ldiags.contains(&see.ldiag) ||
            self.rdiags.contains(&see.rdiag)
        )
    }
}

pub fn calc_squares() -> HashSet<Pos> {
    let mut squares: HashSet<Pos> = HashSet::new();
    for row in 0i8..(NUM_ROWS as i8) {
        for col in 0i8..(NUM_COLS as i8) {
            let pos: Pos = Pos(row, col);
            squares.insert(pos);
        }
    }
    squares
}

pub fn calc_sees() -> HashMap<Pos, See> {
    let mut sees: HashMap<Pos, See> = HashMap::new();
    for row in 0i8..(NUM_ROWS as i8) {
        for col in 0i8..(NUM_COLS as i8) {
            let pos: Pos = Pos(row, col);
            let see: See = See::new(&pos);
            sees.insert(pos, see);
        }
    }
    sees
}

