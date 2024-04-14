use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Opcode(u16);
impl Opcode {
    pub fn merge_bytes(most_significant: u8, least_significant: u8) -> Opcode {
        let most_significant = most_significant as u16;
        let least_significant = least_significant as u16;

        return Self((most_significant << 8) | least_significant);
    }
    /// - `self.nibbles[0]` most significant hexadecimal digit
    /// - `self.nibbles[1]`
    /// - `self.nibbles[2]`
    /// - `self.nibbles[3]` least significant hexadecimal digit
    pub fn nibbles(&self) -> [u8; 4] {
        return [
            (self.0 & 0xF000) as u8,
            (self.0 & 0x0F00) as u8,
            (self.0 & 0x00F0) as u8,
            (self.0 & 0x000F) as u8,
        ];
    }
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        return Self(value);
    }
}
impl Deref for Opcode {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
impl DerefMut for Opcode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
