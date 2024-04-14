pub mod display;
pub mod keypad;
pub mod memory;
pub mod registers;
pub mod stack;
pub mod test;

pub use crate::{display::*, memory::*, registers::*, stack::*, keypad::*};

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
