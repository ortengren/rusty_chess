use crate::board::PieceType;
use crate::board::Color;
use crate::bitboard::{ Bitboard, 
                       A_FILE, B_FILE, G_FILE, H_FILE,
                       FIRST_RANK, SECOND_RANK,
                       SEVENTH_RANK, EIGHTH_RANK,
                       is_on_h_file, is_on_a_file, is_on_b_file, is_on_g_file,
                       is_on_first_rank, is_on_eighth_rank, 
                       is_on_second_rank, is_on_seventh_rank,
                     };
use crate::bitboard;
use crate::tables::BLACK_IDX;
use crate::tables::WHITE_IDX;
use crate::tables::LeaperAttackTable;

pub struct Move {
    start_square: u8,
    end_square: u8,
    promotion: Option<PieceType>,
}

pub fn mask_pawn_attacks(square: usize, color: Color) -> Bitboard {
    let mut piece_bb = Bitboard(0);
    let mut attacks_bb = Bitboard(0);
    piece_bb.set_bit_in_place(square as u8);

    match color {
        Color::White => {
            attacks_bb = bitboard::or(&attacks_bb, &piece_bb.shift_nw());
            attacks_bb = bitboard::or(&attacks_bb, &piece_bb.shift_ne());

        }
        Color::Black => {
            attacks_bb = bitboard::or(&attacks_bb, &piece_bb.shift_sw());
            attacks_bb = bitboard::or(&attacks_bb, &piece_bb.shift_se());
        }
    } 

    if is_on_a_file(square.try_into().unwrap()) {
            attacks_bb = bitboard::and(
                &attacks_bb, 
                &bitboard::not(&H_FILE)
            );
        } else if is_on_h_file(square.try_into().unwrap()) {
            attacks_bb = bitboard::and(
                &attacks_bb,
                &bitboard::not(&A_FILE)
            );
        }

    attacks_bb
}

pub fn mask_knight_attacks(square: usize) -> Bitboard {
    let mut piece_bb = Bitboard(0);
    let mut attacks_bb = Bitboard(0);
    piece_bb.set_bit_in_place(square as u8);

    let ne_attack = piece_bb.shift_north().shift_ne();
    let nw_attack = piece_bb.shift_north().shift_nw();
    let ue_attack = piece_bb.shift_east().shift_ne();
    let le_attack = piece_bb.shift_east().shift_se();
    let se_attack = piece_bb.shift_south().shift_se();
    let sw_attack = piece_bb.shift_south().shift_sw();
    let uw_attack = piece_bb.shift_west().shift_nw();
    let lw_attack = piece_bb.shift_west().shift_sw();

    let righthand_attacks = 
        bitboard::or(
            &bitboard::or(&ne_attack, &ue_attack), 
            &bitboard::or(&le_attack, &se_attack)
        );
    let lefthand_attacks = 
        bitboard::or(
            &bitboard::or(&nw_attack, &uw_attack), 
            &bitboard::or(&lw_attack, &sw_attack)
        );
    attacks_bb = bitboard::or(&righthand_attacks, &lefthand_attacks);

    if is_on_a_file(square as u8) || is_on_b_file(square as u8) {
        attacks_bb = 
            bitboard::and(
                &attacks_bb, 
                &bitboard::not(&bitboard::or(&G_FILE, &H_FILE))
            );
    } else if is_on_g_file(square as u8) || is_on_h_file(square as u8) {
        attacks_bb = 
            bitboard::and(
                &attacks_bb, 
                &bitboard::not(&bitboard::or(&A_FILE, &B_FILE))
            );
    }

    if is_on_first_rank(square as u8) || is_on_second_rank(square as u8) {
        attacks_bb = 
            bitboard::and(
                &attacks_bb, 
                &bitboard::not(&bitboard::or(&SEVENTH_RANK, &EIGHTH_RANK))
            );
    } else if is_on_seventh_rank(square as u8) || is_on_eighth_rank(square as u8) {
        attacks_bb =
            bitboard::and(
                &attacks_bb,
                &bitboard::not(&bitboard::or(&FIRST_RANK, &SECOND_RANK))
            );
    }
    attacks_bb
}

pub fn mask_king_attacks(square: usize) -> Bitboard {
    let mut piece_bb = Bitboard(0);
    let mut attack_bb = Bitboard(0);
    piece_bb.set_bit_in_place(square as u8);
    attack_bb = 
        bitboard::or(
            &bitboard::or(
                &bitboard::or(
                    &piece_bb.shift_north(),
                    &piece_bb.shift_ne()
                ),
                &bitboard::or(
                    &piece_bb.shift_east(),
                    &piece_bb.shift_se()
                )
            ),
            &bitboard::or(
                &bitboard::or(
                    &piece_bb.shift_south(),
                    &piece_bb.shift_sw()
                ),
                &bitboard::or(
                    &piece_bb.shift_west(),
                    &piece_bb.shift_nw()
                )
            )
        );

    if is_on_a_file(square as u8) {
        attack_bb = bitboard::and(&attack_bb, &bitboard::not(&H_FILE));
    } else if is_on_h_file(square as u8) {
        attack_bb = bitboard::and(&attack_bb, &bitboard::not(&A_FILE));
    }

    if is_on_first_rank(square as u8) {
        attack_bb = bitboard::and(&attack_bb, &bitboard::not(&EIGHTH_RANK));
    } else if is_on_eighth_rank(square as u8) {
        attack_bb = bitboard::and(&attack_bb, &bitboard::not(&FIRST_RANK));
    }
    attack_bb
}

pub fn init_leaper_attacks() -> LeaperAttackTable {
    let mut pawn_attacks: [[Bitboard; 64]; 2] = [[Bitboard(0); 64], [Bitboard(0); 64]];
    let mut knight_attacks = [Bitboard(0); 64];
    let mut king_attacks = [Bitboard(0); 64];

    for square in 0..64 {
        pawn_attacks[WHITE_IDX][square] = mask_pawn_attacks(square, Color::White);
        pawn_attacks[BLACK_IDX][square] = mask_pawn_attacks(square, Color::Black);

        knight_attacks[square] = mask_knight_attacks(square);

        king_attacks[square] = mask_king_attacks(square);
    }

    LeaperAttackTable {
        pawn_attacks,
        knight_attacks,
        king_attacks,
    }
}

pub fn mask_bishop_attacks(square: usize) -> Bitboard {
    let mut attack_bb = Bitboard(0);
    
    let mut rank: u8;
    let mut file: u8;

    todo!();
}

mod tests;
