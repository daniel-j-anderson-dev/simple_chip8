//! This module contains the definition of [Opcode]

use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, PartialEq, Eq)]
/// This struct is a wrapper around a [u16] value. It can easily be split up into nibbles and different arguments of an opcode.
pub struct Opcode(u16);
impl Opcode {
    /// Merges two bytes into an [Opcode] <br>
    /// this is done by casting each byte as a [u16].
    /// Then the most significant is bit shifted left by 8 and bitwise or'ed with least significant bit.
    pub fn merge_bytes(most_significant: u8, least_significant: u8) -> Opcode {
        let most_significant = most_significant as u16;
        let least_significant = least_significant as u16;

        return Self((most_significant << 8) | least_significant);
    }
    /// Returns an array containing the opcode as an array of four nibbles.
    /// - `nibbles[0]` most significant hexadecimal digit
    /// - `nibbles[1]`
    /// - `nibbles[2]`
    /// - `nibbles[3]` least significant hexadecimal digit
    pub fn nibbles(&self) -> [u8; 4] {
        return [
            (self.0 & 0xF000) as u8,
            (self.0 & 0x0F00) as u8,
            (self.0 & 0x00F0) as u8,
            (self.0 & 0x000F) as u8,
        ];
    }
    /// The second nibble. Used to look up one of the 16 registers (VX) from V0 through VF.
    pub fn x(&self) -> u8 {
        return (self.0 & 0x0F00) as u8;
    }
    /// The third nibble. Also used to look up one of the 16 registers (VY) from V0 through VF.
    pub fn y(&self) -> u8 {
        return (self.0 & 0x00F0) as u8;
    }
    /// The fourth nibble. A 4-bit number.
    pub fn n(&self) -> u8 {
        return (self.0 & 0x000F) as u8;
    }
    /// The second byte (third and fourth nibbles). An 8-bit immediate number
    pub fn nn(&self) -> u8 {
        return (self.0 & 0x00FF) as u8;
    }
    /// The second, third and fourth nibbles. A 12-bit immediate memory address.
    pub fn nnn(&self) -> u16 {
        return self.0 & 0x0FFF;
    }
}

impl From<u16> for Opcode {
    /// simple constructor
    fn from(value: u16) -> Self {
        return Self(value);
    }
}
impl Deref for Opcode {
    type Target = u16;
    /// allow [Opcode] to use all immutable [u16] methods
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
impl DerefMut for Opcode {
    /// allow [Opcode] to use all mutable [u16] methods
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
