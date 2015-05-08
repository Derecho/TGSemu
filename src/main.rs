// TGS emulator
// Made by Derecho

struct TGS {
    registers_gp: [u8; 8],
    registers_bt: [u8; 2],
    registers_dp: [u8; 4],
    register_pc: u8,
    register_cr: u8,
}
impl TGS {
    fn instruct(&mut self, opcode: u8, operand_left: u8, operand_right: u8) {
        match opcode {
            0b00010000 => {
                let result = self.add(self.load_register(operand_left), self.load_register(operand_right));
                self.store_register(operand_left, result);
            }
            0b00010001 => {
                let result = self.add(self.load_register(operand_left), operand_right);
                self.store_register(operand_left, result);
            }

            0b00010010 => {
                let result = self.sub(self.load_register(operand_left), self.load_register(operand_right));
                self.store_register(operand_left, result);
            }
            0b00010011 => {
                let result = self.sub(self.load_register(operand_left), operand_right);
                self.store_register(operand_left, result);
            }

            0b00100000 => {
                let result = self.lsh(self.load_register(operand_left), self.load_register(operand_right));
                self.store_register(operand_left, result);
            }
            0b00100001 => {
                let result = self.lsh(self.load_register(operand_left), operand_right);
                self.store_register(operand_left, result);
            }

            0b00100010 => {
                let result = self.rsh(self.load_register(operand_left), self.load_register(operand_right));
                self.store_register(operand_left, result);
            }
            0b00100011 => {
                let result = self.rsh(self.load_register(operand_left), operand_right);
                self.store_register(operand_left, result);
            }

            0b00110000 => {
                let result = self.and(self.load_register(operand_left), self.load_register(operand_right));
                self.store_register(operand_left, result);
            }
            0b00110001 => {
                let result = self.and(self.load_register(operand_left), operand_right);
                self.store_register(operand_left, result);
            }

            0b00110010 => {
                let result = self.or(self.load_register(operand_left), self.load_register(operand_right));
                self.store_register(operand_left, result);
            }
            0b00110011 => {
                let result = self.or(self.load_register(operand_left), operand_right);
                self.store_register(operand_left, result);
            }

            0b00110100 => {
                let result = self.xor(self.load_register(operand_left), self.load_register(operand_right));
                self.store_register(operand_left, result);
            }
            0b00110101 => {
                let result = self.xor(self.load_register(operand_left), operand_right);
                self.store_register(operand_left, result);
            }

            0b01000000 => {
                let result = self.cmp(self.load_register(operand_left), self.load_register(operand_right));
                self.register_cr = result;
            }
            0b01000001 => {
                let result = self.cmp(self.load_register(operand_left), operand_right);
                self.register_cr = result;
            }

            0b01010000 => {
                // BR
                self.register_pc = operand_left;
            }

            0b01010010 => {
                // BE
                if self.register_cr == 0 {
                    self.register_pc = operand_left;
                }
            }

            0b01010100 => {
                // BNE
                if self.register_cr != 0 {
                    self.register_pc = operand_left;
                }
            }

            0b01010110 => {
                // BG
                if self.register_cr > 127 {  // 2's complement -128...-1
                    self.register_pc = operand_left;
                }
            }

            0b01011000 => {
                // BL
                if self.register_cr < 128 {
                    self.register_pc = operand_left;
                }
            }

            0b01100000 => {
                // MOV
                let result = self.load_register(operand_right);
                self.store_register(operand_left, result);
            }

            0b01100001 => {
                // MOV
                self.store_register(operand_left, operand_right);
            }

            _          => panic!("Invalid opcode")
        }
    }

    fn load_register(&self, reg: u8) -> u8 {
        match reg {
            x @ 0b00000000 ... 0b00000111 => self.registers_gp[x as usize],
            x @ 0b00010000 ... 0b00010001 => self.registers_bt[(x & !0b00010000) as usize],
            x @ 0b00010010 ... 0b00010101 => self.registers_dp[(x & !0b00010010) as usize],
            0b00010110                    => self.register_pc,
            0b00010111                    => self.register_cr,
            _                             => panic!("Invalid register")
        }
    }

    fn store_register(&mut self, reg: u8, val: u8) {
        match reg {
            x @ 0b00000000 ... 0b00000111 => self.registers_gp[x as usize] = val,
            x @ 0b00010000 ... 0b00010001 => self.registers_bt[(x & !0b00010000) as usize] = val,
            x @ 0b00010010 ... 0b00010101 => self.registers_dp[(x & !0b00010010) as usize] = val,
            0b00010110                    => self.register_pc = val,
            0b00010111                    => self.register_cr = val,
            _                             => panic!("Invalid register")
        }
    }

    fn add(&self, a: u8, b: u8) -> u8 {
        a+b
    }

    fn sub(&self, a: u8, b: u8) -> u8 {
        a-b
    }

    fn lsh(&self, a: u8, b: u8) -> u8 {
        a<<b
    }

    fn rsh(&self, a: u8, b: u8) -> u8 {
        a>>b
    }

    fn and(&self, a: u8, b: u8) -> u8 {
        a&b
    }

    fn or(&self, a: u8, b: u8) -> u8 {
        a|b
    }

    fn xor(&self, a: u8, b: u8) -> u8 {
        a^b
    }

    fn cmp(&self, a: u8, b: u8) -> u8 {
        a-b
    }

}

fn main() {
    // TODO Load file

    let mut tgs = TGS {
        registers_gp: [0; 8],
        registers_bt: [0; 2],
        registers_dp: [0; 4],
        register_pc: 0,
        register_cr: 0
    };

    tgs.instruct(0b00010001, 0b00000000, 5);
}
