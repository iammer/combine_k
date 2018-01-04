extern crate rand;

use std::fmt;

mod tile;
pub use self::tile::Tile;

const SIZE: usize = 4;
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up, Down, Left, Right
}

#[derive(Clone)]
pub struct Board {
    pub tiles: [Tile; SIZE * SIZE],
    pub score: u32
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:^6}\r\n ---- \r\n",self.score)?;
        for (i, t) in self.tiles.iter().enumerate() {
            match i % SIZE {
                0 => write!(f,"|{}",t),
                3 => write!(f, "{}|\r\n", t),
                _ => write!(f, "{}", t)
            }?
        }
        write!(f, " ---- \r\n")
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            tiles: [Tile::Empty; SIZE * SIZE],
            score: 0
        }
    }

    fn move_tile(&mut self, m: Direction, i: usize) -> bool {
        if let Some((n, merged, s)) = self.slide(m, i) {
            self.tiles[i] = Tile::Empty;
            self.tiles[n] = merged;
            self.score += s;
            true
        } else {
            false
        }
    }

    pub fn move_board(&mut self, m: Direction) -> bool {
        if m == Direction::Up || m == Direction::Left {
            (0..SIZE*SIZE)
                .map(|i| self.move_tile(m,i)).collect::<Vec<bool>>()
                .into_iter().any(|x| x)
        } else {
            (0..SIZE*SIZE).rev()
                .map(|i| self.move_tile(m,i)).collect::<Vec<bool>>()
                .into_iter().any(|x| x)
        }
    }

    pub fn add_tile(&mut self) -> bool {
        let (empty_tiles, _): (Vec<usize>,Vec<Tile>) = self.tiles.iter().enumerate()
            .filter(|&(_, &x)| x == Tile::Empty).unzip();

        let empty_count: usize = empty_tiles.len();
        if empty_count > 0 {
            let selected = rand::random::<usize>() % empty_count;
            let value: u8 = if rand::random::<f32>() > 0.9 { 2 } else { 1 };

            self.tiles[empty_tiles[selected]] = Tile::Occupied(value);

            true
        } else {
            false
        }

    }

    fn tile_to(&self, dir: Direction, i: usize) -> Option<(usize, Tile)> {
        space_to(dir, i).map(|n| (n, self.tiles[n]))
    }

    fn slide(&self, dir: Direction, i: usize) -> Option<(usize, Tile, u32)> {
        let c = self.tiles[i];
        if c == Tile::Empty {
            None
        } else {
            let (n, t, s) = self.slide_next(dir, i, c);
            if n == i {
                None
            } else {
                Some((n, t, s))
            }
        }
    }

    fn slide_next(&self, dir: Direction, i: usize, t: Tile) -> (usize, Tile, u32) {
        match self.tile_to(dir, i) {
            Some((n, Tile::Empty)) => self.slide_next(dir, n, t),
            Some((n, o)) if o == t => {
                let new_tile = t.next();
                (n, new_tile, new_tile.score())
            },
            _ => (i, t, 0)
        }
    }

}

fn to_row_col(i: usize) -> (usize, usize) {
    ( i / SIZE, i % SIZE )
}

fn from_row_col(r: usize, c: usize) -> usize {
    r * SIZE + c
}

fn space_to(dir: Direction, i: usize) -> Option<usize> {
    match (dir, to_row_col(i)) {
        (Direction::Up, (0, _)) => None,
        (Direction::Down, (z, _)) if z == SIZE - 1 => None,
        (Direction::Left, (_, 0)) => None,
        (Direction::Right, (_, z)) if z == SIZE -1 => None,
        (Direction::Up, (r, c)) => Some(from_row_col(r-1,c)),
        (Direction::Down, (r, c)) => Some(from_row_col(r+1, c)),
        (Direction::Left, (r, c)) => Some(from_row_col(r, c-1)),
        (Direction::Right, (r, c)) => Some(from_row_col(r, c+1))
    }
}

