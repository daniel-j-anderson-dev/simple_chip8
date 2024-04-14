use thiserror::Error;

#[derive(Debug, Error)]
pub enum Chip8Error {
    #[error("Could not fetch current instruction because the program counter was out of bounds")]
    ProgramCounterOutOfBounds
}
