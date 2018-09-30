pub type Register = u8;
pub type Address = u16;

// TODO: consider splitting them into different types
// (RR/IM/RSCH)
/// CPU Instruction
/// TODO: add support for indirect bit
#[derive(Debug)]
pub enum Instruction {
    None,
    LoadRegister(Register, Register),
    StoreRegister(Register, Register),
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
    Min(Register, Register, Address),
    Max(Register, Register, Address),
}
