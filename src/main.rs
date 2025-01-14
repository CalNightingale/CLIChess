mod board;
mod constants;
mod piece;

use board::{Board, Turn};
use std::io::{self, Write};

fn main() {
    let mut board = Board::new();
    
    loop {
        // Clear screen (Unix/Linux/MacOS)
        print!("\x1B[2J\x1B[1;1H");
        
        board.display();
        println!("\n{}'s turn", if matches!(board.current_turn, Turn::White) { "WHITE" } else { "BLACK" });
        
        print!("Select piece (e.g. 'e2') or 'q' to quit: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        
        if input == "q" {
            break;
        }
        
        if board.select_piece(&input) {
            if let Some(moves) = board.get_piece_moves(&input) {
                print!("\x1B[2J\x1B[1;1H");
                board.display_with_moves(&moves);
                println!("\nValid moves are highlighted in green.");
                print!("Enter move destination or 'c' to cancel: ");
                io::stdout().flush().unwrap();
                
                let mut dest = String::new();
                io::stdin().read_line(&mut dest).unwrap();
                let dest = dest.trim().to_lowercase();
                
                if dest != "c" {
                    if board.make_move(&input, &dest) {
                        continue;
                    }
                    println!("Invalid move! Press Enter to continue...");
                    io::stdin().read_line(&mut String::new()).unwrap();
                }
            }
        } else {
            println!("Invalid selection. Press Enter to continue...");
            io::stdin().read_line(&mut String::new()).unwrap();
        }
    }
}
