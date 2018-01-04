extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{RawTerminal, IntoRawMode};
use std::io::{Write, Stdout, stdout, stdin};

mod board;
use board::{Board, Direction};

struct UI {
    stdout: RawTerminal<Stdout>,
}

impl UI {
    fn new() -> UI {
        UI {
            stdout: stdout().into_raw_mode().unwrap()
        }
    }

    fn draw(&mut self, b: &Board) {
        write!(self.stdout, "{}{}{}{}", termion::clear::All, termion::cursor::Goto(1,1), termion::cursor::Hide, b).unwrap();
        self.stdout.flush().unwrap();
    }

    fn invalid(&mut self) {
        write!(self.stdout, "Invalid").unwrap();
        self.stdout.flush().unwrap();
    }
}

impl std::ops::Drop for UI {
    fn drop(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Show).unwrap()
    }
}

fn main() {
    let mut ui=UI::new();

    let mut b = Board::new().add_tile().unwrap();

    ui.draw(&b);

    for c in stdin().keys() {
        let move_result = match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('h') => b.move_board(Direction::Left),
            Key::Char('j') => b.move_board(Direction::Down),
            Key::Char('k') => b.move_board(Direction::Up),
            Key::Char('l') => b.move_board(Direction::Right),
            _ => { ui.invalid(); None }
        };

        if let Some(r) = move_result {
            if let Some(r) = r.add_tile() {
                ui.draw(&r);
                b = r;
            } else {
                break;
            }
        }
    }

}
