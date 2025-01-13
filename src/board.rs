use crate::piece::{Piece, Color, Pawn};
use crate::constants::*;  // Import all constants

pub struct Board {
    grid: [[Option<Box<dyn Piece>>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        // Initialize empty grid using array::from_fn
        let grid = std::array::from_fn(|_| {
            std::array::from_fn(|_| None)
        });
        
        let mut board = Self { grid };
        
        // Place pawns
        for i in 0..8 {
            board.grid[1][i] = Some(Box::new(Pawn::new(Color::White)));
            board.grid[6][i] = Some(Box::new(Pawn::new(Color::Black)));
        }

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
} 