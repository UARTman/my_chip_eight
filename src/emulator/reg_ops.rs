use crate::emulator::Emulator;

impl Emulator {
    /// Get register at `register`
    pub fn get_reg(&self, register: u8) -> u8 {
        self.registers[register as usize]
    }

    /// Get register at `register` to `value`
    pub fn set_reg(&mut self, register: u8, value: u8) {
        self.registers[register as usize] = value
    }
}
