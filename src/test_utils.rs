use crate::board::Board;
use crate::piece::Piece;

pub struct TestBoard {
    pub board: Board,
}

impl TestBoard {
    pub fn empty() -> Self {
        Self {
            board: Board::new()
        }
    }

    pub fn place_piece(&mut self, piece: Box<dyn Piece>, pos: (usize, usize)) {
        let (row, col) = pos;
        self.board.place_piece_at(piece, row, col);
    }

    pub fn assert_valid_move(&self, from: (usize, usize), to: (usize, usize)) {
        assert!(self.board.is_valid_move(from, to), 
            "Expected move from {:?} to {:?} to be valid", from, to);
    }

    pub fn assert_invalid_move(&self, from: (usize, usize), to: (usize, usize)) {
        assert!(!self.board.is_valid_move(from, to),
            "Expected move from {:?} to {:?} to be invalid", from, to);
    }
} 