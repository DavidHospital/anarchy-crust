use std::ops::BitAnd;

use super::{
    bitboard::BitBoard,
    board::{BoardState, Piece, PieceType, Player},
    constants::{
        BLACK_CASTLING_FLAG_MASK, BLACK_KINGSIDE_CASTLING_FLAG, BLACK_KINGSIDE_ROOK,
        BLACK_QUEENSIDE_CASTLING_FLAG, BLACK_QUEENSIDE_ROOK, RANKS_FLAG_MASK, TURN_FLAG_MASK,
        WHITE_KINGSIDE_CASTLING_FLAG, WHITE_KINGSIDE_ROOK, WHITE_QUEENSIDE_CASTLING_FLAG,
        WHITE_QUEENSIDE_ROOK, WHTIE_CASTLING_FLAG_MASK,
    },
};

#[derive(Clone, Copy)]
struct AtomicMove {
    piece: Piece,
    mv: BitBoard,
}

impl AtomicMove {
    fn mv_on(self, mut board: BoardState) -> BoardState {
        board[self.piece] ^= self.mv;
        board
    }
}

#[derive(Clone, Copy)]
struct SingleMove(AtomicMove);

impl MoveInternal for SingleMove {
    fn _mv_pieces(self, board: BoardState) -> BoardState {
        let board = self.0.mv_on(board);
        match self.0.piece {
            (player, PieceType::King) => board.reset_castling_flags(player),
            (player, PieceType::Rook) => {
                let (kingside_rook, queenside_rook, kingside_flag, queenside_flag) = match player {
                    Player::White => (
                        WHITE_KINGSIDE_ROOK,
                        WHITE_QUEENSIDE_ROOK,
                        WHITE_KINGSIDE_CASTLING_FLAG,
                        WHITE_QUEENSIDE_CASTLING_FLAG,
                    ),
                    Player::Black => (
                        BLACK_KINGSIDE_ROOK,
                        BLACK_QUEENSIDE_ROOK,
                        BLACK_KINGSIDE_CASTLING_FLAG,
                        BLACK_QUEENSIDE_CASTLING_FLAG,
                    ),
                };
                if self.0.mv.bitand(kingside_rook).is_set() {
                    board.reset_flags(kingside_flag)
                } else if self.0.mv.bitand(queenside_rook).is_set() {
                    board.reset_flags(queenside_flag)
                } else {
                    board
                }
            }
            _ => board,
        }
    }
}

#[derive(Clone, Copy)]
struct CaptureMove {
    mv: AtomicMove,
    capture: AtomicMove,
}

impl MoveInternal for CaptureMove {
    fn _mv_pieces(self, board: BoardState) -> BoardState {
        self.capture.mv_on(self.mv.mv_on(board))
    }
}

#[derive(Clone, Copy)]
struct CastlingMove {
    king_mv: AtomicMove,
    rook_mv: AtomicMove,
}

impl MoveInternal for CastlingMove {
    fn _mv_pieces(self, board: BoardState) -> BoardState {
        self.king_mv.mv_on(self.rook_mv.mv_on(board))
    }

    fn _update_flags(self, board: BoardState) -> BoardState {
        board.reset_flags(match self.king_mv.piece.0 {
            Player::White => WHTIE_CASTLING_FLAG_MASK,
            Player::Black => BLACK_CASTLING_FLAG_MASK,
        })
    }
}

trait MoveInternal: Sized + Copy {
    fn _mv_pieces(self, board: BoardState) -> BoardState;
    fn _update_flags(self, board: BoardState) -> BoardState {
        board
    }
}

pub trait Move: Sized + Copy {
    fn perform_mv_on(self, board: BoardState) -> BoardState;
}

impl<T: MoveInternal> Move for T {
    fn perform_mv_on(self, board: BoardState) -> BoardState {
        let board = self._mv_pieces(board);

        // swap turn order, reset ranks flag mask (en passent, knight boost)
        board
            .swap_flags(TURN_FLAG_MASK)
            .reset_flags(RANKS_FLAG_MASK);
        self._update_flags(board)
    }
}
