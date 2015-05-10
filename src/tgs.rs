pub struct TGS<T: TGSUI> {
    registers_gp: [u8; 8],
    registers_bt: [u8; 2],
    registers_dp: [u8; 4],
    register_pc: u8,
    register_cr: u8,
    ui: T,
}
impl<T: TGSUI> TGS<T> {
    pub fn new(ui: T) -> TGS<T> {
        TGS {
            registers_gp: [0; 8],
            registers_bt: [0; 2],
            registers_dp: [0; 4],
            register_pc: 0,
            register_cr: 0,
            ui: ui
        }
    }

    pub fn instruct(&mut self, opcode: u8, operand_left: u8, operand_right: u8) -> u8 {
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
            x @ 0b00010000 ... 0b00010001 => self.registers_bt[(x - 0b00010000) as usize],
            x @ 0b00010010 ... 0b00010101 => self.registers_dp[(x - 0b00010010) as usize],
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

    pub fn update_display(&self) {
        self.ui.update_display(self.registers_dp);
    }

    pub fn update_buttons(&mut self) {
        self.ui.update_buttons(&mut self.registers_bt);
    }
}

pub trait TGSUI {
    fn init(&self);
    fn update_display(&self, dp: [u8; 4]);
    fn update_buttons(&self, bt: &mut [u8; 2]);
}
