use crate::board::{
    bitboard::BitBoard,
    board_state::{BoardState, Piece},
    game_context::GameContext,
};

#[derive(Clone, Copy)]
pub struct AtomicMove {
    piece: Piece,
    mv: BitBoard,
}

impl AtomicMove {
    pub fn mv_on(self, mut board: BoardState) -> BoardState {
        board[self.piece] ^= self.mv;
        board
    }

    pub fn get_piece(self) -> Piece {
        self.piece
    }

    pub fn get_move(self) -> BitBoard {
        self.mv
    }
}

pub trait MoveInternal: Sized + Copy {
    fn move_pieces(self, board: BoardState) -> BoardState;
    fn update_flags(self, board: BoardState, context: GameContext) -> BoardState;
}

pub trait Move: Sized + Copy {
    fn perform_move_on(self, board: BoardState, context: GameContext) -> BoardState;
}

impl<T: MoveInternal> Move for T {
    fn perform_move_on(self, board: BoardState, context: GameContext) -> BoardState {
        let board = self.move_pieces(board);
        self.update_flags(board, context)
    }
}
