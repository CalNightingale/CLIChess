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