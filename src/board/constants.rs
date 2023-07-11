use super::{
    bitboard::BitBoard,
    board_state::{BoardStateFlags, BoardStatePieces},
};

// Starting position
pub const STARTING_WHITE_KINGS: BitBoard = BitBoard::new(0x00_00_00_00_00_00_00_08);
pub const STARTING_WHITE_QUEENS: BitBoard = BitBoard::new(0x00_00_00_00_00_00_00_10);
pub const STARTING_WHITE_ROOKS: BitBoard = BitBoard::new(0x00_00_00_00_00_00_00_81);
pub const STARTING_WHITE_BISHOPS: BitBoard = BitBoard::new(0x00_00_00_00_00_00_00_24);
pub const STARTING_WHITE_KNIGHTS: BitBoard = BitBoard::new(0x00_00_00_00_00_00_00_42);
pub const STARTING_WHITE_PAWNS: BitBoard = BitBoard::new(0x00_00_00_00_00_00_FF_00);
pub const STARTING_BLACK_KINGS: BitBoard = BitBoard::new(0x08_00_00_00_00_00_00_00);
pub const STARTING_BLACK_QUEENS: BitBoard = BitBoard::new(0x10_00_00_00_00_00_00_00);
pub const STARTING_BLACK_ROOKS: BitBoard = BitBoard::new(0x81_00_00_00_00_00_00_00);
pub const STARTING_BLACK_BISHOPS: BitBoard = BitBoard::new(0x24_00_00_00_00_00_00_00);
pub const STARTING_BLACK_KNIGHTS: BitBoard = BitBoard::new(0x42_00_00_00_00_00_00_00);
pub const STARTING_BLACK_PAWNS: BitBoard = BitBoard::new(0x00_FF_00_00_00_00_00_00);

pub const STARTING_PIECES: BoardStatePieces = [
    STARTING_WHITE_KINGS,
    STARTING_WHITE_QUEENS,
    STARTING_WHITE_ROOKS,
    STARTING_WHITE_BISHOPS,
    STARTING_WHITE_KNIGHTS,
    STARTING_WHITE_PAWNS,
    STARTING_BLACK_KINGS,
    STARTING_BLACK_QUEENS,
    STARTING_BLACK_ROOKS,
    STARTING_BLACK_BISHOPS,
    STARTING_BLACK_KNIGHTS,
    STARTING_BLACK_PAWNS,
];

// flags
pub const CASTLING_FLAGS: [BoardStateFlags; 4] = [0x08_00, 0x04_00, 0x02_00, 0x01_00];

// Flag masks
pub const TURN_FLAG_MASK: BoardStateFlags = 0x80_00;
pub const FILES_FLAG_MASK: BoardStateFlags = 0x00_FF;
pub const WHTIE_CASTLING_FLAG_MASK: BoardStateFlags = CASTLING_FLAGS[0] | CASTLING_FLAGS[1];
pub const BLACK_CASTLING_FLAG_MASK: BoardStateFlags = CASTLING_FLAGS[2] | CASTLING_FLAGS[3];

// Starting flags
pub const STARTING_TURN_PLAYER_FLAG: BoardStateFlags = 0x80_00;
pub const STARTING_CASTLING_FLAGS: BoardStateFlags = 0x0F_00;
pub const STARTING_FILES_FLAGS: BoardStateFlags = 0x00_00;
pub const STARTING_FLAGS: BoardStateFlags =
    STARTING_TURN_PLAYER_FLAG | STARTING_CASTLING_FLAGS | STARTING_FILES_FLAGS;
