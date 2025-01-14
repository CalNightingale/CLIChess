use crate::board::Board;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

pub trait Piece {
    fn new(color: Color) -> Self where Self: Sized;
    fn get_color(&self) -> Color;
    fn get_icon(&self) -> char;
    fn get_available_moves(&self, current_pos: (usize, usize), board: &Board) -> Vec<(i32, i32)>;
}

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