use std::ops::{Index, IndexMut};

use super::bitboard::BitBoard;

pub enum Player {
    White,
    Black,
}

pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[inline(always)]
fn piece_index(player: Player, piece: Piece) -> usize {
    (if let Player::Black = player { 6 } else { 0 })
        + match piece {
            Piece::King => 0,
            Piece::Queen => 1,
            Piece::Rook => 2,
            Piece::Bishop => 3,
            Piece::Knight => 4,
            Piece::Pawn => 5,
        }
}

pub type BoardStatePieces = [BitBoard; 12];

#[derive(Debug, Clone, Copy)]
pub struct BoardState {
    pieces: BoardStatePieces,
}

impl BoardState {
    pub const fn new(pieces: BoardStatePieces) -> Self {
        BoardState { pieces }
    }
}

impl Index<(Player, Piece)> for BoardState {
    type Output = BitBoard;

    fn index(&self, (player, piece): (Player, Piece)) -> &Self::Output {
        &self.pieces[piece_index(player, piece)]
    }
}

impl IndexMut<(Player, Piece)> for BoardState {
    fn index_mut(&mut self, (player, piece): (Player, Piece)) -> &mut Self::Output {
        &mut self.pieces[piece_index(player, piece)]
    }
}
