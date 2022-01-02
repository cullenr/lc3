const MAX_MEMORY: usize = 65536;

pub struct Vm {
    mem: [u16; MAX_MEMORY],
    reg: [u16; 10]
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            mem: [0; MAX_MEMORY],
            reg: [0; 10]
        }
    }

    pub fn run_program(&self, program: [u16; 3]) {
        println!("run program");

        loop {

        }
    }

}

pub enum Registers {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND
}

#[derive(Debug)]
pub enum Operations {
    BR = 0, // branch
    ADD,    // add
    LD,     // load
    ST,     // store
    JSR,    // jump register
    AND,    // bitwise and
    LDR,    // load register
    STR,    // store register
    RTI,    // unused
    NOT,    // bitwise not
    LDI,    // load indirect
    STI,    // store indirect
    JMP,    // jump
    RES,    // reserved (unused)
    LEA,    // load effective address
    TRAP    // execute trap
}

impl Operations {
    pub fn exec(&self) {
        print!("asd {:?}", self);
    }
}

pub enum ConditionFlags {
    Pos     = 1 << 0,
    Zero    = 1 << 1,
    Match   = 1 << 2,
}
