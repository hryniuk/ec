type Register = u8;
type Address = usize;

pub enum Instruction {
    SupervisorCall(Register, Register, Address),
}
