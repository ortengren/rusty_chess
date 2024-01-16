use core::fmt;
use std::ascii::AsciiExt;
use std::fmt::Debug;

use crate::board::Color;
use crate::board::Color::{White, Black};
use crate::board::Board;
use crate::board::PieceType::{Pawn, Knight, Bishop, Rook, Queen, King};
use crate::board::{FILES, RANKS};
use crate::board::Piece;


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


#[derive(Debug, PartialEq)]
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
}

mod tests;
