use super::{
    bitboard::BitBoard,
    board::{BoardState, Piece, Player},
    constants::{
        BLACK_CASLTING_FLAG_MASK, RANKS_FLAG_MASK, TURN_FLAG_MASK, WHTIE_CASLTING_FLAG_MASK,
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
        self.0.mv_on(board)
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
            Player::White => WHTIE_CASLTING_FLAG_MASK,
            Player::Black => BLACK_CASLTING_FLAG_MASK,
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

        // swap turn order
        board
            .swap_flags(TURN_FLAG_MASK)
            .reset_flags(RANKS_FLAG_MASK);
        self._update_flags(board)
    }
}
