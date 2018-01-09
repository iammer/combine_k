use std::fmt;
use ui::termion::color;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Occupied(u8)
}

const ASCII_START: u8 = ('A' as u8) - 1; 

impl Tile {
    pub fn next(&self) -> Tile {
        match *self {
            Tile::Occupied(n) => Tile::Occupied(n+1),
            x => x
        }
    }

    pub fn score(&self) -> u32 {
        match *self {
            Tile::Occupied(x) => 2u32.pow(x as u32),
            _ => 0
        }
    }

}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", color::Fg(*self), match *self {
            Tile::Empty => ' ',
            Tile::Occupied(c) if c > 0 && c <= 27 => (ASCII_START + c) as char,
            Tile::Occupied(_) => '?'
        }, color::Fg(color::Reset))
    }
}

impl color::Color for Tile {
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tile::Occupied(1) => color::Blue.write_fg(f),
            Tile::Occupied(2) => color::Red.write_fg(f),
            Tile::Occupied(3) => color::Green.write_fg(f),
            Tile::Occupied(4) => color::Cyan.write_fg(f),
            Tile::Occupied(5) => color::Magenta.write_fg(f),
            Tile::Occupied(6) => color::Yellow.write_fg(f),
            Tile::Occupied(7) => color::LightBlue.write_fg(f),
            Tile::Occupied(8) => color::LightRed.write_fg(f),
            Tile::Occupied(9) => color::LightGreen.write_fg(f),
            Tile::Occupied(10) => color::LightCyan.write_fg(f),
            Tile::Occupied(11) => color::LightMagenta.write_fg(f),
            Tile::Occupied(12) => color::LightYellow.write_fg(f),
            _ => color::White.write_fg(f)
        }
    }

    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        color::Black.write_bg(f)
    }
}


