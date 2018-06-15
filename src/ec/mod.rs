// TODO: refactor to make them private (
pub mod alf;
pub mod mem;

/// Type representing error arising during
/// course of instruction execution
pub enum EcError {
    /// At the start of an instruction execution cycle
    /// the ILC does not contain an even value.
    IllegalInstructionAddress,
    /// There is no operation defined for this operation code.
    UnimplementedInstruction,
    /// The indirect address is not even.
    InvalidIndirectAddress,
    /// The address of a purported word operand to an
    /// instruction is not divisible by four.
    WordAddressingError,
    /// The result of some real-valued operation cannot be
    /// expressed within the format for normalized real numbers.
    UnrepresentableRealValue,
    /// The effective address of an Execute instruction is not even.
    InvalidExecutionAddress,
    /// The divisor in a division or remainder operation is zero.
    ZeroDivisor,
    /// A four-character instruction begins at FFFE.
    WraparoundInstruction,
}

pub struct Ec {
    ilc: u32,
    ccr: u32,
    pub mem: mem::Memory,
}

impl Ec {
    pub fn run() {
        unimplemented!();
    }
}
