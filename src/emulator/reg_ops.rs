use crate::emulator::Emulator;

impl Emulator {
    pub fn get_reg(&self, register: u8) -> u8 {
        self.registers[register as usize]
    }

    pub fn set_reg(&mut self, register: u8, value: u8) {
        self.registers[register as usize] = value
    }

    pub fn reg_op<T>(&mut self, register: u8, x: T)
    where
        T: Fn(&mut u8),
    {
        x(self.registers.get_mut(register as usize).unwrap())
    }
}
