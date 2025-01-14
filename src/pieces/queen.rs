use crate::board::Board;
use crate::piece::{Color, Piece};

pub struct Queen {
    color: Color,
}

impl Piece for Queen {
    fn new(color: Color) -> Self {
        Queen { color }
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn get_icon(&self) -> char {
        match self.color {
            Color::White => '♕',
            Color::Black => '♛',
        }
    }

    fn get_available_moves(&self, current_pos: (usize, usize), board: &Board) -> Vec<(i32, i32)> {
        let mut moves = Vec::new();
        let (row, col) = current_pos;
        
        // Define all eight directions (combining rook and bishop movements)
        let directions = [
            // Orthogonal (rook-like)
            (0, 1), (0, -1), (1, 0), (-1, 0),
            // Diagonal (bishop-like)
            (1, 1), (1, -1), (-1, 1), (-1, -1),
        ];
        
        for &(delta_row, delta_col) in &directions {
            let mut distance = 1;
            
            loop {
                let new_row = row as i32 + (delta_row * distance);
                let new_col = col as i32 + (delta_col * distance);
                
                // Check if we're still on the board
                if new_row < 0 || new_row >= 8 || new_col < 0 || new_col >= 8 {
                    break;
                }
                
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                
                // If the square is empty, add it as a valid move
                if !board.is_occupied(new_row, new_col) {
                    moves.push((delta_row * distance, delta_col * distance));
                } else {
                    // If there's a piece, check if it's capturable (opposite color)
                    if let Some(piece) = board.get_piece_at(new_row, new_col) {
                        if piece.get_color() != self.color {
                            moves.push((delta_row * distance, delta_col * distance));
                        }
                    }
                    break; // Stop checking this direction after encountering any piece
                }
                
                distance += 1;
            }
        }
        
        moves
    }
} 