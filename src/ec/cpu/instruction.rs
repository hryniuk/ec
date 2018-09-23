pub type Register = u8;
pub type Address = u16;

// TODO: consider splitting them into different types
// (RR/IM/RSCH)
/// CPU Instruction
#[derive(Debug)]
pub enum Instruction {
    None,
    /// The first register is loaded with the word at the effective address
    Load(Register, Register, Address),
    SupervisorCall(Register, Register, Address),
    Add(Register, Register, Address),
    Subtract(Register, Register, Address),
    Multiply(Register, Register, Address),
    Divide(Register, Register, Address),
    LoadImmediate(Register, i32),
}
