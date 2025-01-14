use crate::piece::{Piece, Color};
use crate::pieces::pawn::Pawn;
use crate::pieces::rook::Rook;
use crate::pieces::knight::Knight;
use crate::constants::*;  // Import all constants

#[derive(Copy, Clone)]
pub enum Turn {
    White,
    Black,
}

impl Turn {
    fn toggle(&self) -> Self {
        match self {
            Turn::White => Turn::Black,
            Turn::Black => Turn::White,
        }
    }
}

pub struct Board {
    grid: [[Option<Box<dyn Piece>>; 8]; 8],
    pub current_turn: Turn,
}

impl Board {
    pub fn new() -> Self {
        // Initialize empty grid using array::from_fn
        let grid = std::array::from_fn(|_| {
            std::array::from_fn(|_| None)
        });
        
        let mut board = Self { grid, current_turn: Turn::White };
        
        // Place pawns
        for i in 0..8 {
            board.grid[1][i] = Some(Box::new(Pawn::new(Color::White)));
            board.grid[6][i] = Some(Box::new(Pawn::new(Color::Black)));
        }

        // Place rooks
        board.grid[0][0] = Some(Box::new(Rook::new(Color::White)));
        board.grid[0][7] = Some(Box::new(Rook::new(Color::White)));
        board.grid[7][0] = Some(Box::new(Rook::new(Color::Black)));
        board.grid[7][7] = Some(Box::new(Rook::new(Color::Black)));

        // Place knights
        board.grid[0][1] = Some(Box::new(Knight::new(Color::White)));
        board.grid[0][6] = Some(Box::new(Knight::new(Color::White)));
        board.grid[7][1] = Some(Box::new(Knight::new(Color::Black)));
        board.grid[7][6] = Some(Box::new(Knight::new(Color::Black)));

        // Temporarily comment out other pieces until we implement them
        // We'll just start with pawns for now
        
        board
    }

    pub fn display(&self) {
        for row in (0..8).rev() {
            print!("{} ", row + 1); // Print rank numbers
            for col in 0..8 {
                let background = if (row + col) % 2 == 0 {
                    GRAY_BG
                } else {
                    PURPLE_BG
                };

                match &self.grid[row][col] {
                    Some(piece) => {
                        let color = if piece.get_color() == Color::White {
                            WHITE_PIECE
                        } else {
                            BLACK_PIECE
                        };
                        print!("{}{} {} {}", background, color, piece.get_icon(), RESET);
                    }
                    None => print!("{} {} {}", background, " ", RESET),
                }
            }
            println!();
        }
        // Print file letters
        println!("   a  b  c  d  e  f  g  h");
    }

    fn parse_position(input: &str) -> Option<(usize, usize)> {
        if input.len() != 2 {
            return None;
        }
        
        let mut chars = input.chars();
        let file = chars.next()?;
        let rank = chars.next()?;
        
        if !('a'..='h').contains(&file) || !('1'..='8').contains(&rank) {
            return None;
        }
        
        let col = (file as u8 - b'a') as usize;
        let row = (rank as u8 - b'1') as usize;
        
        Some((row, col))
    }
    
    pub fn select_piece(&self, position: &str) -> bool {
        if let Some((row, col)) = Self::parse_position(position) {
            if let Some(piece) = &self.grid[row][col] {
                let piece_color = piece.get_color();
                match self.current_turn {
                    Turn::White => piece_color == Color::White,
                    Turn::Black => piece_color == Color::Black,
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn display_with_moves(&self, moves: &[(usize, usize)]) {
        for row in (0..8).rev() {
            print!("{} ", row + 1);
            for col in 0..8 {
                let is_move = moves.contains(&(row, col));
                let background = if is_move {
                    GREEN_BG
                } else if (row + col) % 2 == 0 {
                    GRAY_BG
                } else {
                    PURPLE_BG
                };

                match &self.grid[row][col] {
                    Some(piece) => {
                        let color = if piece.get_color() == Color::White {
                            WHITE_PIECE
                        } else {
                            BLACK_PIECE
                        };
                        print!("{}{} {} {}", background, color, piece.get_icon(), RESET);
                    }
                    None => print!("{} {} {}", background, " ", RESET),
                }
            }
            println!();
        }
        println!("   a  b  c  d  e  f  g  h");
    }

    pub fn get_piece_moves(&self, position: &str) -> Option<Vec<(usize, usize)>> {
        if let Some((row, col)) = Self::parse_position(position) {
            if let Some(piece) = &self.grid[row][col] {
                // Pass current position and board reference to get_available_moves
                let moves = piece.get_available_moves((row, col), self)
                    .into_iter()
                    .map(|(r, c)| ((row as i32 + r) as usize, (col as i32 + c) as usize))
                    .filter(|(r, c)| *r < 8 && *c < 8)
                    .collect();
                return Some(moves);
            }
        }
        None
    }

    pub fn make_move(&mut self, from: &str, to: &str) -> bool {
        if let (Some((from_row, from_col)), Some((to_row, to_col))) = 
            (Self::parse_position(from), Self::parse_position(to)) {
            
            // First get the valid moves for the piece
            if let Some(valid_moves) = self.get_piece_moves(from) {
                // Check if the destination is in the valid moves list
                if !valid_moves.contains(&(to_row, to_col)) {
                    return false;
                }
                
                // Move is valid, proceed with the move
                if let Some(piece) = self.grid[from_row][from_col].take() {
                    self.grid[to_row][to_col] = Some(piece);
                    self.current_turn = self.current_turn.toggle();
                    return true;
                }
            }
        }
        false
    }

    pub fn is_occupied(&self, row: usize, col: usize) -> bool {
        self.grid[row][col].is_some()
    }

    pub fn get_piece_at(&self, row: usize, col: usize) -> Option<&Box<dyn Piece>> {
        self.grid[row][col].as_ref()
    }
} 