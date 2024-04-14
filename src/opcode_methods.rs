use super::*;

impl Chip8 {
    pub fn execute_00e0(&mut self) -> Result<(), Chip8Error> {
        unimplemented!()
    }
    pub fn execute_1nnn(&mut self, address: u16) -> Result<(), Chip8Error> {
        unimplemented!()
    }
    /// Set a specific [VRegisters] to `value`
    pub fn execute_6xnn(&mut self, v_register_index: u8, value: u8) -> Result<(), Chip8Error> {
        unimplemented!()
    }
    /// Add `value` to the value in the specified [VRegisters]
    pub fn execute_7xnn(&mut self, v_register_index: u8,  value: u8) -> Result<(), Chip8Error> {
        unimplemented!()
    }
    /// Set [IRegister] to `address`
    pub fn execute_annn(&mut self, address: u16) -> Result<(), Chip8Error> {
        unimplemented!()
    }
    /// Draw the [Display]
    pub fn execute_dxyn(&mut self) -> Result<(), Chip8Error> {
        unimplemented!()
    }
}