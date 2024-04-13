use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Opcode(u16);
impl Opcode {
    pub fn merge_bytes(most_significant: u8, least_significant: u8) -> Opcode {
        let most_significant = most_significant as u16;
        let least_significant = least_significant as u16;

        return Self((most_significant << 8) | least_significant);
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
