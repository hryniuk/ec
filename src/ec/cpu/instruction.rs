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
    /// The first register is loaded with the word at the effective address
    LoadImmediate(Register, i32),
    AndImmediate(Register, i32),
    OrImmediate(Register, i32),
    XorImmediate(Register, i32),
    NotImmediate(Register, i32),
    AddImmediate(Register, i32),
    SubtractImmediate(Register, i32),
    MultiplyImmediate(Register, i32),
    DivideImmediate(Register, i32),
}
