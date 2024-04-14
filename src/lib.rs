pub mod memory;
pub mod registers;
pub mod test;
pub mod user_interface;

use crate::{memory::*, registers::*, user_interface::*};

#[derive(Debug, Default)]
pub struct Chip8 {
    memory: Memory<4096>,
    stack: Stack<16>,
    sound_timer: SoundTimer,
    delay_timer: DelayTimer,
    v: VRegisters,
    i: IRegister,
    display: Display<64, 32>,
    keypad: Keypad,
}
