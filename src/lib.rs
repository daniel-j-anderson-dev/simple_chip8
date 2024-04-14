pub mod memory;
pub mod registers;
pub mod test;
pub mod error;
pub mod user_interface;

use crate::{error::*,memory::*,registers::*,user_interface::*};

#[derive(Debug, Default)]
pub struct Chip8 {
    memory: Memory<4096>,
    stack: AddressStack<16>,
    sound_timer: SoundTimer,
    delay_timer: DelayTimer,
    v: VRegisters,
    i: IRegister,
    display: Display<64, 32>,
    keypad: Keypad,
}
impl Chip8 {
    pub fn run(self) -> Result<(), Chip8Error> {
        unimplemented!()
    }
    pub fn execute_instruction(&self, opcode: Opcode) -> Result<(), Chip8Error> {
        match opcode.nibbles() {
            [0x0, 0x0, 0xE, 0x0] => {},
            [0x1,   _,   _,   _] => {},
            [0x6,   _,   _,   _] => {},
            [0x7,   _,   _,   _] => {},
            [0xA,   _,   _,   _] => {},
            [0xD,   _,   _,   _] => {},
            _ => {}
        }

        todo!()
    }
}
