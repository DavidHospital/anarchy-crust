use board::board_state::{BoardState, Piece, PieceType, Player};

mod board;

fn main() {
    let board = BoardState::starting();
    println!("{}\n", board[Piece::new(Player::White, PieceType::Rook)]);
    println!("{}\n", board[Piece::new(Player::White, PieceType::King)]);
}
