pub mod opcode;

/// CHIP-8 Emulator
pub struct Emulator {
    /// Represents CHIP-8 memory: 4096 bytes
    ///
    /// TODO: Write memory locations
    pub memory: [u8; 4096],
    /// Represents CHIP-8 registers
    ///
    /// There are 16 registers
    ///
    /// 16th register is used by system. TODO: Write more about its use.
    pub registers: [u8; 16],
    /// Address register
    pub index_register: u16,
    /// Program counter
    ///
    /// TODO: Write more about it
    pub program_counter: u16,
    /// Stack for basic procedure calls
    pub stack: Vec<u16>,
    /// Delay Timer
    pub delay_timer: u8,
    /// Sound timer
    pub sound_timer: u8
}