use ec::cpu::opcode;

pub type Register = u8;
pub type Address = u16;

// TODO: consider splitting them into different types
// (RR/IM/RSCH)
/// CPU Instruction
/// TODO: add support for indirect bit
#[derive(Debug)]
pub enum Instruction {
    None,
    RegisterRegister(opcode::OpCodeValue, Register, Register),
    RegisterStorage(opcode::OpCodeValue, Register, Register, Address),
    Immediate(opcode::OpCodeValue, Register, i32),
    Character(opcode::OpCodeValue, Register, Register, Address),
}
