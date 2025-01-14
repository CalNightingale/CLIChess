use crate::board::Board;
use crate::piece::{Color, Piece};

pub struct King {
    color: Color,
}

impl Piece for King {
    fn new(color: Color) -> Self {
        King { color }
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_icon(&self) -> char {
        match self.color {
            Color::White => '♔',
            Color::Black => '♚',
        }
    }

    fn get_available_moves(&self, current_pos: (usize, usize), board: &Board) -> Vec<(i32, i32)> {
        let mut moves = Vec::new();
        let (row, col) = current_pos;
        
        // All possible king moves (8 directions, one square each)
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),  // Top-left, top, top-right
            (0, -1),           (0, 1),    // Left, right
            (1, -1),  (1, 0),  (1, 1),    // Bottom-left, bottom, bottom-right
        ];
        
        for &(delta_row, delta_col) in &directions {
            let new_row = row as i32 + delta_row;
            let new_col = col as i32 + delta_col;
            
            // Check if the move is within board boundaries
            if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                
                // Add move if square is empty or contains enemy piece
                if !board.is_occupied(new_row, new_col) {
                    moves.push((delta_row, delta_col));
                } else if let Some(piece) = board.get_piece_at(new_row, new_col) {
                    if piece.get_color() != self.color {
                        moves.push((delta_row, delta_col));
                    }
                }
            }
        }
        
        // Note: Castling moves will need to be added later
        // They require checking:
        // 1. King hasn't moved
        // 2. Rook hasn't moved
        // 3. No pieces between king and rook
        // 4. King is not in check
        // 5. King doesn't pass through attacked squares
        
        moves
    }
} 