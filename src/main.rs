use bitboard::BitBoard;

mod bitboard;
mod board;

fn main() {
    let bitboard =
        BitBoard::new(0x00_FF_00_00_00_00_FF_00) | BitBoard::new(0x42_00_00_00_00_00_00_42);
    println!("{}", bitboard);
}
