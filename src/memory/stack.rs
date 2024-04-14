//! This module contains the definition for a simple array backed [CallStack] of [u16] values

use std::ops::{Deref, DerefMut};

#[derive(Debug)]
/// This struct is the call stack of the chip8 interpreter.
/// - holds `CAPACITY` addresses (defaults to 16)
/// - keeps track of the current address with a [u8]
pub struct CallStack<const CAPACITY: usize = 16> {
    addresses: [u16; CAPACITY],
    pointer: u8,
    is_empty: bool,
}
impl<const N: usize> CallStack<N> {
    /// Returns the generic constant `CAPACITY`
    pub const fn capacity(&self) -> usize {
        return N;    
    }

    /// Returns the number of addresses currently on the stack
    pub fn length(&self) -> usize {
        return self.pointer as usize + 1;
    }

    /// Returns the current stack pointer
    pub fn pointer(&self) -> u8 {
        return self.pointer;
    }

    /// Returns the address at the current stack pointer
    /// ## Returns
    /// - [Some] if there is an address at the current stack_pointer
    /// - [None] if the stack pointer is out of bounds
    pub fn get_current_address(&self) -> Option<u16> {
        return self.get(self.pointer as usize).copied();
    }

    /// Returns a mutable reference to the address at the current stack pointer
    /// ## Returns
    /// - [Some] if there is an address at the current stack_pointer
    /// - [None] if the stack pointer is out of bounds
    pub fn get_current_address_mut(&mut self) -> Option<&mut u16> {
        return self.addresses.get_mut(self.pointer as usize);
    }

    /// Returns the address at the stack pointer, returned value is replaced by 0, and decrements the stack pointer
    /// ## Returns
    /// - [Some] if there is an address at the current stack_pointer
    /// - [None] if the stack pointer is out of bounds
    /// - [None] if there have not been any addresses pushed onto `self`
    pub fn pop(&mut self) -> Option<u16> {
        if self.is_empty {
            return None;
        }

        let current_address = self.get_current_address_mut()?;

        let removed_address = *current_address;

        *current_address = 0;

        if self.pointer == 0 {
            self.is_empty = true;
        } else {
            self.pointer -= 1;
        }

        return Some(removed_address);
    }
    
    /// Increments the stack pointer and assigns `address` to the current address
    /// ## Returns
    /// - [Some] if the stack pointer is in bounds
    /// - [None] if the stack pointer is out of bounds
    pub fn push(&mut self, address: u16) -> Option<()> {
        if (self.pointer as usize) >= self.capacity() - 1 {
            return None;
        }
        
        self.pointer += 1;

        *self.get_current_address_mut()? = address;

        self.is_empty = false;

        Some(())
    }
}

impl<const N: usize> Deref for CallStack<N> {
    type Target = [u16; N];
    /// Allow all immutable slice methods
    fn deref(&self) -> &Self::Target {
        return &self.addresses;
    }
}
impl<const N: usize> DerefMut for CallStack<N> {
    /// Allow all mutable slice methods
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.addresses;
    }
}
impl<const N: usize> Default for CallStack<N> {
    fn default() -> Self {
        return Self {
            addresses: [0; N],
            pointer: 0,
            is_empty: true,
        };
    }
}
