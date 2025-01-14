#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

pub trait Piece {
    fn new(color: Color) -> Self where Self: Sized;
    fn get_color(&self) -> Color;
    fn get_icon(&self) -> char;
    fn get_available_moves(&self) -> Vec<(i32, i32)>; // Will return list of valid moves as (row, col) coordinates
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

    fn get_available_moves(&self) -> Vec<(i32, i32)> {
        match self.color {
            Color::White => vec![(1, 0)],  // White pawns move up one square
            Color::Black => vec![(-1, 0)], // Black pawns move down one square
        }
    }
} 