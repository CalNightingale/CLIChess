mod board;
mod constants;
mod piece;

use board::Board;

fn main() {
    let board = Board::new();
    board.display();
}
