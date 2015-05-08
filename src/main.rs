// TGS emulator
// Made by Derecho

use std::io;
use std::io::Write;

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
                // ADD
                let result = self.load_register(operand_left) + self.load_register(operand_right);
                self.store_register(operand_left, result);
            }
            0b00010001 => {
                // ADD
                let result = self.load_register(operand_left) + operand_right;
                self.store_register(operand_left, result);
            }

            0b00010010 => {
                // SUB
                let result = self.load_register(operand_left) - self.load_register(operand_right);
                self.store_register(operand_left, result);
            }
            0b00010011 => {
                // SUB
                let result = self.load_register(operand_left) - operand_right;
                self.store_register(operand_left, result);
            }

            0b00100000 => {
                // LSH
                let result = self.load_register(operand_left) << self.load_register(operand_right);
                self.store_register(operand_left, result);
            }
            0b00100001 => {
                // LSH
                let result = self.load_register(operand_left) << operand_right;
                self.store_register(operand_left, result);
            }

            0b00100010 => {
                // RSH
                let result = self.load_register(operand_left) >> self.load_register(operand_right);
                self.store_register(operand_left, result);
            }
            0b00100011 => {
                // RSH
                let result = self.load_register(operand_left) >> operand_right;
                self.store_register(operand_left, result);
            }

            0b00110000 => {
                // AND
                let result = self.load_register(operand_left) & self.load_register(operand_right);
                self.store_register(operand_left, result);
            }
            0b00110001 => {
                // AND
                let result = self.load_register(operand_left) & operand_right;
                self.store_register(operand_left, result);
            }

            0b00110010 => {
                // OR
                let result = self.load_register(operand_left) | self.load_register(operand_right);
                self.store_register(operand_left, result);
            }
            0b00110011 => {
                // OR
                let result = self.load_register(operand_left) | operand_right;
                self.store_register(operand_left, result);
            }

            0b00110100 => {
                // XOR
                let result = self.load_register(operand_left) ^ self.load_register(operand_right);
                self.store_register(operand_left, result);
            }
            0b00110101 => {
                // XOR
                let result = self.load_register(operand_left) ^ operand_right;
                self.store_register(operand_left, result);
            }

            0b01000000 => {
                // CMP
                let result = self.load_register(operand_left) - self.load_register(operand_right);
                self.register_cr = result;
            }
            0b01000001 => {
                // CMP
                let result = self.load_register(operand_left) - operand_right;
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
                if self.register_cr < 128 {
                    self.register_pc = operand_left;
                }
            }

            0b01011000 => {
                // BL
                if self.register_cr > 127 {  // 2's complement -128...-1
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
            x @ 0b00010000 ... 0b00010001 => self.registers_bt[(x - 0b00010000) as usize] = val,
            x @ 0b00010010 ... 0b00010101 => self.registers_dp[(x - 0b00010010) as usize] = val,
            0b00010110                    => self.register_pc = val,
            0b00010111                    => self.register_cr = val,
            _                             => panic!("Invalid register")
        }
    }

    fn print_display(&self) {
        //  _   _   _   _
        // |_| |_| |_| |_|
        // |_| |_| |_| |_| 
        //
        //  1
        // 6 2
        //  7
        // 5 3
        //  4

        // Line 1
        print!("  ");
        if (self.registers_dp[3] & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        print!("   ");
        if (self.registers_dp[2] & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        print!("   ");
        if (self.registers_dp[1] & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        print!("   ");
        if (self.registers_dp[0] & 1) == 1 {
            print!("_");
        }

        print!("\n");
        io::stdout().flush().ok();

        // Line 2
        print!(" ");
        if ((self.registers_dp[3] >> 5) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[3] >> 6) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[3] >> 1) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((self.registers_dp[2] >> 5) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[2] >> 6) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[2] >> 1) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((self.registers_dp[1] >> 5) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[1] >> 6) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[1] >> 1) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((self.registers_dp[0] >> 5) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[0] >> 6) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[0] >> 1) & 1) == 1 {
            print!("|");
        }

        print!("\n");
        io::stdout().flush().ok();

        // Line 3
        print!(" ");
        if ((self.registers_dp[3] >> 4) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[3] >> 3) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[3] >> 2) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((self.registers_dp[2] >> 4) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[2] >> 3) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[2] >> 2) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((self.registers_dp[1] >> 4) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[1] >> 3) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[1] >> 2) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((self.registers_dp[0] >> 4) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[0] >> 3) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((self.registers_dp[0] >> 2) & 1) == 1 {
            print!("|");
        }

        print!("\n");
        io::stdout().flush().ok();
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

    // Display "hi"
    tgs.instruct(0x61, 0x13, 0x06);
    tgs.instruct(0x61, 0x14, 0x74);
    tgs.print_display();
}
