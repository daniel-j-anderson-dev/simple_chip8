//! This module contains the definitions of [Memory], [CallStack], and [Opcode]

pub mod opcode;
pub mod stack;

pub use self::{opcode::*, stack::*};

use std::ops::{Deref, DerefMut};

#[derive(Debug)]
/// This struct represents the internal memory and program counter of a Chip8 interpreter
/// - The capacity is fixed and must be known at compile time
///   - Defaults to 4096 bytes
/// - [Memory] can be [Deref]renceded as a u8 slice so all slice methods are available
pub struct Memory<const CAPACITY: usize = 4096> {
    bytes: [u8; CAPACITY],
    program_counter: u16,
}
impl<const N: usize> Memory<N> {
    /// returns the current program counter
    pub fn get_program_counter(&self) -> u16 {
        return self.program_counter;
    }
    /// returns a mut reference to the current program counter
    pub fn get_program_counter_mut(&mut self) -> &mut u16 {
        return &mut self.program_counter;
    }
    /// increments the program counter by 2 (each instruction is 2 bytes)
    /// ## Returns
    /// - [Some] if the `program_counter` was incremented
    /// - [None] if the `program_counter` was not modified
    pub fn increment_program_counter(&mut self) -> Option<()> {
        self.program_counter = self.program_counter.checked_add(2)?;
        return Some(());
    }
    /// Reads the current and next byte and merges them into an opcode.
    /// ## Returns
    /// - [Some] if the `program_counter` AND `program_counter + 1` are in bounds
    /// - [None] if the `program_counter` AND `program_counter + 1` are out of bounds
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
    /// Allow all immutable slice methods
    fn deref(&self) -> &Self::Target {
        return &self.bytes;
    }
}
impl<const N: usize> DerefMut for Memory<N> {
    /// Allow all mutable slice methods
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.bytes;
    }
}
impl<const N: usize> Default for Memory<N> {
    /// default to all zeros
    fn default() -> Self {
        Self {
            bytes: [0; N],
            program_counter: 0,
        }
    }
}
