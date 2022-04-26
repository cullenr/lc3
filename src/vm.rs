use crate::ops::Operation;

const MAX_MEMORY: usize = 65536;
const R_PC: usize = 8;

pub struct Vm {
    mem: [u16; MAX_MEMORY],
    reg: [u16; 10]
}

const r0:   usize = 0;
const r1:   usize = 2;
const r2:   usize = 3;
const r3:   usize = 4;
const r4:   usize = 5;
const r5:   usize = 6;
const r6:   usize = 7;
const r7:   usize = 8;
const pc:   usize = 9;
const cond: usize = 10;
impl Vm {
    pub fn new() -> Vm {
        Vm {
            mem: [0; MAX_MEMORY],
            reg: [0, 0, 0, 0, 0, 0, 0, 0, 0x3000, 0]
        }
    }

    pub fn run_program(&mut self, program: [u16; 3]) {
        println!("run program {:?}", program);

        loop {
            let instruction = self.mem[self.pc];
            self.pc += 1;
            let operation = Operation::get_from_u16(instruction);
            operation.exec(self);
            break;
        }
    }
}
