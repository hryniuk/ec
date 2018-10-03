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
    LoadRegister(Register, Register),
    StoreRegister(Register, Register),
    AndRegister(Register, Register),
    OrRegister(Register, Register),
    XorRegister(Register, Register),
    NotRegister(Register, Register),
    AddRegister(Register, Register),
    SubtractRegister(Register, Register),
    MultiplyRegister(Register, Register),
    DivideRegister(Register, Register),
    /// The first register is loaded with the word at the effective address
    Load(Register, Register, Address),
    Swap(Register, Register, Address),
    SupervisorCall(Register, Register, Address),
    And(Register, Register, Address),
    Or(Register, Register, Address),
    Xor(Register, Register, Address),
    Not(Register, Register, Address),
    Add(Register, Register, Address),
    Subtract(Register, Register, Address),
    Multiply(Register, Register, Address),
    Divide(Register, Register, Address),
    LoadImmediate(Register, i32),
    AndImmediate(Register, i32),
    OrImmediate(Register, i32),
    XorImmediate(Register, i32),
    NotImmediate(Register, i32),
    AddImmediate(Register, i32),
    SubtractImmediate(Register, i32),
    MultiplyImmediate(Register, i32),
    DivideImmediate(Register, i32),
    Min(Register, Register, Address),
    Max(Register, Register, Address),
}
