use crate::board::*;
use super::*;

#[test]
fn test_mask_pawn_attacks() {
    assert_eq!(
        Bitboard(171798691840), 
        mask_pawn_attacks(28, Color::White)
    );
    assert_eq!(
        Bitboard(4194304),
        mask_pawn_attacks(31, Color::Black)
    );
}

#[test]
fn test_mask_knight_attacks() {
    assert_eq!(
        Bitboard(345879119952),
        mask_knight_attacks(21)
    );

    assert_eq!(
        Bitboard(2685403152),
        mask_knight_attacks(14)
    );

    assert_eq!(
        Bitboard(1128098930098176),
        mask_knight_attacks(56)
    );
}

#[test]
fn test_mask_king_attacks() {
    assert_eq!(
        Bitboard(241192927232),
        mask_king_attacks(28)
    );

    assert_eq!(
        Bitboard(4665729213955833856),
        mask_king_attacks(63)
    );
}

#[test]
fn test_init_leaper_attacks() {
    let test = init_leaper_attacks();
    
    assert_eq!(
        Bitboard(171798691840),
        test.pawn_attacks[0][28]
    );
    
    assert_eq!(
        Bitboard(9077567998918656),
        test.knight_attacks[63]
    );

    assert_eq!(
        Bitboard(7188),
        test.king_attacks[3]
    );
}
