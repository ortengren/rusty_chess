use crate::board::*;
use crate::game::*;
use crate::bitboard::*;
use crate::position::*;

#[test]
fn test_get_directions() {
    // test square in middle of board
    let d4: Bitboard = Bitboard(0x0000000008000000);
    assert_eq!(d4.shift_east(), Bitboard(0x0000000010000000));
    assert_eq!(d4.shift_se(), Bitboard(0x100000));
    assert_eq!(d4.shift_south(), Bitboard(0x80000));
    assert_eq!(d4.shift_sw(), Bitboard(0x40000));
    assert_eq!(d4.shift_west(), Bitboard(0x4000000));
    assert_eq!(d4.shift_nw(), Bitboard(0x400000000));
    assert_eq!(d4.shift_north(), Bitboard(0x800000000));
    assert_eq!(d4.shift_ne(), Bitboard(0x1000000000));
    // TODO: test squares at edges of board
}

// TODO: test from_fen()
