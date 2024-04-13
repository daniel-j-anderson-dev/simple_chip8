use std::ops::{Deref, DerefMut};

pub struct Opcode(u16);
impl Opcode {
    pub fn merge_bytes(most_significant_byte: u8, least_significant_byte: u8) -> Opcode {
        let most_significant_byte = most_significant_byte as u16;
        let least_significant_byte = least_significant_byte as u16;

        return Self((most_significant_byte << 8) | least_significant_byte);
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
