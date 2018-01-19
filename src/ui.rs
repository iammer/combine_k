pub extern crate termion;

use self::termion::event::Key;
use self::termion::input::{TermRead, Keys};
use self::termion::raw::{RawTerminal, IntoRawMode};
use std::io::{Write, Stdout, Stdin, stdout, stdin};
use std::ops::Drop;

use board::{Direction, Board};

pub struct UI {
    stdout: RawTerminal<Stdout>,
    keys: Keys<Stdin>
}

pub enum Command {
    Move(Direction),
    Back,
    Quit,
    Restart
}

impl UI {
    pub fn new() -> UI {
        UI {
            stdout: stdout().into_raw_mode().unwrap(),
            keys: stdin().keys()
        }
    }

    pub fn next_command(&mut self) -> Command {
        match self.keys.next() {
            None => Command::Quit,
            Some(Err(_)) => Command::Quit,
            Some(Ok(Key::Char('h'))) => Command::Move(Direction::Left),
            Some(Ok(Key::Char('j'))) => Command::Move(Direction::Down),
            Some(Ok(Key::Char('k'))) => Command::Move(Direction::Up),
            Some(Ok(Key::Char('l'))) => Command::Move(Direction::Right),
            Some(Ok(Key::Char('q'))) => Command::Quit,
            Some(Ok(Key::Char('b'))) => Command::Back,
            Some(Ok(Key::Char('r'))) => Command::Restart,
            Some(Ok(_)) => { self.invalid(); self.next_command() }
        }
    }

    pub fn draw(&mut self, b: &Board) {
        write!(self.stdout, "{}{}{}{}", termion::clear::All, termion::cursor::Goto(1,1), termion::cursor::Hide, b).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn invalid(&mut self) {
        write!(self.stdout, "Invalid").unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn game_over(&mut self) {
        write!(self.stdout, "Game Over!\r\n").unwrap();
        self.stdout.flush().unwrap();
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Show).unwrap()
    }
}

