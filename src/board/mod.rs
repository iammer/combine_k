extern crate rand;

use std::fmt;

#[cfg(test)]
mod tests;

mod tile;
pub use self::tile::Tile;

const SIZE: usize = 4;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up, Down, Left, Right
}

struct SlideResult {
    tile: Tile,
    position: usize,
    score: u32
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub tiles: Vec<Tile>,
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
            tiles: vec![Tile::Empty; SIZE * SIZE],
            score: 0
        }
    }

    fn finalize(self) -> Self {
        Board {
            tiles: self.tiles.into_iter().map(|t| t.finalize()).collect(),
            score: self.score
        }
    }

    //Moves all tiles on the board in direction m
    pub fn move_board(&self, m: Direction) -> Option<Self> {
        let mut board = self.clone();

        let did_move;
        if m == Direction::Up || m == Direction::Left {
            did_move = (0..SIZE*SIZE)
                .map(|i| board.move_tile(m,i)).collect::<Vec<bool>>()
                .into_iter().any(|x| x);
        } else {
            did_move = (0..SIZE*SIZE).rev()
                .map(|i| board.move_tile(m,i)).collect::<Vec<bool>>()
                .into_iter().any(|x| x);
        }

        if did_move {
            Some(board.finalize())
        } else {
            None
        }
    }

    //Adds a new tile to board in a random empty space
    pub fn add_tile(&self) -> Option<Self> {
        let (empty_tiles, _): (Vec<usize>,Vec<Tile>) = self.tiles.iter().enumerate()
            .filter(|&(_, &x)| x == Tile::Empty).unzip();

        let empty_count: usize = empty_tiles.len();
        if empty_count > 0 {
            let mut board = self.clone();
            let selected = rand::random::<usize>() % empty_count;
            let value: u8 = if rand::random::<f32>() > 0.9 { 2 } else { 1 };

            board.tiles[empty_tiles[selected]] = Tile::Occupied(value);

            Some(board)
        } else {
            None
        }

    }

    //Slides a single tile in direction returns true if tile was moved
    fn move_tile(&mut self, m: Direction, i: usize) -> bool {
        if let Some(r) = self.slide(m, i) {
            self.tiles[i] = Tile::Empty;
            self.tiles[r.position] = r.tile;
            self.score += r.score;
            true
        } else {
            false
        }
    }

    //Finds next tile in dir
    //Returns None if no tile (dir is edge)
    //Returns Some((position, tile)) if tile exists
    fn tile_to(&self, dir: Direction, i: usize) -> Option<(usize, Tile)> {
        space_to(dir, i).map(|n| (n, self.tiles[n]))
    }

    //Gets result of sliding tile @ position i in dir
    //Returns None if tile was not able to slide
    //Returns Some(SlideResult) if tile could slide
    fn slide(&self, dir: Direction, i: usize) -> Option<SlideResult> {
        let c = self.tiles[i];
        if c == Tile::Empty {
            None
        } else {
            let r = self.slide_next(dir, i, c);
            if r.position == i {
                None
            } else {
                Some(r)
            }
        }
    }

    //Finds result of sliding tile in direction dir, always returns a slide result
    //(slide result may be a 0-tile slide to the same position)
    fn slide_next(&self, dir: Direction, i: usize, t: Tile) -> SlideResult {
        match self.tile_to(dir, i) {
            //If tile is empty keep going
            Some((n, Tile::Empty)) => self.slide_next(dir, n, t),
            //If tile exists and is the same value merge
            Some((n, o)) if t.can_merge(o) => {
                let new_tile = t.next();
                SlideResult {
                    position: n,
                    tile: new_tile,
                    score: new_tile.score()
                }
            },
            //Either there is an edge or a non-matching tile
            _ => SlideResult {
                position: i,
                tile: t,
                score: 0
            }
        }
    }
}

fn to_row_col(i: usize) -> (usize, usize) {
    ( i / SIZE, i % SIZE )
}

fn from_row_col(r: usize, c: usize) -> usize {
    r * SIZE + c
}

//Returns Some(position) of the next tile in dir or None if dir is an edge
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
    
