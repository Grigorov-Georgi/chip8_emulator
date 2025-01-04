use rand;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const FONT_SET_SIZE: usize = 80;
const FONT_SET: [u8; FONT_SET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80 // F
];

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const START_ADDR: u16 = 0x200;

pub struct Emu {
    pc: u16, // program counter
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS],
    i_reg: u16, // RAM indexing register
    stack: [u16; STACK_SIZE], // this can be implemented with std::collections::VecDeque and without sp
    sp: u16, // stack pointer
    keys: [bool; NUM_KEYS],
    dt: u8, // delay timer
    st: u8, // sound timer
}

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Emu {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };

        new_emu.ram[..FONT_SET_SIZE].copy_from_slice(&FONT_SET); // copy the font set into RAM

        new_emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.ram[..FONT_SET_SIZE].copy_from_slice(&FONT_SET);
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
    }

    pub fn tick(&mut self) {
        let op = self.fetch_opcode();

        // decode & execute
        self.execute(op);
    }

    // opcode is exactly 2 bytes long and the operands become part of the opcode
    fn fetch_opcode(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        let op = (higher_byte << 8) | lower_byte; // 1111 0000 << 8 | 1111 0000 = 1111 0000 1111 0000
        self.pc += 2;
        op
    }

    fn execute(&mut self, op: u16) {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => return, // Nop - do nothing
            (0, 0, 0xE, 0) => { // CLS - clear screen
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            (0, 0, 0xE, 0xE) => { // RET - return from subroutine
                let ret_addr = self.pop();
                self.pc = ret_addr;
            }
            (1, _, _, _) => { // JMP NNN - jump to subroutine
                //TODO: change to 0xFFF if that does not work but it should be the same
                let nnn = op & 0x0FFF;
                self.pc = nnn;
            }
            (2, _, _, _) => { // CALL NNN - call subroutine
                let nnn = op & 0x0FFF;
                self.push(self.pc);
                self.pc = nnn;
            }
            (3, _, _, _) => { // SKIP VX == NN
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
            }
            (4, _, _, _) => { // SKIP VX != NN
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            }
            (5, _, _, 0) => { // SKIP VX == VY
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            }
            (6, _, _, _) => { // VX == NN
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                self.v_reg[x] = nn;
            }
            (7, _, _, _) => { // VX += NN
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn); // preventing overflow panic
            }
            (8, _, _, 0) => { // VX = VY
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] = self.v_reg[y];
            }
            (8, _, _, 1) => { // VX |= VY bitwise OR
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] |= self.v_reg[y];
            }
            (8, _, _, 2) => { // VX &= VY bitwise AND
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] &= self.v_reg[y];
            }
            (8, _, _, 3) => { // VX ^= VY bitwise XOR
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.v_reg[x] ^= self.v_reg[y];
            }
            (8, _, _, 4) => { // VX += VY
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if carry { 1 } else { 0 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            (8, _, _, 5) => { // VX += VY
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            (8, _, _, 6) => { // VX >>= 1
                let x = digit2 as usize;
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            }
            (8, _, _, 7) => { // VX = VY - VX
                let x = digit2 as usize;
                let y = digit3 as usize;

                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }
            (8, _, _, 0xE) => { // VX <<= 1
                let x = digit2 as usize;
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            }
            (9, _, _, 0) => { // VX != VY
                let x = digit2 as usize;
                let y = digit3 as usize;

                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => { // I = NNN
                let nnn = op & 0x0FFF;
                self.i_reg = nnn;
            }
            (0xB, _, _, _) => { // JMP V0 + NNN
                let nnn = op & 0x0FFF;
                self.pc = (self.v_reg[0] as u16) + nnn;
            }
            (0xC, _, _, _) => { // VX = rand() & NN
                let x = digit2 as usize;
                let nn = (op & 0x00FF) as u8;
                let rng: u8 = rand::random();
                self.v_reg[x] = rng & nn;
            }
            (0xD, _, _, _) => { // DRAW
                // Get the (x, y) coords for our sprite
                let x_coord = self.v_reg[digit2 as usize] as u16;
                let y_coord = self.v_reg[digit3 as usize] as u16;

                // The last digit determines how many rows high our sprite is
                let num_rows = digit4;

                // Keep track if any pixels were flipped
                let mut flipped = false;

                // Iterate over each row of our sprite
                for y_line in 0..num_rows {

                    // Determine which memory address our row's data is stored
                    let addr = self.i_reg + y_line as u16;
                    let pixels = self.ram[addr as usize];

                    // Iterate over each column in our row
                    for x_line in 0..8 {

                        // Use a mask to fetch current pixel's bit. Only flip if a 1
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {

                            // Sprites should wrap around screen, so apply modulo
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            // Get our pixel's index for our 1D screen array
                            let idx = x + SCREEN_WIDTH * y;

                            // Check if we're about to flip the pixel and set
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }

                // Populate VF register
                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            }
            (0xE, _, 9, 0xE) => { // SKIP KEY PRESS
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                if key {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 1) => { // SKIP KEY RELEASE
                let x = digit2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                if !key {
                    self.pc += 2;
                }
            }
            (0xF, _, 0, 7) => { // VX = DT
                let x = digit2 as usize;
                self.v_reg[x] = self.dt;
            }
            (0xF, _, 0, 0xA) => { // WAIT KEY
                let x = digit2 as usize;
                let mut pressed = false;

                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    // Redo opcode
                    self.pc -= 2;
                }
            }
            (0xF, _, 1, 5) => { // DT = VX
                let x = digit2 as usize;
                self.dt = self.v_reg[x];
            }
            (0xF, _, 1, 8) => { // ST = VX
                let x = digit2 as usize;
                self.st = self.v_reg[x];
            }
            (0xF, _, 1, 0xE) => { // I += VX
                let x = digit2 as usize;
                let vx = self.v_reg[x] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
            }
            (0xF, _, 2, 9) => { // I = FONT
                let x = digit2 as usize;
                let c = self.v_reg[x] as u16;
                self.i_reg = c * 5;
            }
            (0xF, _, 3, 3) => { // BCD
                let x = digit2 as usize;
                let vx = self.v_reg[x] as f32;

                let hundreds = (vx / 100.0).floor() as u8;
                let tens = ((vx / 10.0) % 10.0).floor() as u8;
                let ones = (vx % 10.0) as u8;

                self.ram[self.i_reg as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
            }
            (0xF, _, 5, 5) => { // STORE V0 - VX
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.ram[i + idx] = self.v_reg[idx];
                }
            }
            (0xF, _, 6, 5) => { // LOAD V0 - VX
                let x = digit2 as usize;
                let i = self.i_reg as usize;
                for idx in 0..=x {
                    self.v_reg[idx] = self.ram[i + idx];
                }
            }

            (_, _, _, _) => panic!("Unimplemented opcode: {}", op),
        }
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                todo!("beep")
            }

            self.st -= 1;
        }
    }

    pub fn get_display(&self) -> &[bool] {
        &self.screen
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn load_game(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}

