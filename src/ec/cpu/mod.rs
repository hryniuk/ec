mod instruction;

use ec::mem;
use ec::sv;
use ec::EcError;
use std::cell::RefCell;
use std::rc::Rc;

type OpCode = u8;
type IndirectBit = u8;

pub struct Cpu {
    ilc: u32,
    mem: Rc<RefCell<mem::Memory>>,
}

impl Cpu {
    pub fn new(mem: Rc<RefCell<mem::Memory>>) -> Cpu {
        Cpu { ilc: 0xa, mem }
    }
    fn read_opcode(&self, address: usize) -> (IndirectBit, OpCode) {
        let byte = self.mem.borrow().get(address);
        (byte & 0x40, byte & 0x7f)
    }
    fn get_instruction() -> instruction::Instruction {
        instruction::Instruction::SupervisorCall(0, 0, 0)
    }
    pub fn poll(&self, trace: bool) -> Result<sv::Action, EcError> {
        let next_instr = Cpu::get_instruction();
        if trace {
            trace!("{:?}", next_instr);
        }
        match next_instr {
            instruction::Instruction::SupervisorCall(r1, r2, addr) => {
                // TODO: consider additional interface for register
                // value retrieval
                let action_id = self.mem.borrow().get(r1 as usize);
                match action_id {
                    0 => return Ok(sv::Action::Exit),
                    _ => return Ok(sv::Action::Exit),
                }
            }
        }
        Ok(sv::Action::None)
    }
}
