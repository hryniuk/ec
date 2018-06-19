type Register = u8;
type Address = usize;

#[derive(Debug)]
pub enum Instruction {
    SupervisorCall(Register, Register, Address),
}
