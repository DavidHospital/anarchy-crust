use std::ops::Index;

use crate::bitboard::BitBoard;

enum Player {
    White,
    Black,
}

enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub struct BoardState {
    pieces: [BitBoard; 12],
}

impl BoardState {
    pub fn empty() -> Self {
        BoardState {
            pieces: Default::default(),
        }
    }
}

impl Index<(Player, Piece)> for BoardState {
    type Output = BitBoard;

    fn index(&self, (player, piece): (Player, Piece)) -> &Self::Output {
        let index = match piece {
            Piece::King => 0,
            Piece::Queen => 1,
            Piece::Rook => 2,
            Piece::Bishop => 3,
            Piece::Knight => 4,
            Piece::Pawn => 5,
        };
        &self.pieces[index + if let Player::Black = player { 6 } else { 0 }]
    }
}
