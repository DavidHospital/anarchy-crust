use crate::board::{
    bitboard::BitBoard,
    board::{Piece, PieceType, Player},
};

mod board {
    pub mod bitboard;
    pub mod board;
    pub mod constants;
    pub mod moves;
}

fn main() {
    // let mut board = STARTING_BOARD;
    // println!("{}\n", board[(Player::White, PieceType::Rook)]);
    // println!("{}\n", board[(Player::White, PieceType::King)]);

    // let mv = Move2::new(
    //     (Player::White, PieceType::Rook),
    //     BitBoard::new(0x00_00_00_00_00_00_00_01) | BitBoard::new(0x00_00_00_00_00_00_00_04),
    //     (Player::White, PieceType::King),
    //     BitBoard::new(0x00_00_00_00_00_00_00_08) | BitBoard::new(0x00_00_00_00_00_00_00_02),
    // );

    // board = mv.perform_mv_on(board);
    // println!("{}\n", board[(Player::White, PieceType::Rook)]);
    // println!("{}\n", board[(Player::White, PieceType::King)]);
}
