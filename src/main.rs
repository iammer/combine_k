mod board;
use board::Board;

mod ui;
use ui::{UI, Command};

fn main() {
    let mut ui=UI::new();
    let mut boards: Vec<Board> = Vec::new();

    let mut b = Board::new().add_tile().unwrap();

    loop {
        ui.draw(&b);

        if let Some(r) = match ui.next_command() {
            Command::Quit => break,
            Command::Move(d) => b.move_board(d),
            Command::Back => {
                if let Some(r) = boards.pop() {
                    b=r
                }
                None
            },
            Command::Restart => Some(Board::new())
        } {
            if let Some(r) = r.add_tile() {
                boards.push(b);
                b=r;
            } else {
                break;
            }
        }
    }

}
