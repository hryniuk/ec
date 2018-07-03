// TODO: make it private
pub mod instruction;

use ec::cpu::instruction::Address;
use ec::cpu::instruction::Register;
use ec::mem;
use ec::sv;
use ec::EcError;
use std::cell::RefCell;
use std::rc::Rc;

const REGISTERS_OFFSET: usize = 1;
const ADDRESS_OFFSET: usize = 2;

type OpCode = u8;
type IndirectBit = u8;

pub enum OpCodeValue {
    Svc = 0x2e,
}

pub enum OpType {
    Rs,
}

pub struct Cpu {
    ilc: usize,
    mem: Rc<RefCell<mem::Memory>>,
}

impl Cpu {
    pub fn new(mem: Rc<RefCell<mem::Memory>>) -> Cpu {
        Cpu { ilc: 0x10, mem }
    }
    fn op_type(op_code: OpCode) -> OpType {
        OpType::Rs
    }
    fn read_opcode(&self, address: usize) -> (IndirectBit, OpCode) {
        let byte = self.mem.borrow().get(address);
        (byte & 0x40, byte & 0x7f)
    }
    fn read_op_registers(&self, address: usize) -> (Register, Register) {
        let registers_byte = self.mem.borrow().get(address + REGISTERS_OFFSET);
        ((registers_byte & 0xf0) >> 4, registers_byte & 0x0f)
    }
    fn read_op_address(&self, address: usize) -> Address {
        self.mem.borrow().read_word(address + ADDRESS_OFFSET)
    }
    fn read_instruction(&self) -> instruction::Instruction {
        let (indirect_bit, op_code) = self.read_opcode(self.ilc);
        match Cpu::op_type(op_code) {
            Rs => {
                let (r1, r2) = self.read_op_registers(self.ilc);
                let address = self.read_op_address(self.ilc);
                match op_code {
                    Svc => {
                        return instruction::Instruction::SupervisorCall(r1, r2, address);
                    }
                }
            }
            _ => (),
        }
        instruction::Instruction::None
    }
    /// Single fetch-decode-execute cycle
    pub fn poll(&mut self, trace: bool) -> Result<sv::Action, EcError> {
        if self.ilc % 2 != 0 {
            return Err(EcError::IllegalInstructionAddress);
        }

        let next_instr = Cpu::read_instruction(&self);
        if trace {
            trace!("{:?}", next_instr);
        }
        match next_instr {
            instruction::Instruction::SupervisorCall(r1, r2, addr) => {
                // TODO: consider additional interface for register
                // value retrieval
                let action_id = self.mem.borrow().get(r1 as usize);
                trace!("Supervisor call with id {}", action_id);
                match action_id {
                    0 => return Ok(sv::Action::Exit),
                    5 => return Ok(sv::Action::WriteInt(addr)),
                    _ => return Ok(sv::Action::Exit),
                }
            }
            instruction::Instruction::None => (),
        }
        Ok(sv::Action::None)
    }
}
