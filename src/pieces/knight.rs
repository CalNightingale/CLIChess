use crate::board::Board;
use crate::piece::{Color, Piece};

pub struct Knight {
    color: Color,
}

impl Piece for Knight {
    fn new(color: Color) -> Self {
        Knight { color }
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_icon(&self) -> char {
        match self.color {
            Color::White => '♘',
            Color::Black => '♞',
        }
    }

    fn get_available_moves(&self, current_pos: (usize, usize), board: &Board) -> Vec<(i32, i32)> {
        let mut moves = Vec::new();
        let (row, col) = current_pos;
        
        // All possible knight moves
        let knight_moves = [
            (-2, -1), (-2, 1),  // Up 2, left/right 1
            (-1, -2), (-1, 2),  // Up 1, left/right 2
            (1, -2), (1, 2),    // Down 1, left/right 2
            (2, -1), (2, 1),    // Down 2, left/right 1
        ];
        
        for &(delta_row, delta_col) in &knight_moves {
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
        
        moves
    }
}
