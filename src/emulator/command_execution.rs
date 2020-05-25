use crate::emulator::opcode::OpCode;
use crate::emulator::Emulator;
use OpCode::*;

impl Emulator {
    /// Execute opcode
    ///
    /// # Parameters:
    /// - 'opcode` - opcode to execute
    pub fn execute_opcode(&mut self, opcode: OpCode) {
        match opcode {
            _NativeCall { target: _ } => panic!("Called a _NativeCall OpCode, which is deprecated"),
            ClearScreen => todo!(),
            Return => self.ret(),
            Goto { target } => self.goto(target),
            Subroutine { target } => self.subroutine(target),
            SkipNextIfRegEqualToConst { register, constant } => {
                if self.get_reg(register) == constant {
                    self.skip()
                }
            }
            SkipNextIfRegNotEqualToConst { register, constant } => {
                if self.get_reg(register) != constant {
                    self.skip()
                }
            }
            SkipNextIfRegEqualToReg {
                register_x,
                register_y,
            } => {
                let rx = self.get_reg(register_x);
                let ry = self.get_reg(register_y);
                if rx == ry {
                    self.skip()
                }
            }
            RegSetConst { register, constant } => self.set_reg(register, constant),
            RegAddConst { register, constant } => {
                let rx = self.get_reg(register);
                self.set_reg(register, rx + constant);
            }
            RegMov {
                register_x,
                register_y,
            } => self.registers[register_x as usize] = self.registers[register_y as usize],
            RegBitwiseOr {
                register_x,
                register_y,
            } => {
                let rx = self.get_reg(register_x);
                let ry = self.get_reg(register_y);
                self.set_reg(register_x, rx | ry);
            }
            RegBitwiseAnd {
                register_x,
                register_y,
            } => {
                let rx = self.get_reg(register_x);
                let ry = self.get_reg(register_y);
                self.set_reg(register_x, rx & ry)
            }
            RegBitwiseXor {
                register_x,
                register_y,
            } => {
                let rx = self.get_reg(register_x);
                let ry = self.get_reg(register_y);
                self.set_reg(register_x, rx ^ ry)
            }
            RegAdd {
                register_x,
                register_y,
            } => {
                let rx = self.get_reg(register_x);
                let ry = self.get_reg(register_y);
                self.set_reg(register_x, rx + ry)
            }
            RegSub {
                register_x,
                register_y,
            } => {
                let rx = self.get_reg(register_x);
                let ry = self.get_reg(register_y);
                self.set_reg(register_x, rx - ry);
            }
            RegRightShift { register } => {
                let rx = self.get_reg(register);
                self.set_reg(0xF, rx % (1 << 1));
                self.set_reg(register, rx >> 1);
            }
            RegReverseSub {
                register_x,
                register_y,
            } => {
                let rx = self.get_reg(register_x);
                let ry = self.get_reg(register_y);
                self.registers[register_x as usize] = ry - rx;
            }
            RegLeftShift { register } => {
                let rx = self.get_reg(register);
                self.set_reg(0xF, rx >> 7);
                self.set_reg(0xF, rx << 1);
            }
            SkipNextIfRegNotEqualToReg {
                register_x: _,
                register_y: _,
            } => todo!(),
            Mem { target } => self.index_register = target,
            JumpRegZero { target } => self.goto(self.get_reg(0) as u16 + target),
            RandToReg {
                register: _,
                constant: _,
            } => todo!(),
            DisplaySprite {
                coord_x: _,
                coord_y: _,
                height: _,
            } => todo!(),
            SkipNextIfRegKeyPressed { register: _ } => todo!(),
            SkipNextIfRegKeyNotPressed { register: _ } => todo!(),
            SetRegToDelayTimer { register } => self.set_reg(register, self.delay_timer),
            SetRegToKeyPressed { register: _ } => todo!(),
            SetDelayTimerToReg { register } => self.delay_timer = self.get_reg(register),
            SetSoundTimerToReg { register } => self.sound_timer = self.get_reg(register),
            MemAddReg { register } => {
                self.index_register += self.get_reg(register) as u16;
            }
            MemMoveToRegChar { register: _ } => todo!(),
            StoreBCD { register: _ } => todo!(),
            RegDump { register } => {
                for i in 0..=register {
                    self.memory[self.index_register as usize + i as usize] = self.get_reg(i)
                }
            }
            RegLoad { register } => {
                for i in 0..=register {
                    self.set_reg(i, self.memory[self.index_register as usize + i as usize])
                }
            }
        }
    }

    /// Move program counter to a `dest`
    pub fn goto(&mut self, dest: u16) {
        self.program_counter = dest;
    }

    /// Start a subroutine in `dent`
    pub fn subroutine(&mut self, dest: u16) {
        self.stack.push(self.program_counter);
        self.goto(dest);
    }

    /// Return from a subroutine
    pub fn ret(&mut self) {
        self.program_counter = self.stack.pop().unwrap();
    }

    /// Skip next instruction
    pub fn skip(&mut self) {
        self.program_counter += 2;
    }
}

#[cfg(test)]
pub mod tests {}
