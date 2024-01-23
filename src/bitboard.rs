use core::fmt;
use std::fmt::{Debug, Formatter};


pub const A_FILE: Bitboard = Bitboard(0x0101010101010101);
pub const B_FILE: Bitboard = Bitboard(0x0202020202020202);
pub const C_FILE: Bitboard = Bitboard(0x0404040404040404);
pub const D_FILE: Bitboard = Bitboard(0x0808080808080808);
pub const E_FILE: Bitboard = Bitboard(0x1010101010101010);
pub const F_FILE: Bitboard = Bitboard(0x2020202020202020);
pub const G_FILE: Bitboard = Bitboard(0x4040404040404040);
pub const H_FILE: Bitboard = Bitboard(0x8080808080808080);

pub const FIRST_RANK: Bitboard = Bitboard(0x00000000000000ff);
pub const SECOND_RANK: Bitboard = Bitboard(0x000000000000ff00);
pub const THIRD_RANK: Bitboard = Bitboard(0x0000000000ff0000);
pub const FOURTH_RANK: Bitboard = Bitboard(0x00000000ff000000);
pub const FIFTH_RANK: Bitboard = Bitboard(0x000000ff00000000);
pub const SIXTH_RANK: Bitboard = Bitboard(0x0000ff0000000000);
pub const SEVENTH_RANK: Bitboard = Bitboard(0x00ff000000000000);
pub const EIGHTH_RANK: Bitboard = Bitboard(0xff00000000000000);

pub const FIRST_DIAG: Bitboard = Bitboard(0x100000000000000);
pub const SECOND_DIAG: Bitboard = Bitboard(0x201000000000000);
pub const THIRD_DIAG: Bitboard = Bitboard(0x402010000000000);
pub const FOURTH_DIAG: Bitboard = Bitboard(0x804020100000000);
pub const FIFTH_DIAG: Bitboard = Bitboard(0x1008040201000000);
pub const SIXTH_DIAG: Bitboard = Bitboard(0x2010080402010000);
pub const SEVENTH_DIAG: Bitboard = Bitboard(0x4020100804020100);
pub const EIGHTH_DIAG: Bitboard = Bitboard(0x8040201008040201);
pub const NINTH_DIAG: Bitboard = Bitboard(0x80402010080402);
pub const TENTH_DIAG: Bitboard = Bitboard(0x804020100804);
pub const ELEVENTH_DIAG: Bitboard = Bitboard(0x8040201008);
pub const TWELFTH_DIAG: Bitboard = Bitboard(0x80402010);
pub const THIRTEENTH_DIAG: Bitboard = Bitboard(0x804020);
pub const FOURTEENTH_DIAG: Bitboard = Bitboard(0x8040);
pub const FIFTEENTH_DIAG: Bitboard = Bitboard(0x80);

pub const FIRST_ANTIDIAG: Bitboard = Bitboard(0x1);
pub const SECOND_ANTIDIAG: Bitboard = Bitboard(0x102);
pub const THIRD_ANTIDIAG: Bitboard = Bitboard(0x10204);
pub const FOURTH_ANTIDIAG: Bitboard = Bitboard(0x1020408);
pub const FIFTH_ANTIDIAG: Bitboard = Bitboard(0x102040810);
pub const SIXTH_ANTIDIAG: Bitboard = Bitboard(0x10204081020);
pub const SEVENTH_ANTIDIAG: Bitboard = Bitboard(0x1020408102040);
pub const EIGHTH_ANTIDIAG: Bitboard = Bitboard(0x102040810204080);
pub const NINTH_ANTIDIAG: Bitboard = Bitboard(0x204081020408000);
pub const TENTH_ANTIDIAG: Bitboard = Bitboard(0x408102040800000);
pub const ELEVENTH_ANTIDIAG: Bitboard = Bitboard(0x810204080000000);
pub const TWELFTH_ANTIDIAG: Bitboard = Bitboard(0x1020408000000000);
pub const THIRTEENTH_ANTIDIAG: Bitboard = Bitboard(0x2040800000000000);
pub const FOURTEENTH_ANTIDIAG: Bitboard = Bitboard(0x4080000000000000);
pub const FIFTEENTH_ANTIDIAG: Bitboard = Bitboard(0x8000000000000000);

pub const LIGHT_SQUARES: Bitboard = Bitboard(0x5555555555555555);
pub const DARK_SQUARES: Bitboard = Bitboard(0xAAAAAAAAAAAAAAAA);

pub const OUTER_SQUARES: Bitboard = 
    Bitboard(FIRST_RANK.0 | EIGHTH_RANK.0 | A_FILE.0 | H_FILE.0);


pub fn idx_to_reading_order(idx: u64) -> u64 {
    println!("{}", idx);
    (idx + 56) - ((idx/8) * 16)
}

pub fn is_on_a_file(square: u8) -> bool {
    square % 8 == 0
}

pub fn is_on_b_file(square: u8) -> bool {
    square % 8 == 1
}

pub fn is_on_g_file(square: u8) -> bool {
    square % 8 == 6
}

pub fn is_on_h_file(square: u8) -> bool {
    square % 8 == 7
}

pub fn is_on_first_rank(square: u8) -> bool {
    square <= 7
}

pub fn is_on_second_rank(square: u8) -> bool {
    square >= 8 && square <= 15
}

pub fn is_on_seventh_rank(square: u8) -> bool {
    square >= 48 && square <= 55
}

pub fn is_on_eighth_rank(square: u8) -> bool {
    square >= 56
}

pub fn not(bitboard: &Bitboard) -> Bitboard {
    Bitboard(!bitboard.0)
}

pub fn and(bb1: &Bitboard, bb2: &Bitboard) -> Bitboard {
    Bitboard(bb1.0 & bb2.0)
}

pub fn or(bb1: &Bitboard, bb2: &Bitboard) -> Bitboard {
    Bitboard(bb1.0 | bb2.0)
}

#[derive(PartialEq, Clone, Copy)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn from_square(square: (char, u8)) -> Self {
        let file_bb = match square.0 {
            'A' => A_FILE,
            'B' => B_FILE,
            'C' => C_FILE,
            'D' => D_FILE,
            'E' => E_FILE,
            'F' => F_FILE,
            'G' => G_FILE,
            'H' => H_FILE,
            _ => panic!(),
        };
        let rank_bb = match square.1 {
            1 => FIRST_RANK,
            2 => SECOND_RANK,
            3 => THIRD_RANK,
            4 => FOURTH_RANK,
            5 => FIFTH_RANK,
            6 => SIXTH_RANK,
            7 => SEVENTH_RANK,
            8 => EIGHTH_RANK,
            _ => panic!(),
        };
        Bitboard(file_bb.0 & rank_bb.0)
    }

    pub fn add(&self, bb1: Self) -> Self {
        Bitboard(self.0 + bb1.0)
    }

    pub fn shift_west(&self) -> Bitboard {
        Bitboard(self.0 >> 1)
    }

    pub fn shift_east(&self) -> Bitboard {
        Bitboard(self.0 << 1)
    }

    pub fn shift_south(&self) -> Bitboard {
        Bitboard(self.0 >> 8)
    }

    pub fn shift_north(&self) -> Bitboard {
        Bitboard(self.0 << 8)
    }

    pub fn shift_se(&self) -> Bitboard {
        Bitboard(self.0 >> 7)
    }

    pub fn shift_sw(&self) -> Bitboard {
        Bitboard(self.0 >> 9)
    }

    pub fn shift_nw(&self) -> Bitboard {
        Bitboard(self.0 << 7)
    }

    pub fn shift_ne(&self) -> Bitboard {
        Bitboard(self.0 << 9)
    }

    pub fn and(&self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 & other.0)
    }

    pub fn or(&self, other: Bitboard) -> Bitboard {
        Bitboard(self.0 | other.0)
    }

    pub fn not(&self) -> Bitboard {
        Bitboard(!self.0)
    }

    pub fn get_bit(&self, idx: u8) -> u8 {
        match self.0 & (1u64 << idx) > 0 {
            true => 1,
            false => 0,
        }
    }

    pub fn set_bit(&self, idx: u8) -> Bitboard {
        Bitboard(self.0 | (1u64 << idx))
    }

    pub fn set_bit_in_place(&mut self, idx:u8) {
        self.0 = self.0 | (1u64 << idx);
    }

    pub fn clear_bit(&self, idx: u8) -> Bitboard {
        Bitboard(self.0 & !(1u64 << idx))
    }

}

pub fn format_bit(bit: u8) -> char {
    match bit {
        0 => '.',
        1 => '1',
        _ => unreachable!(),
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for rank in (0..8).rev() {
            write!(f, "{}  ", rank + 1)?;
            for file in 0..8 {
                write!(f, "{} ", format_bit(self.get_bit(rank * 8 + file)))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n   A B C D E F G H\n\n\n\
               hex: {:x}\n\
               decimal: {}\n\n", self.0, self.0)
    }
}

impl Default for Bitboard {
    fn default() -> Bitboard {
        Bitboard(0)
    }
}

mod tests;
