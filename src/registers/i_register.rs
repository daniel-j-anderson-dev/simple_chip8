use std::ops::{Deref, DerefMut};

#[derive(Debug, Default)]
pub struct IRegister(u16);
impl Deref for IRegister {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
impl DerefMut for IRegister {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
