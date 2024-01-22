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
