use crate::vm::Vm;

// TODO: do these go in vm?
pub enum ConditionFlags {
    Pos     = 1 << 0,
    Zero    = 1 << 1,
    Match   = 1 << 2,
}

#[derive(Debug)]
pub enum Operation {
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

impl Operation {
    pub fn from_instruction(instruction: u16) -> Operation {
        // we know we have covered all possible values of the remaining four bits here so
        // we choose to panic if somthing has gone wrong as this truly us unrecoverable
        match instruction >> 12 { // Maybe we could use a u4 crate here to avoid the '_'?
            0  => Operation::BR,
            1  => Operation::ADD,
            2  => Operation::LD,
            3  => Operation::ST,
            4  => Operation::JSR,
            5  => Operation::AND,
            6  => Operation::LDR,
            7  => Operation::STR,
            8  => Operation::RTI,
            9  => Operation::NOT,
            10 => Operation::LDI,
            11 => Operation::STI,
            12 => Operation::JMP,
            13 => Operation::RES,
            14 => Operation::LEA,
            15 => Operation::TRAP,
            _ => panic!("unable to extract opcode from {}", instruction)
        }

    }
    pub fn exec(&self, vm: &mut Vm) {
        println!("exec {:?}", self);
        match self {
            Operation::BR    => exec_br(vm),
            Operation::ADD   => exec_add(vm),
            Operation::LD    => exec_ld(vm),
            Operation::ST    => exec_st(vm),
            Operation::JSR   => exec_jsr(vm),
            Operation::AND   => exec_and(vm),
            Operation::LDR   => exec_ldr(vm),
            Operation::STR   => exec_str(vm),
            Operation::RTI   => exec_rti(vm),
            Operation::NOT   => exec_not(vm),
            Operation::LDI   => exec_ldi(vm),
            Operation::STI   => exec_sti(vm),
            Operation::JMP   => exec_jmp(vm),
            Operation::RES   => exec_res(vm),
            Operation::LEA   => exec_lea(vm),
            Operation::TRAP  => exec_trap(vm),
        }
    }
}

fn sign_extend(x: u16, bit_count: u8) -> u16 {
    let out: u16;
    // this & 1 part us necessary for bitcount to override any garbage 1's above bitcount
    // eg
    // 0b1111_0000_0001_0000 with a bitcount of 5 will fail the following 'if' clause
    if (x >> (bit_count - 1)) & 1 == 1 {
        out = x | (0xFFFF << bit_count) as u16;
    } else {
        out = x
    }
    out
}

fn exec_br(vm: &mut Vm) {
}

fn exec_add(vm: &mut Vm) {
}

fn exec_ld(vm: &mut Vm) {
}

fn exec_st(vm: &mut Vm) {
}

fn exec_jsr(vm: &mut Vm) {
}

fn exec_and(vm: &mut Vm) {
}

fn exec_ldr(vm: &mut Vm) {
}

fn exec_str(vm: &mut Vm) {
}

fn exec_rti(vm: &mut Vm) {
}

fn exec_not(vm: &mut Vm) {
}

fn exec_ldi(vm: &mut Vm) {
}

fn exec_sti(vm: &mut Vm) {
}

fn exec_jmp(vm: &mut Vm) {
}

fn exec_res(vm: &mut Vm) {
}

fn exec_lea(vm: &mut Vm) {
}

fn exec_trap(vm: &mut Vm) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_extend() {
        // works on the full width
        assert_eq!(sign_extend(0b0000_0000_0000_0000, 16), 0b0000_0000_0000_0000);
        assert_eq!(sign_extend(0b0000_0000_0000_0001, 16), 0b0000_0000_0000_0001);
        assert_eq!(sign_extend(0b1000_0000_0000_0000, 16), 0b1000_0000_0000_0000);

        // works on arbatrary widths
        assert_eq!(sign_extend(0b0000_0000_0000_0000, 5), 0b0000_0000_0000_0000);
        assert_eq!(sign_extend(0b0000_0000_0000_0001, 5), 0b0000_0000_0000_0001);
        assert_eq!(sign_extend(0b0000_0000_0001_0010, 5), 0b1111_1111_1111_0010);

        // check that we can ignore 1s beyond the bitcount (second param)
        assert_eq!(sign_extend(0b1000_0000_0001_0010, 5), 0b1111_1111_1111_0010);
    }

    #[test]
    fn test_exec_br() {
    }

#[test]
    fn test_exec_add() {
    }

#[test]
    fn test_exec_ld() {
    }

#[test]
    fn test_exec_st() {
    }

#[test]
    fn test_exec_jsr() {
    }

#[test]
    fn test_exec_and() {
    }

#[test]
    fn test_exec_ldr() {
    }

#[test]
    fn test_exec_str() {
    }

#[test]
    fn test_exec_rti() {
    }

#[test]
    fn test_exec_not() {
    }

#[test]
    fn test_exec_ldi() {
    }

#[test]
    fn test_exec_sti() {
    }

#[test]
    fn test_exec_jmp() {
    }

#[test]
    fn test_exec_res() {
    }

#[test]
    fn test_exec_lea() {
    }

#[test]
    fn test_exec_trap() {
    }
}

