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
