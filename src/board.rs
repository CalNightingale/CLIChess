use crate::constants::*;

pub struct Board {
    grid: [[char; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut grid = [[' '; 8]; 8];
        
        // Place pawns
        for i in 0..8 {
            grid[1][i] = WHITE_PAWN;
            grid[6][i] = BLACK_PAWN;
        }

        // Place rooks
        grid[0][0] = WHITE_ROOK;
        grid[0][7] = WHITE_ROOK;
        grid[7][0] = BLACK_ROOK;
        grid[7][7] = BLACK_ROOK;

        // Place knights
        grid[0][1] = WHITE_KNIGHT;
        grid[0][6] = WHITE_KNIGHT;
        grid[7][1] = BLACK_KNIGHT;
        grid[7][6] = BLACK_KNIGHT;

        // Place bishops
        grid[0][2] = WHITE_BISHOP;
        grid[0][5] = WHITE_BISHOP;
        grid[7][2] = BLACK_BISHOP;
        grid[7][5] = BLACK_BISHOP;

        // Place queens
        grid[0][3] = WHITE_QUEEN;
        grid[7][3] = BLACK_QUEEN;

        // Place kings
        grid[0][4] = WHITE_KING;
        grid[7][4] = BLACK_KING;

        Self { grid }
    }

    pub fn display(&self) {
        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                let bg_color = if (row_idx + col_idx) % 2 == 0 {
                    GRAY_BG
                } else {
                    PURPLE_BG
                };
                
                let piece_color = if cell.is_uppercase() {
                    WHITE_PIECE
                } else {
                    BLACK_PIECE
                };
                
                if *cell == ' ' {
                    print!("{}   {}", bg_color, RESET);
                } else {
                    print!("{}{} {} {}", bg_color, piece_color, cell, RESET);
                }
            }
            println!();
        }
    }
} 