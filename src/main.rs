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

        if !b.has_possible_moves() {
            break;
        }

        let command_result = match ui.next_command() {
            Command::Quit => break,
            Command::Move(d) => b.move_board(d),
            Command::Restart => Some(Board::new()),
            Command::Back => {
                if let Some(r) = boards.pop() {
                    b=r
                }
                None
            },
        };

        if let Some(r) = command_result {
            if let Some(r) = r.add_tile() {
                boards.push(b);
                b=r;
            } else {
                break;
            }
        }
    }
    
    ui.game_over();

}
