use std::ops::{Deref, DerefMut};

#[derive(Debug, Default)]
pub struct VRegisters([u8; 16]);
impl Deref for VRegisters {
    type Target = [u8; 16];
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
impl DerefMut for VRegisters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
