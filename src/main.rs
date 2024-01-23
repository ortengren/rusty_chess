use rusty_chess::bitboard::Bitboard;
use rusty_chess::tables;

fn main() {
    let blockers = Bitboard(4535485481984);
    println!("{:?}", blockers);
    print!("{:?}", tables::otf_bishop_attacks(28, blockers));
}
