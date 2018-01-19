extern crate rand;

use std::fmt;

#[cfg(test)]
mod tests;

mod tile;
pub use self::tile::Tile;

const SIZE: usize = 4;
const LENGTH: usize = SIZE * SIZE;
const NEW_TILE_RATIO: f32 = 0.9;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up, Down, Left, Right
}

struct TileMovement {
    tile: Tile,
    new_position: usize
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
            tiles: vec![Tile::Empty; LENGTH],
            score: 0
        }
    }

    //Adds WillOccupy tiles to score and transforms WillOccupy to Occupied Tiles
    fn finalize(self) -> Self {
        let score: u32 = self.tiles.iter().map(|t| t.score()).sum();
        Board {
            tiles: self.tiles.into_iter().map(|t| t.finalize()).collect(),
            score: self.score + score
        }
    }

    pub fn can_move(&self, dir: Direction) -> bool {
        let mut board = self.clone();

        scan_sequence(dir).any(|i| board.move_tile(dir, i))
    }

    //Moves all tiles on the board in direction m
    pub fn move_board(&self, dir: Direction) -> Option<Self> {
        let mut board = self.clone();

        let did_move = scan_sequence(dir)
            .map(|i| board.move_tile(dir,i))
            .collect::<Vec<bool>>() //Collect and into_iter to prevent any from short-circuting
            .into_iter().any(|x| x);

        if did_move {
            Some(board.finalize())
        } else {
            None
        }
    }

    pub fn empty_tiles(&self) -> Vec<usize> {
        self.tiles.iter()
            .enumerate()
            .filter(|&(_, &x)| x == Tile::Empty)
            .map(|(i,_)| i)
            .collect()
    }

    //Adds a new tile to board in a random empty space
    pub fn add_tile(&self) -> Option<Self> {
        let empty_tiles =  self.empty_tiles();
        let empty_count: usize = empty_tiles.len();
        if empty_count > 0 {
            let mut board = self.clone();
            let selected = rand::random::<usize>() % empty_count;
            let value: u8 = if rand::random::<f32>() > NEW_TILE_RATIO { 2 } else { 1 };

            board.tiles[empty_tiles[selected]] = Tile::Occupied(value);

            Some(board)
        } else {
            None
        }
    }

    //Slides a single tile in direction returns true if tile was moved
    fn move_tile(&mut self, dir: Direction, i: usize) -> bool {
        let c = self.tiles[i];

        if c == Tile::Empty {
            false
        } else {
            let r = self.find_tile_movement(dir, i, c);
            if r.new_position == i {
                false
            } else {
                self.tiles[i] = Tile::Empty;
                self.tiles[r.new_position] = r.tile;
                true
            }
        }
    }

    //Finds next tile in dir
    //Returns None if no tile (dir is edge)
    //Returns Some((position, tile)) if tile exists
    fn tile_to(&self, dir: Direction, i: usize) -> Option<(usize, Tile)> {
        space_to(dir, i).map(|n| (n, self.tiles[n]))
    }

    //Finds result of sliding tile in direction dir, always returns a slide result
    //(slide result may be a 0-tile slide to the same position)
    fn find_tile_movement(&self, dir: Direction, i: usize, t: Tile) -> TileMovement {
        match self.tile_to(dir, i) {
            //If tile is empty keep going
            Some((n, Tile::Empty)) => self.find_tile_movement(dir, n, t),
            //If tile exists and is the same value merge
            Some((n, o)) if t.can_merge(o) => {
                TileMovement {
                    new_position: n,
                    tile: t.next()
                }
            },
            //Either there is an edge or a non-matching tile
            _ => TileMovement {
                new_position: i,
                tile: t
            }
        }
    }

    pub fn has_possible_moves(&self) -> bool {
        self.empty_tiles().len() > 0 ||
        self.can_move(Direction::Up) ||
        self.can_move(Direction::Down) ||
        self.can_move(Direction::Left) ||
        self.can_move(Direction::Right)
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

//Returns a range indicating the order to iterate tile indexes
//(Down and Right should be iterated in reverse order)
fn scan_sequence(dir: Direction) -> Box<Iterator<Item=usize>> {
    if dir == Direction::Up || dir == Direction::Left {
        Box::new((0..LENGTH))
    } else {
        Box::new((0..LENGTH).rev())
    }
}
    
