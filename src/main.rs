use board::constants::STARTING_PIECES;

use crate::board::board::{BoardState, Piece, Player};

mod board {
    pub mod bitboard;
    pub mod board;
    pub mod constants;
}

fn main() {
    let board = BoardState::new(STARTING_PIECES);
    println!("{}", board[(Player::White, Piece::Knight)]);
}
