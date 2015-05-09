// TGS emulator
// Made by Derecho

extern crate libc;

use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::env;
use std::process;
use libc::funcs::posix88::unistd;

struct TGS {
    registers_gp: [u8; 8],
    registers_bt: [u8; 2],
    registers_dp: [u8; 4],
    register_pc: u8,
    register_cr: u8,
    ui: UI,
}
impl TGS {
    fn instruct(&mut self, opcode: u8, operand_left: u8, operand_right: u8) -> u8 {
        self.register_pc += 1;

        // All instructions that allow for either a register or direct value to
        // be specified as the second operand, make this distinction in the
        // least significant bit of their opcode. Only the branching
        // instructions, which all have 0101 in their most significant nibble,
        // do not conform to this and do not use the second operand.
        let value_right = if ((opcode & 0b11110000) != 0b01010000) & ((opcode & 1) == 0) {
            self.load_register(operand_right)
        }
        else {
            operand_right
        };

        // Ignore least significant bit
        match opcode & 0b11111110 {
            0b00010000 => {
                // ADD
                let result = self.load_register(operand_left) + value_right;
                self.store_register(operand_left, result);
            }

            0b00010010 => {
                // SUB
                let result = self.load_register(operand_left) - value_right;
                self.store_register(operand_left, result);
            }

            0b00100000 => {
                // LSH
                let result = self.load_register(operand_left) << value_right;
                self.store_register(operand_left, result);
            }

            0b00100010 => {
                // RSH
                let result = self.load_register(operand_left) >> value_right;
                self.store_register(operand_left, result);
            }

            0b00110000 => {
                // AND
                let result = self.load_register(operand_left) & value_right;
                self.store_register(operand_left, result);
            }

            0b00110010 => {
                // OR
                let result = self.load_register(operand_left) | value_right;
                self.store_register(operand_left, result);
            }

            0b00110100 => {
                // XOR
                let result = self.load_register(operand_left) ^ value_right;
                self.store_register(operand_left, result);
            }

            0b01000000 => {
                // CMP
                let result = self.load_register(operand_left) - value_right;
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
                self.store_register(operand_left, value_right);
            }

            _          => panic!("Invalid opcode")
        }

        self.register_pc
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

    fn update_display(&self) {
        self.ui.update_display(self.registers_dp);
    }
}

struct UI;
impl UI {
    fn update_display(&self, dp: [u8; 4]) {
        //  _   _   _   _
        // |_| |_| |_| |_|
        // |_| |_| |_| |_| 
        //
        //  1
        // 6 2
        //  7
        // 5 3
        //  4

        print!("\x1B[0;0H");  // Reset cursor

        // Line 1
        print!("  ");
        if (dp[3] & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        print!("   ");
        if (dp[2] & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        print!("   ");
        if (dp[1] & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        print!("   ");
        if (dp[0] & 1) == 1 {
            print!("_");
        }

        print!("\n");
        io::stdout().flush().ok();

        // Line 2
        print!(" ");
        if ((dp[3] >> 5) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[3] >> 6) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[3] >> 1) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((dp[2] >> 5) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[2] >> 6) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[2] >> 1) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((dp[1] >> 5) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[1] >> 6) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[1] >> 1) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((dp[0] >> 5) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[0] >> 6) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[0] >> 1) & 1) == 1 {
            print!("|");
        }

        print!("\n");
        io::stdout().flush().ok();

        // Line 3
        print!(" ");
        if ((dp[3] >> 4) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[3] >> 3) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[3] >> 2) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((dp[2] >> 4) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[2] >> 3) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[2] >> 2) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((dp[1] >> 4) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[1] >> 3) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[1] >> 2) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        print!(" ");
        if ((dp[0] >> 4) & 1) == 1 {
            print!("|");
        }
        else {
            print!(" ");
        }
        if ((dp[0] >> 3) & 1) == 1 {
            print!("_");
        }
        else {
            print!(" ");
        }
        if ((dp[0] >> 2) & 1) == 1 {
            print!("|");
        }

        print!("\n");
        io::stdout().flush().ok();
    }
}

fn main() {
    if env::args().count() != 2 {
        println!("Usage: {} <rom.bin>", env::args().next().unwrap());
        process::exit(1);
    }

    // Read ROM file
    let filename = env::args().skip(1).next().unwrap();
    let mut file = File::open(filename).ok().expect("Error opening ROM file");
    let mut program: Vec<[u8; 3]> = Vec::new();
    let mut buf: [u8; 3] = [0; 3];
    loop {
        let n = file.read(&mut buf).ok().unwrap();
        if n == 0 {
            // Finished reading file
            break;
        }
        else if n != 3 {
            panic!("Invalid ROM file");
        }
        program.push(buf);
    }

    let ui = UI;

    let mut tgs = TGS {
        registers_gp: [0; 8],
        registers_bt: [0; 2],
        registers_dp: [0; 4],
        register_pc: 0,
        register_cr: 0,
        ui: ui
    };

    print!("\x1B[?25l");  // Hide cursor
    print!("\x1B[2J");  // Clear screen

    let mut pc = 0;
    loop {
        pc = tgs.instruct(program[pc][0], program[pc][1], program[pc][2]) as usize;
        tgs.update_display();

        // Sleep for 2uS to approximate 500kHz clock. Real clock speed will be less due to time
        // taken by all calls in this loop.
        unsafe {
            unistd::usleep(2);
        }
    }
}
