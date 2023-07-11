use crate::board::{board_state::BoardState, game_context::GameContext};

use super::{
    castling::update_castling_flags,
    moves::{AtomicMove, MoveInternal},
};

#[derive(Clone, Copy)]
pub struct SimpleMove<const N: usize> {
    moves: [AtomicMove; N],
}

impl<const N: usize> SimpleMove<N> {
    fn moves(self) -> [AtomicMove; N] {
        self.moves
    }
}

impl<const N: usize> MoveInternal for SimpleMove<N> {
    fn move_pieces(self, mut board: BoardState) -> BoardState {
        for mv in self.moves().iter() {
            board = mv.mv_on(board);
        }
        board
    }

    fn update_flags(self, board: BoardState, context: GameContext) -> BoardState {
        self.moves
            .iter()
            .fold(board, |b, m| update_castling_flags(*m, b, context))
    }
}
