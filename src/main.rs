use crate::board::{
    board::{Piece, Player},
    constants::STARTING_BOARD,
};

mod board {
    pub mod bitboard;
    pub mod board;
    pub mod constants;
}

fn main() {
    let board = STARTING_BOARD;
    println!("{}", board[(Player::White, Piece::Knight)]);
}
