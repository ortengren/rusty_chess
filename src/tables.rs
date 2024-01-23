use crate::bitboard::{self, FIRST_DIAG, SECOND_DIAG, THIRD_DIAG, FOURTH_DIAG, FIFTH_DIAG, SIXTH_DIAG, SEVENTH_DIAG, EIGHTH_ANTIDIAG, EIGHTH_DIAG, NINTH_DIAG, TENTH_DIAG, ELEVENTH_DIAG, TWELFTH_DIAG, THIRTEENTH_DIAG, FOURTEENTH_DIAG, FIFTEENTH_DIAG, FIRST_ANTIDIAG, SECOND_ANTIDIAG, THIRD_ANTIDIAG, FOURTH_ANTIDIAG, FIFTH_ANTIDIAG, SIXTH_ANTIDIAG, SEVENTH_ANTIDIAG, NINTH_ANTIDIAG, TENTH_ANTIDIAG, ELEVENTH_ANTIDIAG, TWELFTH_ANTIDIAG, THIRTEENTH_ANTIDIAG, FOURTEENTH_ANTIDIAG, FIFTEENTH_ANTIDIAG, OUTER_SQUARES, FIRST_RANK, SECOND_RANK, THIRD_RANK, FOURTH_RANK, FIFTH_RANK, SIXTH_RANK, SEVENTH_RANK, EIGHTH_RANK, A_FILE, B_FILE, C_FILE, D_FILE, E_FILE, F_FILE, G_FILE, H_FILE};
use crate::bitboard::Bitboard;
use crate::moves::init_leaper_attacks;

pub const WHITE_IDX: usize = 0;
pub const BLACK_IDX: usize = 1;

pub struct Tables {
    leaper_attacks: LeaperAttackTable,
}

impl Tables {
    pub fn init() -> Tables {
        Tables {
            leaper_attacks: init_leaper_attacks(),
        }
    }
}

pub struct LeaperAttackTable {
    pub pawn_attacks: [[Bitboard; 64]; 2],
    pub knight_attacks: [Bitboard; 64],
    pub king_attacks: [Bitboard; 64],
}

pub fn mask_bishop_attacks(square: usize) -> Bitboard {
    let attack_bb;

    let mut piece_bb = Bitboard(0);
    piece_bb.set_bit_in_place(square as u8);

    let mut diagonal: Bitboard = Bitboard(0);
    let mut antidiagonal: Bitboard = Bitboard(0);

    let diagonals = 
       [FIRST_DIAG, SECOND_DIAG, THIRD_DIAG, FOURTH_DIAG, FIFTH_DIAG, 
        SIXTH_DIAG, SEVENTH_DIAG, EIGHTH_DIAG, NINTH_DIAG, TENTH_DIAG, 
        ELEVENTH_DIAG, TWELFTH_DIAG, THIRTEENTH_DIAG, FOURTEENTH_DIAG, FIFTEENTH_DIAG];

    let antidiagonals = 
        [FIRST_ANTIDIAG, SECOND_ANTIDIAG, THIRD_ANTIDIAG, 
         FOURTH_ANTIDIAG, FIFTH_ANTIDIAG, SIXTH_ANTIDIAG, 
         SEVENTH_ANTIDIAG, EIGHTH_ANTIDIAG, NINTH_ANTIDIAG, 
         TENTH_ANTIDIAG, ELEVENTH_ANTIDIAG, TWELFTH_ANTIDIAG, 
         THIRTEENTH_ANTIDIAG, FOURTEENTH_ANTIDIAG, FIFTEENTH_ANTIDIAG];

    // This is slow.  It would be faster to begin at eighth diagonal & work towards edges
    for diag in diagonals {
        if bitboard::and(&piece_bb, &diag).0 != 0 {
            diagonal = diag;
            break;
        }
    }
    // Also slow (see above)
    for antidiag in antidiagonals {
        if bitboard::and(&piece_bb, &antidiag).0 != 0 {
            antidiagonal = antidiag;
            break;
        }
    }

    if diagonal.0 == 0 {
        println!("ERROR: square {} was calculated to have no diagonal", square);
        panic!();
    }
    if antidiagonal.0 == 0 {
        println!("ERROR: square {} was calculated to have no antidiagonal", square);
        panic!();
    }

    attack_bb = bitboard::and(
        &bitboard::or(&diagonal, &antidiagonal), 
        &bitboard::not(&OUTER_SQUARES)
    );
    bitboard::and(&attack_bb, &bitboard::not(&piece_bb))
}

pub fn mask_rook_attacks(square: usize) -> Bitboard {
    let rank = (square / 8) as u8;
    let file = (square % 8) as u8;

    let rank_bb: Bitboard = 
        match rank {
            0 => FIRST_RANK,
            1 => SECOND_RANK,
            2 => THIRD_RANK,
            3 => FOURTH_RANK,
            4 => FIFTH_RANK,
            5 => SIXTH_RANK,
            6 => SEVENTH_RANK,
            7 => EIGHTH_RANK,
            _ => {
                println!("{} is not a valid rank", rank + 1);
                unreachable!();
            },
        };
    
    let file_bb: Bitboard =
        match file {
            0 => A_FILE,
            1 => B_FILE,
            2 => C_FILE,
            3 => D_FILE,
            4 => E_FILE,
            5 => F_FILE,
            6 => G_FILE,
            7 => H_FILE,
            _ => {
                println!("{} is not a valid file index", file);
                unreachable!();
            },
        };

    bitboard::and(
        &bitboard::and(
            &bitboard::or(&rank_bb, &file_bb), 
            &bitboard::not(&Bitboard(0).set_bit(square as u8))
        ),
        &bitboard::not(&OUTER_SQUARES)
    )
}

pub fn otf_bishop_attacks(square: u8, blockers: Bitboard) -> Bitboard {
    let mut attacks_bb = Bitboard(0);

    let mut rank: u8;
    let mut file: u8;

    let target_rank = (square / 8) as u8;
    let target_file = (square % 8) as u8;

    let mut shifted: u64;
    rank = target_rank + 1;
    file = target_file + 1;
    while rank <= 7 && file <= 7 {
        shifted = 1u64 << (rank * 8 + file);
        attacks_bb = bitboard::or(&attacks_bb, &Bitboard(shifted));
        if bitboard::and(&blockers, &Bitboard(shifted)).0 != 0 {
            break;
        }
        rank += 1;
        file +=1;
    }

    rank = target_rank - 1;
    file = target_file + 1;
    loop {
        shifted = 1u64 << (rank * 8 + file);
        attacks_bb = bitboard::or(&attacks_bb, &Bitboard(shifted));
        if rank == 0 || file == 7 || bitboard::and(&blockers, &Bitboard(shifted)).0 != 0 {
            break;
        }
        rank -= 1;
        file +=1;
    }

    rank = target_rank + 1;
    file = target_file - 1;
    loop {
        shifted = 1u64 << (rank * 8 + file);
        attacks_bb = bitboard::or(&attacks_bb, &Bitboard(shifted));
        if rank == 7 || file == 0 || bitboard::and(&blockers, &Bitboard(shifted)).0 != 0 {
            break;
        }
        rank += 1;
        file -=1;
    }

    rank = target_rank - 1;
    file = target_file - 1;
    loop {
        shifted = 1u64 << (rank * 8 + file);
        attacks_bb = bitboard::or(&attacks_bb, &Bitboard(shifted));
        if rank == 0 || file == 0 || bitboard::and(&blockers, &Bitboard(shifted)).0 != 0  {
            break;
        }
        rank -= 1;
        file -=1;
    }
    attacks_bb
}
