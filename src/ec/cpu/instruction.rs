pub type Register = u8;
pub type Address = u16;

#[derive(Debug)]
pub enum Instruction {
    None,
    SupervisorCall(Register, Register, Address),
    LoadImmediate(Register, i32),
}
