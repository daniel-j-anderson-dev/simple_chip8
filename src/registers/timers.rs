use std::ops::{Deref, DerefMut};

#[derive(Debug, Default)]
pub struct DelayTimer(u8);
impl Deref for DelayTimer {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
impl DerefMut for DelayTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
impl From<u8> for DelayTimer {
    fn from(value: u8) -> Self {
        return Self(value);
    }
}

#[derive(Debug, Default)]
pub struct SoundTimer(u8);
impl Deref for SoundTimer {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
impl DerefMut for SoundTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
impl From<u8> for SoundTimer {
    fn from(value: u8) -> Self {
        return Self(value);
    }
}
