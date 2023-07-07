use std::ops::{Index, IndexMut};

use super::{
    bitboard::BitBoard,
    constants::{STARTING_FLAGS, STARTING_PIECES},
};

#[derive(Clone, Copy)]
pub enum Player {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub type Piece = (Player, PieceType);

#[inline(always)]
fn piece_index((player, piece_type): Piece) -> usize {
    (if let Player::Black = player { 6 } else { 0 })
        + match piece_type {
            PieceType::King => 0,
            PieceType::Queen => 1,
            PieceType::Rook => 2,
            PieceType::Bishop => 3,
            PieceType::Knight => 4,
            PieceType::Pawn => 5,
        }
}

pub type BoardStatePieces = [BitBoard; 12];
pub type BoardStateFlags = u16;

#[derive(Debug, Clone, Copy)]
pub struct BoardState {
    pieces: BoardStatePieces,
    flags: BoardStateFlags,
}

impl BoardState {
    pub const fn new(pieces: BoardStatePieces, flags: BoardStateFlags) -> Self {
        BoardState { pieces, flags }
    }

    pub const fn starting() -> Self {
        BoardState {
            pieces: STARTING_PIECES,
            flags: STARTING_FLAGS,
        }
    }

    pub fn swap_flags(mut self, flags: BoardStateFlags) -> Self {
        self.flags *= flags;
        self
    }

    pub fn reset_flags(mut self, flags: BoardStateFlags) -> Self {
        self.flags &= !flags;
        self
    }
}

impl Index<Piece> for BoardState {
    type Output = BitBoard;

    fn index(&self, piece: Piece) -> &Self::Output {
        &self.pieces[piece_index(piece)]
    }
}

impl IndexMut<Piece> for BoardState {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self.pieces[piece_index(piece)]
    }
}
