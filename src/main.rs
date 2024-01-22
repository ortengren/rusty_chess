use rusty_chess::bitboard::Bitboard;
use rusty_chess::board::Color::{Black, White};
use rusty_chess::moves;

fn main() {
    let test: Bitboard = Bitboard(268435456);
    print!("{:?}", moves::mask_pawn_attacks(28, Black));
}
