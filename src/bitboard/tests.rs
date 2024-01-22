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

    let e4: Bitboard = Bitboard(268435456);
    assert_eq!(e4.shift_ne(), Bitboard(137438953472));
    assert_eq!(e4.shift_sw(), Bitboard(524288));
    // TODO: test squares at edges of board
}

#[test]
fn test_idx_to_reading_order() {
    assert_eq!(idx_to_reading_order(0), 56);
    assert_eq!(idx_to_reading_order(7), 63);
    assert_eq!(idx_to_reading_order(8), 48);
    assert_eq!(idx_to_reading_order(20), 44);
    assert_eq!(idx_to_reading_order(28), 36);
    assert_eq!(idx_to_reading_order(32), 24);
    assert_eq!(idx_to_reading_order(39), 31);
    assert_eq!(idx_to_reading_order(40), 16);
    assert_eq!(idx_to_reading_order(47), 23);
    assert_eq!(idx_to_reading_order(48), 8);
    assert_eq!(idx_to_reading_order(55), 15);
    assert_eq!(idx_to_reading_order(56), 0);
    assert_eq!(idx_to_reading_order(63), 7);
}

#[test]
fn test_or() {
    assert_eq!(Bitboard(1), Bitboard(0).or(Bitboard(1)));
    assert_eq!(Bitboard(72057937635311746), 
               Bitboard(72057662757404800).or(Bitboard(274877906946)));
    assert_eq!(Bitboard(51541704864), 
               Bitboard(51541704704).or(Bitboard(2097312)));
}

#[test]
fn test_set_bit_in_place() {
    let mut actual = Bitboard(0);
    actual.set_bit_in_place(28);

    assert_eq!(Bitboard(268435456), actual);
}
