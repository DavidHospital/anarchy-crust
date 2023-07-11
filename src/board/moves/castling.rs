use std::ops::BitAnd;

use crate::board::{
    bitboard::BitBoard,
    board_state::{BoardState, BoardStateFlags, PieceType},
    constants::CASTLING_FLAGS,
    game_context::GameContext,
};

use super::moves::AtomicMove;

pub fn update_castling_flags(
    mv: AtomicMove,
    mut board: BoardState,
    context: GameContext,
) -> BoardState {
    let piece = mv.get_piece();

    if let PieceType::King = piece.piece_type() {
        board = board.reset_castling_flags(piece.player())
    }

    board.reset_flags(
        context
            .rooks
            .iter()
            .zip(CASTLING_FLAGS)
            .fold(0, |acc, (rook, flag)| {
                acc | if board.check_flag(flag) {
                    _update_castling_flag(mv, *rook, flag).unwrap_or_else(|| 0)
                } else {
                    acc
                }
            }),
    )
}

fn _update_castling_flag(
    mv: AtomicMove,
    rook_square: BitBoard,
    flag: BoardStateFlags,
) -> Option<BoardStateFlags> {
    mv.get_move().bitand(rook_square).is_set().then_some(flag)
}
