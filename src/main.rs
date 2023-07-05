use bitboard::BitBoard;

use crate::board::{BoardState, Piece, Player};

mod bitboard;
mod board;

fn main() {
    let mut board = BoardState::empty();
    println!("{:?}", board);

    board[(Player::White, Piece::Knight)] = BitBoard::new(37);
    println!("{}", board[(Player::White, Piece::Knight)]);
    println!("{:?}", board);
}
