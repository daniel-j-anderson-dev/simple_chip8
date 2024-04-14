pub mod opcode;
pub use self::opcode::*;

use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Memory<const CAPACITY: usize> {
    bytes: [u8; CAPACITY],
    program_counter: u16,
}
impl<const N: usize> Memory<N> {
    pub fn get_program_counter(&self) -> u16 {
        return self.program_counter;
    }
    pub fn get_program_counter_mut(&mut self) -> &mut u16 {
        return &mut self.program_counter;
    }
    pub fn increment_program_counter(&mut self) -> Option<()> {
        self.program_counter = self.program_counter.checked_add(2)?;
        return Some(());
    }
    pub fn get_current_instruction(&self) -> Option<Opcode> {
        let program_counter = self.program_counter as usize;
        let most_significant = self.get(program_counter)?;
        let least_significant = self.get(program_counter + 1)?;
        let current_instruction = Opcode::merge_bytes(*most_significant, *least_significant);
        return Some(current_instruction);
    }
}

impl<const N: usize> Deref for Memory<N> {
    type Target = [u8; N];
    fn deref(&self) -> &Self::Target {
        return &self.bytes;
    }
}
impl<const N: usize> DerefMut for Memory<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.bytes;
    }
}
impl<const N: usize> Default for Memory<N> {
    fn default() -> Self {
        Self {
            bytes: [0; N],
            program_counter: 0,
        }
    }
}
