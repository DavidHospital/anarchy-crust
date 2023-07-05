use bitboard::BitBoard;

mod bitboard;
mod board;

fn main() {
    let b = BitBoard::new(27);
    println!("{b}");
}
