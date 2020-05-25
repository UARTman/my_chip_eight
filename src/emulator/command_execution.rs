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
                self.set_reg(register, rx << 1);
            }
            SkipNextIfRegNotEqualToReg {
                register_x,
                register_y,
            } => {
                let rx = self.get_reg(register_x);
                let ry = self.get_reg(register_y);
                if rx != ry {
                    self.skip()
                }
            }
            Mem { target } => self.index_register = target,
            JumpRegZero { target } => self.goto(self.get_reg(0) as u16 + target),
            RandToReg { register, constant } => {
                self.set_reg(register, self.rng.clone().rand() & constant)
            }
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
pub mod tests {
    use crate::emulator::opcode::OpCode::*;
    use crate::emulator::Emulator;

    #[test]
    #[should_panic]
    fn test_native_call() {
        let mut e = Emulator::default();
        e.execute_opcode(_NativeCall { target: 0 })
    }

    /// Test ClearScreen execution
    #[test]
    #[should_panic]
    fn test_clear_screen() {
        let mut e = Emulator::default();
        e.execute_opcode(ClearScreen);
    }

    ///Test Return execution
    #[test]
    fn test_return() {
        let mut e = Emulator::default();
        e.stack.push(5);
        e.execute_opcode(Return);
        assert_eq!(e.program_counter, 5);
    }

    /// Test Goto execution
    #[test]
    fn test_goto() {
        let mut e = Emulator::default();
        e.execute_opcode(Goto { target: 4 });
        assert_eq!(e.program_counter, 4);
    }

    /// Test Subroutine execution
    #[test]
    fn test_subroutine() {
        let mut e = Emulator::default();
        e.execute_opcode(Subroutine { target: 4 });
        assert_eq!(e.program_counter, 4);
        e.execute_opcode(Return);
        assert_eq!(e.program_counter, 0);
    }

    /// Test SkipNextIfRegEqualToConst execution
    #[test]
    fn test_skip_reg_eq_const() {
        let mut e = Emulator::default();
        e.set_reg(0, 4);
        e.execute_opcode(SkipNextIfRegEqualToConst {
            register: 0,
            constant: 4,
        });
        assert_eq!(e.program_counter, 2);
        e.execute_opcode(SkipNextIfRegEqualToConst {
            register: 0,
            constant: 0,
        });
        assert_eq!(e.program_counter, 2);
    }

    /// Test SkipNextIfReqNotEqualToConst execution
    #[test]
    fn test_skip_reg_neq_const() {
        let mut e = Emulator::default();
        e.set_reg(0, 4);
        e.execute_opcode(SkipNextIfRegNotEqualToConst {
            register: 0,
            constant: 0,
        });
        assert_eq!(e.program_counter, 2);
        e.execute_opcode(SkipNextIfRegNotEqualToConst {
            register: 0,
            constant: 4,
        });
        assert_eq!(e.program_counter, 2);
    }

    /// Test SkipNextIfRegEqualToReg execution
    #[test]
    fn test_skip_reg_eq_reg() {
        let mut e = Emulator::default();
        e.set_reg(0, 3);
        e.set_reg(1, 3);
        e.execute_opcode(SkipNextIfRegEqualToReg {
            register_x: 0,
            register_y: 1,
        });
        assert_eq!(e.program_counter, 2);
        e.set_reg(1, 2);
        e.execute_opcode(SkipNextIfRegEqualToReg {
            register_x: 0,
            register_y: 1,
        })
    }

    /// Test RegSetConst execution
    #[test]
    fn test_reg_set_const() {
        let mut e = Emulator::default();
        e.execute_opcode(RegSetConst {
            register: 0,
            constant: 64,
        });
        assert_eq!(e.get_reg(0), 64);
    }

    /// Test RegAddConst execution
    #[test]
    fn test_reg_add_const() {
        let mut e = Emulator::default();
        e.set_reg(0, 1);
        e.execute_opcode(RegAddConst {
            register: 0,
            constant: 1,
        });
        assert_eq!(e.get_reg(0), 2);
    }

    /// Test RegMov execution
    #[test]
    fn test_reg_mov() {
        let mut e = Emulator::default();
        e.set_reg(0, 3);
        e.execute_opcode(RegMov {
            register_x: 1,
            register_y: 0,
        });
        assert_eq!(e.get_reg(0), 3);
    }

    ///Test RegBitwiseOr execution
    #[test]
    fn test_reg_bit_or() {
        let mut e = Emulator::default();
        e.set_reg(0, 34);
        e.set_reg(1, 224);
        e.execute_opcode(RegBitwiseOr {
            register_x: 0,
            register_y: 1,
        });
        assert_eq!(e.get_reg(0), 34 | 224);
    }

    /// Test RegBitwiseAnd execution
    #[test]
    fn test_reg_bit_and() {
        let mut e = Emulator::default();
        e.set_reg(0, 34);
        e.set_reg(1, 224);
        e.execute_opcode(RegBitwiseAnd {
            register_x: 0,
            register_y: 1,
        });
        assert_eq!(e.get_reg(0), 34 & 224);
    }

    /// Test RegBitwiseXor execution
    #[test]
    fn test_reg_bit_xor() {
        let mut e = Emulator::default();
        e.set_reg(0, 0b1100);
        e.set_reg(1, 0b1011);
        e.execute_opcode(RegBitwiseXor {
            register_x: 0,
            register_y: 1,
        });
        assert_eq!(e.get_reg(0), 0b0111);
    }

    /// Test RegAdd execution
    #[test]
    fn test_reg_add() {
        let mut e = Emulator::default();
        e.set_reg(0, 5);
        e.set_reg(1, 6);
        e.execute_opcode(RegAdd {
            register_x: 0,
            register_y: 1,
        });
        assert_eq!(e.get_reg(0), 11);
    }

    /// Test RegSub execution
    #[test]
    fn test_reg_sub() {
        let mut e = Emulator::default();
        e.set_reg(0, 6);
        e.set_reg(1, 5);
        e.execute_opcode(RegSub {
            register_x: 0,
            register_y: 1,
        });
        assert_eq!(e.get_reg(0), 1);
    }

    /// Test RegRightShift execution
    #[test]
    fn test_reg_rshift() {
        let mut e = Emulator::default();
        e.set_reg(0, 0b101);
        e.execute_opcode(RegRightShift { register: 0 });
        assert_eq!(e.get_reg(15), 1);
        assert_eq!(e.get_reg(0), 0b10);
        e.set_reg(0, 0b100);
        e.execute_opcode(RegRightShift { register: 0 });
        assert_eq!(e.get_reg(15), 0);
        assert_eq!(e.get_reg(0), 0b10);
    }

    /// Test RegReverseRub execution
    #[test]
    fn test_reg_reverse_sub() {
        let mut e = Emulator::default();
        e.set_reg(0, 5);
        e.set_reg(1, 6);
        e.execute_opcode(RegReverseSub {
            register_x: 0,
            register_y: 1,
        });
        assert_eq!(e.get_reg(0), 1);
    }

    /// Test RegLeftShift execution
    #[test]
    fn test_reg_lshift() {
        let mut e = Emulator::default();
        e.set_reg(0, 0b00001000);
        e.execute_opcode(RegLeftShift { register: 0 });
        assert_eq!(e.get_reg(15), 0);
        assert_eq!(e.get_reg(0), 0b10000);
        e.set_reg(0, 0b10001001);
        e.execute_opcode(RegLeftShift { register: 0 });
        assert_eq!(e.get_reg(15), 1);
        assert_eq!(e.get_reg(0), 0b10010)
    }

    /// Test SkipNextIfRegNotEqualToReg execution
    #[test]
    fn test_skip_reg_neq_reg() {
        let mut e = Emulator::default();
        e.set_reg(0, 1);
        e.execute_opcode(SkipNextIfRegNotEqualToReg {
            register_x: 0,
            register_y: 1,
        });
        assert_eq!(e.program_counter, 2);
        e.set_reg(1, 1);
        e.execute_opcode(SkipNextIfRegNotEqualToReg {
            register_x: 0,
            register_y: 1,
        });
        assert_eq!(e.program_counter, 2);
    }

    /// Test Mem execution
    #[test]
    fn test_mem() {
        let mut e = Emulator::default();
        e.execute_opcode(Mem { target: 37 });
        assert_eq!(e.index_register, 37);
    }

    /// Test JumpRegZero execution
    #[test]
    fn test_jump_reg0() {
        let mut e = Emulator::default();
        e.set_reg(0, 54);
        e.execute_opcode(JumpRegZero { target: 46 });
        assert_eq!(e.program_counter, 100);
    }

    /// Test RandToReg execution
    #[test]
    fn test_rand2reg() {
        let mut e = Emulator::default();
        e.execute_opcode(RandToReg {
            register: 0,
            constant: 54,
        });
        e.execute_opcode(RandToReg {
            register: 1,
            constant: 54,
        });
        assert_ne!(e.get_reg(0), e.get_reg(1));
    }

    /// Test DisplaySprite execution
    #[test]
    #[should_panic]
    fn test_display_sprite() {
        let mut e = Emulator::default();
        e.execute_opcode(DisplaySprite {
            coord_x: 0,
            coord_y: 0,
            height: 0,
        })
    }

    /// Test SkipNextIfRegKeyPressed execution
    #[test]
    #[should_panic]
    fn test_skip_key() {
        let mut e = Emulator::default();
        e.execute_opcode(SkipNextIfRegKeyPressed { register: 0 })
    }

    /// Test SkipNextIfRegKeyNotPressed execution
    #[test]
    #[should_panic]
    fn test_skip_not_key() {
        let mut e = Emulator::default();
        e.execute_opcode(SkipNextIfRegKeyNotPressed { register: 0 })
    }

    /// Test SetRegToDelayTimer execution
    #[test]
    fn test_reg2delay() {
        let mut e = Emulator::default();
        e.delay_timer = 3;
        e.execute_opcode(SetRegToDelayTimer { register: 0 });
        assert_eq!(e.get_reg(0), 3);
    }

    /// Test SetRegToKeyPressed execution
    #[test]
    fn test_key2reg() {}

    /// Test SetDelayTimerToReg execution
    #[test]
    fn test_delay2reg() {
        let mut e = Emulator::default();
        e.set_reg(0, 45);
        e.execute_opcode(SetDelayTimerToReg { register: 0 });
        assert_eq!(e.delay_timer, 45);
    }

    /// Test SetSoundTimerToReg execution
    #[test]
    fn test_sound2reg() {
        let mut e = Emulator::default();
        e.set_reg(0, 45);
        e.execute_opcode(SetSoundTimerToReg { register: 0 });
        assert_eq!(e.sound_timer, 45);
    }

    /// Test MemAddReg execution
    #[test]
    fn test_mem_add_reg() {
        let mut e = Emulator::default();
        e.set_reg(0, 45);
        e.index_register = 5;
        e.execute_opcode(MemAddReg { register: 0 });
        assert_eq!(e.index_register, 50);
    }

    /// Test MemMoveToCharReg execution
    #[test]
    #[should_panic]
    fn test_mem_move_char() {
        let mut e = Emulator::default();
        e.execute_opcode(MemMoveToRegChar { register: 0 })
    }

    /// Test StoreBCD execution
    #[test]
    #[should_panic]
    fn test_store_bcd() {
        let mut e = Emulator::default();
        e.execute_opcode(StoreBCD { register: 0 })
    }

    /// Test RegDump execution
    #[test]
    fn test_reg_dump() {
        let mut e = Emulator::default();
        e.set_reg(0, 1);
        e.set_reg(1, 2);
        e.set_reg(2, 3);
        e.execute_opcode(RegDump { register: 2 });
        assert_eq!(e.memory[0..3], [1, 2, 3])
    }

    /// Test RegLoad execution
    #[test]
    fn test_reg_load() {
        let mut e = Emulator::default();
        e.memory[0] = 1;
        e.memory[1] = 2;
        e.memory[2] = 3;
        e.execute_opcode(RegLoad { register: 2 });
        assert_eq!(e.registers[0..3], [1, 2, 3]);
    }
}
