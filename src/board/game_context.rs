use super::bitboard::BitBoard;

pub const WHITE_KINGSIDE_ROOK: BitBoard = BitBoard::new(0x00_00_00_00_00_00_00_01);
pub const WHITE_QUEENSIDE_ROOK: BitBoard = BitBoard::new(0x00_00_00_00_00_00_00_80);
pub const BLACK_KINGSIDE_ROOK: BitBoard = BitBoard::new(0x01_00_00_00_00_00_00_00);
pub const BLACK_QUEENSIDE_ROOK: BitBoard = BitBoard::new(0x80_00_00_00_00_00_00_00);

#[derive(Clone, Copy)]
pub struct GameContext {
    pub rooks: [BitBoard; 4],
}

impl Default for GameContext {
    fn default() -> Self {
        Self {
            rooks: [
                WHITE_KINGSIDE_ROOK,
                WHITE_QUEENSIDE_ROOK,
                BLACK_KINGSIDE_ROOK,
                BLACK_QUEENSIDE_ROOK,
            ],
        }
    }
}
