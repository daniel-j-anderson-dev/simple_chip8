use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Display<const WIDTH: usize, const HEIGHT: usize>([[bool; WIDTH]; HEIGHT]);

impl<const W: usize, const H: usize> Deref for Display<W, H> {
    type Target = [[bool; W]; H];
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
impl<const W: usize, const H: usize> DerefMut for Display<W, H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
impl<const W: usize, const H: usize> Default for Display<W, H> {
    fn default() -> Self {
        return Self([[false; W]; H]);
    }
}
