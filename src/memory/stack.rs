use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Stack<const CAPACITY: usize> {
    addresses: [u16; CAPACITY],
    pointer: u8,
}
impl<const N: usize> Stack<N> {
    pub fn pointer(&self) -> u8 {
        return self.pointer as u8;
    }
    pub fn get_current_address(&self) -> Option<u16> {
        return self.addresses.get(self.pointer as usize).copied();
    }
    pub fn get_current_address_mut(&mut self) -> Option<&mut u16> {
        return self.addresses.get_mut(self.pointer as usize);
    }
    pub fn pop(&mut self) -> Option<u16> {
        return if let Some(current_address) = self.get_current_address_mut() {
            let address = *current_address;
            *current_address = 0;
            self.pointer = self.pointer.saturating_sub(1);
            Some(address)
        } else {
            None
        };
    }
    pub fn push(&mut self, address: u16) -> Option<()> {
        self.pointer = self.pointer.saturating_add(1);

        return if let Some(current_address) = self.get_current_address_mut() {
            *current_address = address;
            Some(())
        } else {
            self.pointer -= 1;
            None
        };
    }
}

impl<const N: usize> Deref for Stack<N> {
    type Target = [u16; N];
    fn deref(&self) -> &Self::Target {
        return &self.addresses;
    }
}
impl<const N: usize> DerefMut for Stack<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.addresses;
    }
}
impl<const N: usize> Default for Stack<N> {
    fn default() -> Self {
        return Self {
            addresses: [0; N],
            pointer: 0,
        };
    }
}
