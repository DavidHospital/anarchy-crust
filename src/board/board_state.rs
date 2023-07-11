use std::ops::{Index, IndexMut};

use super::{
    bitboard::BitBoard,
    constants::{
        BLACK_CASTLING_FLAG_MASK, STARTING_FLAGS, STARTING_PIECES, WHTIE_CASTLING_FLAG_MASK,
    },
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

#[derive(Clone, Copy)]
pub struct Piece {
    player: Player,
    piece_type: PieceType,
}

impl Piece {
    pub fn new(player: Player, piece_type: PieceType) -> Self {
        Piece { player, piece_type }
    }

    pub fn player(self) -> Player {
        self.player
    }

    pub fn piece_type(self) -> PieceType {
        self.piece_type
    }
}

#[inline(always)]
fn piece_index(piece: Piece) -> usize {
    (if let Player::Black = piece.player {
        6
    } else {
        0
    }) + match piece.piece_type {
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

    pub fn check_flag(self, flag: BoardStateFlags) -> bool {
        self.flags & flag > 0
    }

    pub fn swap_flags(mut self, flags: BoardStateFlags) -> Self {
        self.flags *= flags;
        self
    }

    pub fn reset_flags(mut self, flags: BoardStateFlags) -> Self {
        self.flags &= !flags;
        self
    }

    pub fn reset_castling_flags(self, player: Player) -> Self {
        match player {
            Player::White => self.reset_flags(WHTIE_CASTLING_FLAG_MASK),
            Player::Black => self.reset_flags(BLACK_CASTLING_FLAG_MASK),
        }
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
