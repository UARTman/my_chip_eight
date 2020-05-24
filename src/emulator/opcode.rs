/// Represents a processor command.
///
/// Exists for purposes of pattern matching.
///
/// Internal symbols are:
/// - `NNN`: address, represented by type `u16`
/// - `NN`: 8-bit constant, represented by type `u8`
/// - `N`: 4-bit constant, represented by type `u8`
/// - `X`/`Y`: 4-bit register identifier, represented by type `u8`
/// - `PC`: Program counter, represented by variable `Emulator::program_counter`
/// - `I`: Index Register, represented by variable `Emulator::index_register`
/// - `VN`: `N`-th register, represented by variable `Emulator::registers[N]`
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum OpCode {
    /// `0x0NNN`, where
    /// - `NNN` is `target`
    ///
    /// Exists only for reference. Should not be called.
    _NativeCall {
        target: u16
    },
    /// `0x00E0`
    ///
    /// Clears screen
    ClearScreen,
    /// `0x00EE`
    ///
    /// Returns from a subroutine
    Return,
    /// `0x1NNN`, where
    /// - `NNN` is `target`
    ///
    /// Jumps to address `NNN`
    Goto {
        target: u16
    },
    /// `0x2NNN`, where
    /// - `NNN` is `target`
    ///
    /// Calls subroutine at `NNN`
    Subroutine {
        target: u16
    },
    /// `0x3XNN`, where
    /// - `X` is `register`
    /// - `NN` is `constant`
    ///
    /// Skips the next instruction if `VX` equals `NN`.
    /// (Usually the next instruction is a jump to skip a code block)
    SkipNextIfRegEqualToConst {
        register: u8,
        constant: u8,
    },
    /// `0x4XNN`, where
    /// - `X` is `register`
    /// - `NN` is `constant`
    ///
    /// Skips the next instruction if `VX` doesn't equal `NN`.
    /// (Usually the next instruction is a jump to skip a code block)
    SkipNextIfRegNotEqualToConst {
        register: u8,
        constant: u8,
    },
    /// `0x5XY0`, where
    /// - `X` is `register_x`
    /// - `Y` is `register_y`
    ///
    /// Skips the next instruction if `VX` equals `VY`.
    /// (Usually the next instruction is a jump to skip a code block)
    SkipNextIfRegEqualToReg {
        register_x: u8,
        register_y: u8,
    },
    /// `0x6XNN`, where
    /// - `X` is `register`
    /// - `Y` is `constant`
    ///
    /// Sets `VX` to `NN`
    RegSetConst {
        register: u8,
        constant: u8,
    },
    /// `0x7XNN`, where
    /// - `X` is `register`
    /// - `Y` is `constant`
    ///
    /// Adds `NN` to `VX`
    RegAddConst {
        register: u8,
        constant: u8,
    },
    /// `0x8XY0`, where
    /// - `X` is `register_x`
    /// - `Y` is `register_y`
    ///
    /// Sets `VX` to a value of `VY`
    RegMov {
        register_x: u8,
        register_y: u8,
    },
    /// `0x8XY1`, where
    /// - `X` is `register_x`
    /// - `Y` is `register_y`
    ///
    /// Sets `VX` to a `VX | VY`
    /// (Bitwise Or)
    RegBitwiseOr {
        register_x: u8,
        register_y: u8,
    },
    /// `0x8XY2`, where
    /// - `X` is `register_x`
    /// - `Y` is `register_y`
    ///
    /// Sets `VX` to a `VX & VY`
    /// (Bitwise And)
    RegBitwiseAnd {
        register_x: u8,
        register_y: u8,
    },
    /// `0x8XY3`, where
    /// - `X` is `register_x`
    /// - `Y` is `register_y`
    ///
    /// Sets `VX` to a `VX ^ VY`
    RegBitwiseXor {
        register_x: u8,
        register_y: u8,
    },
    /// `0x8XY4`, where
    /// - `X` is `register_x`
    /// - `Y` is `register_y`
    ///
    /// Sets `VX` to a `VX + VY`
    /// `VF` is set to 1 when there's a carry, and to 0 when there isn't. TODO: Try to understand what this means
    RegAdd {
        register_x: u8,
        register_y: u8,
    },
    /// `0x8XY5`, where
    /// - `X` is `register_x`
    /// - `Y` is `register_y`
    ///
    /// Sets `VX` to a `VX - VY`
    /// `VF` is set to 0 when there's a borrow, and to 1 when there isn't. TODO: Try to understand what this means
    RegSub {
        register_x: u8,
        register_y: u8,
    },
    /// `0x8XY6`, where
    /// - `X` is `register_x`
    /// - `Y` is not used
    ///
    /// Sets `VX` to a `VX >> 1`
    /// `VF` is set to `VX`'s least significant bit. TODO: Try to understand what this means
    RegRightShift {
        register_x: u8,
        // register_y: u8,
    },
    /// `0x8XY7`, where
    /// - `X` is `register_x`
    /// - `Y` is `register_y`
    ///
    /// Sets `VX` to a `VX - VY`
    /// `VF` is set to 0 when there's a borrow, and to 1 when there isn't. TODO: Try to understand what this means
    RegReverseSub {
        register_x: u8,
        register_y: u8,
    },
    /// `0x8XYE`, where
    /// - `X` is `register_x`
    /// - `Y` is not used
    ///
    /// Sets `VX` to a `VX << 1`
    /// `VF` is set to `VX`'s most significant bit. TODO: Try to understand what this means
    RegLeftShift {
        register_x: u8,
        // register_y: u8,
    },
    /// `0x9XY0`, where
    /// - `X` is `register_x`
    /// - `Y` is `register_y`
    ///
    /// Skips the next instruction if `VX` doesn't equal `VY`.
    /// (Usually the next instruction is a jump to skip a code block)
    SkipNextIfRegNotEqualToReg {
        register_x: u8,
        register_y: u8,
    },
    /// `0xANNN`, where
    /// - `NNN` is `target`
    ///
    /// Sets `I` to the address `NNN`
    Mem {
        target: u16
    },
    /// `0xBNNN`, where
    /// - `NNN` is `target`
    ///
    /// Jumps to the address `NNN + V0`
    JumpRegZero {
        target: u16
    },
    /// `0xCXNN`, where
    /// - `X` is `register`
    /// - `NN` is `constant`
    ///
    /// Sets `VX` to `rand() & NN` where rand is in (0..255).
    RandToReg {
        constant: u8,
        register: u8
    },
    /// `0xDXYN`, where
    /// - `X` is `coord_x`
    /// - `Y` is `coord_y`
    /// - `N` is `height`
    ///
    /// Draws a sprite at coordinate `(VX, VY)` that has a width of 8 pixels and a height of `N` pixels.
    /// Each row of 8 pixels is read as bit-coded starting from memory location I;
    /// I value doesn’t change after the execution of this instruction.
    /// VF is set to 1 if any screen pixels are flipped from set, to unset when the sprite is drawn,
    /// and to 0 if that doesn’t happen
    DisplaySprite {
        coord_x: u8,
        coord_y: u8,
        height: u8
    },
    /// `0xEX9E`, where
    /// - `X` is `register`
    ///
    /// Skips the next instruction if the key stored in `VX` is pressed.
    /// (Usually the next instruction is a jump to skip a code block)
    SkipNextIfRegKeyPressed {
        register: u8
    },
    /// `0xEXA1`, where
    /// - `X` is `register`
    ///
    /// Skips the next instruction if the key stored in `VX` isn't pressed.
    /// (Usually the next instruction is a jump to skip a code block)
    SkipNextIfRegKeyNotPressed {
        register: u8
    },
    /// `0xFX07`, where
    /// - `X` is `register`
    ///
    /// Sets `VX` to the value of the delay timer.
    SetRegToDelayTimer {
        register: u8
    },
    /// `0xFX0A`, where
    /// - `X` is `register`
    ///
    /// Sets `VX` to pressed key.
    /// It awaits key press.
    SetRegToKeyPressed {
        register: u8
    },
    ///  `0xFX15`, where
    /// - `X` is `register`
    ///
    /// Sets delay timer to `VX`
    SetDelayTimerToReg {
        register: u8
    },
    /// `0xFX18`, where
    /// - `X` is `register`
    ///
    /// Sets sound timer to `VX`
    SetSoundTimerToReg {
        register: u8
    },
    /// `0xFX1E`, where
    /// - `X` is `register`
    ///
    /// Adds `VX` to `I`.
    /// `VF` is set to 1 when there is a range overflow (I+VX>0xFFF),
    /// and to 0 when there isn't.
    MemAddReg {
        register: u8
    },
    /// `0xFX29`, where
    /// - `X` is `register`
    ///
    /// Sets `I` to the location of the sprite for the character in `VX`.
    /// Characters 0-F (in hexadecimal) are represented by a 4x5 font.
    MemMoveCharReg {
        register: u8
    },
    /// `0xFX33`, where
    /// - `X` is `register`
    ///
    /// Stores the binary-coded decimal representation of VX,
    /// with the most significant of three digits at the address in I,
    /// the middle digit at I plus 1, and the least significant digit at I plus 2.
    /// (In other words, take the decimal representation of VX,
    /// place the hundreds digit in memory at location in I, the tens digit at location I+1,
    /// and the ones digit at location I+2.) TODO: Understand how this works
    StoreBCD {
        register: u8
    },
    /// `0xFX55`, where
    /// - `X` is `register`
    ///
    /// Stores `V0` to `VX` (including `VX`) in memory starting at address `I`.
    /// The offset from `I` is increased by 1 for each value written, but `I` itself is left unmodified.
    RegDump {
        register: u8
    },
    /// `0xFX65`, where
    /// - `X` is `register`
    ///
    /// Fills `V0` to `VX` (including `VX`) with values from memory starting at address I.
    /// The offset from `I` is increased by 1 for each value written, but `I` itself is left unmodified.
    RegLoad {
        register: u8
    }
}