/// Contains CHIP-8 Opcodes
pub mod opcode;

/// CHIP-8 Emulator
pub struct Emulator {
    /// Represents CHIP-8 memory: 4096 bytes
    ///
    /// Bytes 0x000-0x200 - Interpreter space
    /// Bytes 0x200-0xFFF - Working ROM and RAM
    pub memory: [u8; 4096],
    /// Represents CHIP-8 registers
    ///
    /// There are 16 registers
    ///
    /// 16th register is used as a carry flag
    pub registers: [u8; 16],
    /// Memory address register
    pub index_register: u16,
    /// Program counter
    ///
    /// Points to a current instruction
    pub program_counter: u16,
    /// Stack for basic subroutine calls
    pub stack: Vec<u16>,
    /// Delay Timer
    pub delay_timer: u8,
    /// Sound timer
    pub sound_timer: u8
}