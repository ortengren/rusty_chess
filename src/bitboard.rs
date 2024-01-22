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

pub const LIGHT_SQUARES: Bitboard = Bitboard(0x5555555555555555);
pub const DARK_SQUARES: Bitboard = Bitboard(0xAAAAAAAAAAAAAAAA);


pub fn idx_to_reading_order(idx: u64) -> u64 {
    // 0 -> 56      +56     + (56 - ((0/8)*16))
    // 7 -> 63      +56
    // 8 -> 48      +40
    // 15 -> 55     +40
    // 16 -> 40     +24
    // 23 -> 47     +24
    // 24 -> 32     +8
    // 31 -> 39     +8
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

impl Debug for Bitboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for rank in (0..8).rev() {
            write!(f, "{}  ", rank + 1)?;
            for file in 0..8 {
                write!(f, "{} ", self.get_bit(rank * 8 + file))?;
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
