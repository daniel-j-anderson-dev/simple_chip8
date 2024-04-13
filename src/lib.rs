pub mod memory;
pub mod registers;
pub mod stack;
pub mod test;

pub use crate::{memory::*, registers::*, stack::*};

#[derive(Debug, Default)]
pub struct Chip8 {
    memory: Memory<4096>,
    stack: AddressStack<16>,
    sound_timer: SoundTimer,
    delay_timer: DelayTimer,
    v: VRegisters,
    i: IRegister,
}
