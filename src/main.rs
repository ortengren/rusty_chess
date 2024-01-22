use rusty_chess::bitboard::Bitboard;

fn main() {
    let test: Bitboard = Bitboard(134217728);
    print!("{:?}", test);
}
