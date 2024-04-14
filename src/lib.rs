pub mod error;
pub mod memory;
pub mod registers;
pub mod test;
pub mod user_interface;

mod opcode_methods;

use crate::{error::*, memory::*, registers::*, user_interface::*};

#[derive(Debug, Default)]
pub struct Chip8 {
    memory: Memory,
    stack: CallStack,
    sound_timer: SoundTimer,
    delay_timer: DelayTimer,
    v: VRegisters,
    i: IRegister,
    display: Display,
    keypad: Keypad,
}
impl Chip8 {
    pub fn run(mut self) -> Result<(), Chip8Error> {
        loop {
            let opcode = self.memory.get_current_instruction().ok_or(Chip8Error::ProgramCounterOutOfBounds)?;
            self.execute_instruction(opcode)?;

            todo!()
        }
    }
    pub fn execute_instruction(&mut self, opcode: Opcode) -> Result<(), Chip8Error> {
        match opcode.nibbles() {
            [0x0, 0x0, 0xE, 0x0] => self.execute_00e0()?,
            [0x1,   _,   _,   _] => self.execute_1nnn(opcode.nnn())?,
            [0x6,   _,   _,   _] => self.execute_6xnn(opcode.x(), opcode.nn())?,
            [0x7,   _,   _,   _] => self.execute_7xnn(opcode.x(), opcode.nn())?,
            [0xA,   _,   _,   _] => self.execute_annn(opcode.nnn())?,
            [0xD,   _,   _,   _] => self.execute_dxyn()?,
            _ => {}
        };

        todo!()
    }
}
