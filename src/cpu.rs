// The Cpu struct represents the state of the cpu for the chip-8 emulation including
// memory, registers, and graphics
pub struct Cpu {
	opcode: u16,
    
    memory: [u8; 4096],

    register: [u8; 16],
    address_register: u16,
    program_counter: u16,
    
    graphics: [u8; 64 * 32],

    delay_timer: u8,
    sound_timer: u8,

    stack: [u16; 16],
    stack_pointer: u16,

    key: [u8; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        let mut cpu = Cpu { opcode: 0, memory: [0; 4096], register: [0; 16], address_register: 0, 
                            program_counter: 0x200, graphics: [0; 64 * 32], delay_timer: 0, 
                            sound_timer: 0, stack: [0; 16], stack_pointer: 0, key: [0; 16] };

        // allocate the first portion of memory to the fontset
        for i in 0..80 {
            cpu.memory[i] = FONTSET[i];
        }

        cpu
    }

    pub fn cycle(&mut self) {
        
        self.fetch_opcode();

        // execute
        // update timers
    }

    fn fetch_opcode(&mut self) {
        self.opcode = ((self.memory[self.program_counter as usize] as u16) << 8) | 
            (self.memory[(self.program_counter + 1) as usize]) as u16
    }

    fn nop() {
        // mainly for testing purposes
    }

    fn clear_screen(&mut self) {
        for i in 0..(64 * 32) {
            self.graphics[i] = 0;
        }

        self.program_counter += 2;
    }

    fn cpu_return(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];

        self.program_counter += 2;
    }

    // 0x1NNN
    fn jump(&mut self) {
        self.program_counter = self.opcode & 0x0FFF;
    }

    // 0x2NNN
    fn call(&mut self) {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;

        self.program_counter = self.opcode & 0x0FFF;
    }

    // 3XNN: Skip next instruction if VX == NN
    fn skip_equal(&mut self) {
        let x = self.opcode & 0x0F00;
        let val = self.opcode & 0x00FF;

        if self.register[x as usize] == val as u8 {
            self.program_counter += 2;
        }

        self.program_counter += 2;
    }
    
    // 4XNN: Skip next instruction if VX != NN
    fn skip_not_equal(&mut self) {
        let x = self.opcode & 0x0F00;
        let val = self.opcode & 0x00FF;

        if self.register[x as usize] != val as u8 {
            self.program_counter += 2;
        }

        self.program_counter += 2;
    }

    // 5XY0: Skip next instruction if VX == VY
    fn skip_regs_equal(&mut self) {
        let x = self.opcode & 0x0F00;
        let y = self.opcode & 0x00F0;

        if self.register[x as usize] == self.register[y as usize] {
            self.program_counter += 2;
        }

        self.program_counter += 2;
    }

    // 6XNN: Set VX = NN
    fn set_vx_num(&mut self) {
        let x = self.opcode & 0x0F00;
        let val = self.opcode & 0x00FF;

        self.register[x as usize] = val as u8;

        self.program_counter += 2;
    }

    // 7XNN: Add NN to VX
    fn add_vx_num(&mut self) {
        let x = self.opcode & 0x0F00;
        let val = self.opcode & 0x00FF;

        self.register[x as usize] += val as u8;

        self.program_counter += 2;
    }
    
    // Opcode 0x8000 instructions
    fn opcode_8(&mut self) {
        let x = self.opcode & 0x0F00;
        let y = self.opcode & 0x00F0;

        // use lookup table to call the appropriate instruction passing x and y as usize
    
        self.program_counter += 2;
    }

    // 8XY0: Set VX to VY
    fn set_vx_vy(&mut self, x: usize, y: usize) {
        self.register[x] = self.register[y];
    }

    // 8XY1: Set VX = VX | VY
    fn set_vx_or(&mut self, x: usize, y: usize) {
        self.register[x] = self.register[x] | self.register[y];
    }

    // 8XY2: Set VX = VX & VY
    fn set_vx_and(&mut self, x: usize, y: usize) {
        self.register[x] = self.register[x] & self.register[y];
    }

    // 8XY3: Set VX = VX ^ VY
    fn set_vx_xor(&mut self, x: usize, y: usize) {
        self.register[x] = self.register[x] & self.register[y];
    }

    // 8XY4: VX += VY with carry flag
    fn add_vx_vy(&mut self, x: usize, y: usize) {
        self.register[x] += self.register[y];

        if(self.register[x] < self.register[y]) {
            self.register[0xF] = 1;
        } 
        else {
            self.register[0xF] = 0;
        }
    }

    // 8XY5: VX -= VY with borrow flag
    fn sub_vx_vy(&mut self, x: usize, y: usize) { 
        if(self.register[x] < self.register[y]) {
            self.register[0xF] = 0;
        }
        else {
            self.register[0xF] = 1;
        }
        
        self.register[x] -= self.register[y];
    }

    // 8XY6: Set VF to LSB of VX then shift VX right
    fn shr_vx(&mut self, x: usize, y: usize) {
        self.register[0xF] = self.register[x] & 1;
        self.register[x] = self.register[x] >> 1;
    }

    // 8XY7: Set VX = VY - VX and set borrow flag
    fn sub_vy_vx(&mut self, x: usize, y: usize) {
        if(self.register[y] < self.register[x]) {
            self.register[0xF] = 0;
        }
        else {
            self.register[0xF] = 1;
        }

        self.register[x] = self.register[y] - self.register[x];
    }

    // 8XYE: Set flag to MSB then shift VX left
    fn shl_vx(&mut self, x: usize, y: usize) {
        self.register[0xF] = self.register[x] & 0x80;
        self.register[x] = self.register[x] << 1;
    }

    // 9XY0: Skip the instruction if VX != VY
    fn skip_regs_not_equal(&mut self) {
        let x = self.opcode & 0x0F00;
        let y = self.opcode & 0x00F0;

        if(self.register[x as usize] != self.register[y as usize]) {
            self.program_counter += 2;
        }

        self.program_counter += 2;
    }

    // ANNN: Set address register
    fn set_adr_reg(&mut self) {
        self.address_register = self.opcode & 0x0FFF;

        self.program_counter += 2;
    }

    // BNNN: Jump to address NNN+V0
    fn jump_add(&mut self) {
        self.program_counter = (self.opcode & 0x0FFF) + self.register[0] as u16;
    }

    // CXNN
    
    // DXYN

    // 0xEX9E and EXA1 skip instruction of key in VX if it is pressed/not pressed depending on
    // opcode
    fn skip_key_press(&mut self) {
        // TODO: Implement key press data first
    }

    // Opcode 0xF000 instructions
    fn opcode_f(&mut self) {
        let x = self.opcode & 0x0F00;

        // Call function from lookup table
        
        self.program_counter += 2;
    }

    // FX07: VX = delay_timer
    fn set_vx_delay(&mut self, x: usize) {
        self.register[x] = self.delay_timer;
    }

    // FX0A: Store key press in VX
    fn set_vx_key(&mut self, x: usize) {
        // TODO: Key press
    }

    // FX15: Set delay_timer = VX
    fn set_delay_vx(&mut self, x: usize) {
        self.delay_timer = self.register[x];
    }

    // FX18: Set sound_timer = VX
    fn set_sound_vx(&mut self, x: usize) {
        self.sound_timer = self.register[x];
    }

    // FX1E: Add VX to address pointer
    fn add_adr_reg(&mut self, x: usize) {
        self.address_register += self.register[x] as u16;
    }

    // FX29
    
    // FX33
    
    // FX55
    
    // FX65
}

// Each character in the font set is 5 characters hide and 4 pixels wide
// so each entry represents one row in a character
static FONTSET: [u8; 80] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

//static OPTABLE: [fn(); 16] = [
//    nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop, nullop,
//    nullop, nullop,
//];
