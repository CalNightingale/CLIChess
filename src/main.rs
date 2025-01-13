mod board;
mod constants;
use board::Board;

fn main() {
    let board = Board::new();
    board.display();
}
