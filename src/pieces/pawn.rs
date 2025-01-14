use crate::board::Board;
use crate::piece::{Color, Piece};

pub struct Pawn {
    color: Color,
}

impl Piece for Pawn {
    fn new(color: Color) -> Self {
        Pawn { color }
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_icon(&self) -> char {
        match self.color {
            Color::White => '♙',
            Color::Black => '♟',
        }
    }

    fn get_available_moves(&self, current_pos: (usize, usize), board: &Board) -> Vec<(i32, i32)> {
        let mut moves = Vec::new();
        let (row, col) = current_pos;
        let direction = match self.color {
            Color::White => 1,
            Color::Black => -1,
        };

        // Check forward move
        let forward_pos = ((row as i32 + direction), col as i32);
        if forward_pos.0 >= 0 && forward_pos.0 < 8 && !board.is_occupied(forward_pos.0 as usize, forward_pos.1 as usize) {
            moves.push((direction, 0));
            
            // Initial two-square move
            let starting_rank = if self.color == Color::White { 1 } else { 6 };
            if row == starting_rank {
                let double_forward = ((row as i32 + 2 * direction), col as i32);
                if !board.is_occupied(double_forward.0 as usize, double_forward.1 as usize) {
                    moves.push((2 * direction, 0));
                }
            }
        }

        // Check diagonal captures
        for col_offset in [-1, 1] {
            let capture_pos = ((row as i32 + direction), col as i32 + col_offset);
            if capture_pos.0 >= 0 && capture_pos.0 < 8 && capture_pos.1 >= 0 && capture_pos.1 < 8 {
                if let Some(piece) = board.get_piece_at(capture_pos.0 as usize, capture_pos.1 as usize) {
                    if piece.get_color() != self.color {
                        moves.push((direction, col_offset));
                    }
                }
            }
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestBoard;

    #[test]
    fn test_pawn_movement() {
        let mut test_board = TestBoard::empty();
        
        // Place white pawn for testing
        test_board.place_piece(Box::new(Pawn::new(Color::White)), (1, 1));
        
        // Test valid moves
        test_board.assert_valid_move((1, 1), (2, 1));   // One square forward
        test_board.assert_valid_move((1, 1), (3, 1));   // Two squares forward
        
        // Test invalid moves
        test_board.assert_invalid_move((1, 1), (1, 2)); // Sideways movement
        test_board.assert_invalid_move((1, 1), (0, 1)); // Backward movement
    }

    #[test]
    fn test_pawn_captures() {
        let mut test_board = TestBoard::empty();
        
        // Place white pawn and black pieces for capture
        test_board.place_piece(Box::new(Pawn::new(Color::White)), (3, 3));
        test_board.place_piece(Box::new(Pawn::new(Color::Black)), (4, 2));
        test_board.place_piece(Box::new(Pawn::new(Color::Black)), (4, 4));
        
        // Test diagonal captures
        test_board.assert_valid_move((3, 3), (4, 2));   // Capture left
        test_board.assert_valid_move((3, 3), (4, 4));   // Capture right
    }

    #[test]
    fn test_pawn_blocked_movement() {
        let mut test_board = TestBoard::empty();
        
        // Test blocked forward movement
        test_board.place_piece(Box::new(Pawn::new(Color::White)), (1, 1));
        test_board.place_piece(Box::new(Pawn::new(Color::Black)), (2, 1));
        
        test_board.assert_invalid_move((1, 1), (2, 1)); // Blocked by piece
        test_board.assert_invalid_move((1, 1), (3, 1)); // Can't jump over piece
        
        // Test blocked by friendly piece
        test_board.place_piece(Box::new(Pawn::new(Color::White)), (1, 3));
        test_board.place_piece(Box::new(Pawn::new(Color::White)), (2, 4));
        
        test_board.assert_invalid_move((1, 3), (2, 4)); // Can't capture friendly piece
    }
} 