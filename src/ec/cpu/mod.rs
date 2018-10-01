// TODO: make it private
pub mod instruction;

use ec::cpu::instruction::Address;
use ec::cpu::instruction::Register;
use ec::mem;
use ec::sv;
use ec::EcError;
use num_traits::FromPrimitive;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

const REGISTERS_OFFSET: usize = 1;
const ADDRESS_OFFSET: usize = 2;

type OpCode = u8;
type IndirectBit = u8;

// NOTE: remember to add this value to proper *Instr array below
#[derive(FromPrimitive)]
pub enum OpCodeValue {
    Lr = 0x00,
    Str = 0x02,
    Andr = 0x04,
    Orr = 0x05,
    Xorr = 0x06,
    Notr = 0x07,
    Ar = 0x10,
    Sr = 0x11,
    Mr = 0x13,
    Dr = 0x14,
    L = 0x20,
    Swap = 0x23,
    And = 0x24,
    Or = 0x25,
    Xor = 0x26,
    Not = 0x27,
    Svc = 0x2e,
    A = 0x30,
    S = 0x31,
    M = 0x33,
    D = 0x34,
    Li = 0x40,
    Min = 0x7a,
    Max = 0x7b,
}

pub enum OpType {
    Rr,
    Rs,
    Im,
}

static RrInstr: &'static [OpCode] = &[
    OpCodeValue::Lr as OpCode,
    OpCodeValue::Str as OpCode,
    OpCodeValue::Andr as OpCode,
    OpCodeValue::Orr as OpCode,
    OpCodeValue::Xorr as OpCode,
    OpCodeValue::Notr as OpCode,
    OpCodeValue::Ar as OpCode,
    OpCodeValue::Sr as OpCode,
    OpCodeValue::Mr as OpCode,
    OpCodeValue::Dr as OpCode,
];
static RsInstr: &'static [OpCode] = &[
    OpCodeValue::L as u8,
    OpCodeValue::Swap as u8,
    OpCodeValue::Svc as u8,
    OpCodeValue::And as u8,
    OpCodeValue::Or as u8,
    OpCodeValue::Xor as u8,
    OpCodeValue::Not as u8,
    OpCodeValue::A as u8,
    OpCodeValue::S as u8,
    OpCodeValue::M as u8,
    OpCodeValue::D as u8,
    OpCodeValue::Min as u8,
    OpCodeValue::Max as u8,
];
static ImInstr: &'static [OpCode] = &[OpCodeValue::Li as OpCode];

pub struct Cpu {
    ilc: usize,
    mem: Rc<RefCell<mem::Memory>>,
}

impl Cpu {
    pub fn new(mem: Rc<RefCell<mem::Memory>>) -> Cpu {
        Cpu { ilc: 0x40, mem }
    }
    fn op_type(op_code: OpCode) -> Option<OpType> {
        if RrInstr.contains(&op_code) {
            return Some(OpType::Rr);
        } else if RsInstr.contains(&op_code) {
            return Some(OpType::Rs);
        } else if ImInstr.contains(&op_code) {
            return Some(OpType::Im);
        }
        None
    }
    fn read_opcode(&self, address: usize) -> (IndirectBit, OpCode) {
        let byte = self.mem.borrow().get(address);
        (((byte & 0x40) != 0) as IndirectBit, byte & 0x7f)
    }
    fn read_op_registers(&self, address: usize) -> (Register, Register) {
        let registers_byte = self.mem.borrow().get(address);
        ((registers_byte & 0xf0) >> 4, registers_byte & 0x0f)
    }
    // TODO: extend it to the different algorithms of calculating
    // effective address. For now it corresponds to the situation, when
    // indirect bit is 0 and index register designator (bits 12-15) is 0
    fn read_op_address(&self, address: usize) -> Address {
        ((self.mem.borrow().get(address + ADDRESS_OFFSET) as Address) << 8)
            + (self.mem.borrow().get(address + ADDRESS_OFFSET + 1) as Address)
    }
    fn read_r1_and_value(&self, address: usize) -> (Register, i32) {
        // TODO: add proper asserts
        let b1 = self.mem.borrow().get(address + 1);
        let b2 = self.mem.borrow().get(address + 2);
        let b3 = self.mem.borrow().get(address + 3);

        // TODO: take care of sign extension:
        // https://en.wikipedia.org/wiki/Sign_extension
        (
            (b1 & 0xf0) >> 4,
            ((((b1 & 0xf) as i32) << 16) | ((b2 as i32) << 8) | b3 as i32),
        )
    }
    fn read_instruction(&self) -> instruction::Instruction {
        trace!("Reading next instruction at {}", self.ilc);
        let (_indirect_bit, op_code) = self.read_opcode(self.ilc);
        match Cpu::op_type(op_code) {
            Some(OpType::Rr) => {
                let (r1, r2) = self.read_op_registers(self.ilc + REGISTERS_OFFSET);
                trace!("RR instruction {} {}", r1, r2);
                match FromPrimitive::from_u8(op_code) {
                    // TODO: remove this special case and handle program end properly
                    // 1) treat "program" ALF and "memory" ALF differently and mark program
                    //    bytes in memory (so execution will continue as long as next instruction
                    //    is within marked space)
                    // 2) add some special combination of bytes to mark end of program (vulnerable
                    //    to, e.g. jumps)
                    // 3) add a separate memory for a program's ALF with the same size a program
                    //    (allocated after loading ALF)
                    // 4) save address of the last byte of the read program (which should be last?
                    //    with the highest address or last read (is it always the same?)) and
                    //    exit on reaching it
                    Some(OpCodeValue::Lr) => {
                        if r1 == 0 && r2 == 0 {
                            return instruction::Instruction::None;
                        }
                        return instruction::Instruction::LoadRegister(r1, r2);
                    }
                    Some(OpCodeValue::Str) => {
                        return instruction::Instruction::StoreRegister(r1, r2);
                    }
                    Some(OpCodeValue::Andr) => {
                        return instruction::Instruction::AndRegister(r1, r2);
                    }
                    Some(OpCodeValue::Orr) => {
                        return instruction::Instruction::OrRegister(r1, r2);
                    }
                    Some(OpCodeValue::Xorr) => {
                        return instruction::Instruction::XorRegister(r1, r2);
                    }
                    Some(OpCodeValue::Notr) => {
                        return instruction::Instruction::NotRegister(r1, r2);
                    }
                    Some(OpCodeValue::Ar) => {
                        return instruction::Instruction::AddRegister(r1, r2);
                    }
                    Some(OpCodeValue::Sr) => {
                        return instruction::Instruction::SubtractRegister(r1, r2);
                    }
                    Some(OpCodeValue::Mr) => {
                        return instruction::Instruction::MultiplyRegister(r1, r2);
                    }
                    Some(OpCodeValue::Dr) => {
                        return instruction::Instruction::DivideRegister(r1, r2);
                    }
                    Some(_) => (),
                    None => (),
                }
            }
            Some(OpType::Rs) => {
                let (r1, r2) = self.read_op_registers(self.ilc + REGISTERS_OFFSET);
                trace!("RS instruction, r1 = {} r2 = {}", r1, r2);
                let address = self.read_op_address(self.ilc);
                // TODO: write a macro to handle these patterns
                // e.g. Match(OpCodeValue::L, instruction::Instruction::Load)
                // that will pass proper arguments to them
                match FromPrimitive::from_u8(op_code) {
                    Some(OpCodeValue::L) => {
                        return instruction::Instruction::Load(r1, r2, address);
                    }
                    Some(OpCodeValue::Swap) => {
                        return instruction::Instruction::Swap(r1, r2, address);
                    }
                    Some(OpCodeValue::Svc) => {
                        return instruction::Instruction::SupervisorCall(r1, r2, address);
                    }
                    Some(OpCodeValue::And) => {
                        return instruction::Instruction::And(r1, r2, address);
                    }
                    Some(OpCodeValue::Or) => {
                        return instruction::Instruction::Or(r1, r2, address);
                    }
                    Some(OpCodeValue::Xor) => {
                        return instruction::Instruction::Xor(r1, r2, address);
                    }
                    Some(OpCodeValue::Not) => {
                        return instruction::Instruction::Not(r1, r2, address);
                    }
                    Some(OpCodeValue::A) => {
                        return instruction::Instruction::Add(r1, r2, address);
                    }
                    Some(OpCodeValue::S) => {
                        return instruction::Instruction::Subtract(r1, r2, address);
                    }
                    Some(OpCodeValue::M) => {
                        return instruction::Instruction::Multiply(r1, r2, address);
                    }
                    Some(OpCodeValue::D) => {
                        return instruction::Instruction::Divide(r1, r2, address);
                    }
                    Some(OpCodeValue::Min) => {
                        return instruction::Instruction::Min(r1, r2, address);
                    }
                    Some(OpCodeValue::Max) => {
                        return instruction::Instruction::Max(r1, r2, address);
                    }
                    Some(_) => (),
                    None => (),
                }
            }
            Some(OpType::Im) => {
                let (r1, value) = self.read_r1_and_value(self.ilc);
                trace!("IM instruction {} {}", r1, value);
                match FromPrimitive::from_u8(op_code) {
                    Some(OpCodeValue::Li) => {
                        return instruction::Instruction::LoadImmediate(r1, value);
                    }
                    Some(_) => (),
                    None => (),
                }
            }
            _ => (),
        }
        instruction::Instruction::None
    }
    fn assert_ilc_valid(&self) -> Result<(), EcError> {
        if self.ilc % 2 != 0 {
            return Err(EcError::IllegalInstructionAddress);
        }
        Ok(())
    }
    /// Single fetch-decode-execute cycle
    pub fn poll(&mut self, trace: bool) -> Result<sv::Action, EcError> {
        self.assert_ilc_valid()?;

        let next_instr = Cpu::read_instruction(&self);
        let (_indirect_bit, op_code) = self.read_opcode(self.ilc);
        // TODO: it's a tricky hack, that depends on assumption that
        // ilc is only changed in this place and not in any other method
        // ilc should be encapsulated and put in some structure taking care
        // of the full information about next instruction
        match Cpu::op_type(op_code) {
            Some(OpType::Rr) => {
                self.ilc += 0x2;
            }
            _ => {
                self.ilc += 0x4;
            }
        }
        if trace {
            trace!("{:?}", next_instr);
        }
        // TODO: Match instruction format (RR/RS/IM/CH) instead of type,
        // cause handling one instruction format is in most cases the same.
        // Next, instruction types can me map to functions, e.g.
        // A => |a, b| a + b;
        // S => |a, b| a - b;
        match next_instr {
            // TODO: add indirect bit support
            instruction::Instruction::LoadRegister(r1, r2) => {
                trace!(
                    "running LoadRegister instruction with r1 = {} r2 = {}",
                    r1,
                    r2
                );
                let value = self.mem.borrow().read_reg(r2 as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, value);
                return Ok(sv::Action::None);
            }
            // TODO: add indirect bit support
            instruction::Instruction::StoreRegister(r1, r2) => {
                trace!(
                    "running StoreRegister instruction with r1 = {} r2 = {}",
                    r1,
                    r2
                );
                let value = self.mem.borrow().read_reg(r1 as usize);
                self.mem.borrow_mut().write_reg(r2 as usize, value);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::AndRegister(r1, r2) => {
                // TODO: compare sum to 0 and set CCR
                let result = self.mem.borrow().read_reg(r1 as usize)
                    & self.mem.borrow().read_reg(r2 as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::OrRegister(r1, r2) => {
                // TODO: compare sum to 0 and set CCR
                let result = self.mem.borrow().read_reg(r1 as usize)
                    | self.mem.borrow().read_reg(r2 as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::XorRegister(r1, r2) => {
                // TODO: compare sum to 0 and set CCR
                let result = self.mem.borrow().read_reg(r1 as usize)
                    ^ self.mem.borrow().read_reg(r2 as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::NotRegister(r1, r2) => {
                // TODO: compare sum to 0 and set CCR
                let result = !self.mem.borrow().read_reg(r2 as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::AddRegister(r1, r2) => {
                // TODO: compare sum to 0 and set CCR
                let result = self.mem.borrow().read_reg(r1 as usize)
                    + self.mem.borrow().read_reg(r2 as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::SubtractRegister(r1, r2) => {
                // TODO: compare sum to 0 and set CCR
                let result = self.mem.borrow().read_reg(r1 as usize)
                    - self.mem.borrow().read_reg(r2 as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::MultiplyRegister(r1, r2) => {
                // TODO: compare sum to 0 and set CCR
                let result = self.mem.borrow().read_reg(r1 as usize)
                    * self.mem.borrow().read_reg(r2 as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::DivideRegister(r1, r2) => {
                // TODO: compare sum to 0 and set CCR
                let result = self.mem.borrow().read_reg(r1 as usize)
                    / self.mem.borrow().read_reg(r2 as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::Load(r1, _r2, addr) => {
                let value = self.mem.borrow().read_word(addr as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, value);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::Swap(r1, _r2, addr) => {
                // TODO: set proper CCR bits
                let r1_value = self.mem.borrow().read_reg(r1 as usize);
                let swap_value = self.mem.borrow().read_word(addr as usize);
                debug!(
                    "running swap instruction with {} ({} reg) and {} ({} addr)",
                    r1_value, r1, swap_value, addr
                );
                self.mem.borrow_mut().write_reg(r1 as usize, swap_value);
                self.mem.borrow_mut().write_word(addr as usize, r1_value);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::SupervisorCall(r1, _r2, addr) => {
                // TODO: consider additional interface for register
                // value retrieval
                let action_id = self.mem.borrow().read_reg(r1 as usize);
                trace!("Supervisor call with id {}", action_id);
                match action_id {
                    0 => return Ok(sv::Action::Exit),
                    1 => return Ok(sv::Action::ReadInt(addr)),
                    3 => return Ok(sv::Action::ReadChar(addr)),
                    5 => return Ok(sv::Action::WriteInt(addr)),
                    7 => return Ok(sv::Action::WriteChar(addr)),
                    _ => return Ok(sv::Action::Exit),
                }
            }
            instruction::Instruction::And(r1, _r2, addr) => {
                // TODO: set proper CCR bits
                let result = self.mem.borrow().read_reg(r1 as usize)
                    & self.mem.borrow().read_word(addr as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::Or(r1, _r2, addr) => {
                // TODO: set proper CCR bits
                let result = self.mem.borrow().read_reg(r1 as usize)
                    | self.mem.borrow().read_word(addr as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::Xor(r1, _r2, addr) => {
                // TODO: set proper CCR bits
                let result = self.mem.borrow().read_reg(r1 as usize)
                    ^ self.mem.borrow().read_word(addr as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::Not(r1, _r2, addr) => {
                // TODO: set proper CCR bits
                let result = !self.mem.borrow().read_word(addr as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::Add(r1, _r2, addr) => {
                // TODO: compare sum to 0 and set CCR
                let result = self.mem.borrow().read_reg(r1 as usize)
                    + self.mem.borrow().read_word(addr as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::Subtract(r1, _r2, addr) => {
                // TODO: compare sum to 0 and set CCR
                let result = self.mem.borrow().read_reg(r1 as usize)
                    - self.mem.borrow().read_word(addr as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::Multiply(r1, _r2, addr) => {
                // TODO: compare sum to 0 and set CCR
                let result = self.mem.borrow().read_reg(r1 as usize)
                    * self.mem.borrow().read_word(addr as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::Divide(r1, _r2, addr) => {
                // TODO: compare sum to 0 and set CCR
                let result = self.mem.borrow().read_reg(r1 as usize)
                    / self.mem.borrow().read_word(addr as usize);
                self.mem.borrow_mut().write_reg(r1 as usize, result);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::LoadImmediate(r1, value) => {
                self.mem.borrow_mut().write_reg(r1 as usize, value as i32);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::Min(r1, _r2, addr) => {
                let r1_value = self.mem.borrow().read_reg(r1 as usize);
                let addr_value = self.mem.borrow().read_word(addr as usize);
                self.mem
                    .borrow_mut()
                    .write_reg(r1 as usize, cmp::min(r1_value, addr_value) as i32);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::Max(r1, _r2, addr) => {
                let r1_value = self.mem.borrow().read_reg(r1 as usize);
                let addr_value = self.mem.borrow().read_word(addr as usize);
                self.mem
                    .borrow_mut()
                    .write_reg(r1 as usize, cmp::max(r1_value, addr_value) as i32);
                return Ok(sv::Action::None);
            }
            instruction::Instruction::None => (),
        }
        Ok(sv::Action::Exit)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reads_correctly_opcode() {
        let mem = Rc::new(RefCell::new(mem::Memory::new()));
        let cpu = Cpu::new(Rc::clone(&mem));
        mem.borrow_mut().set(0x10, 0xff);
        assert_eq!((0x01, 0x7f), cpu.read_opcode(0x10));

        mem.borrow_mut().set(0x20, 0x1b);
        assert_eq!((0x00, 0x1b), cpu.read_opcode(0x20));
    }

    #[test]
    fn test_reads_registers_correctly() {
        let mem = Rc::new(RefCell::new(mem::Memory::new()));
        let cpu = Cpu::new(Rc::clone(&mem));
        mem.borrow_mut().set(0x10, 0x1c);
        assert_eq!((0x01, 0x0c), cpu.read_op_registers(0x10));

        mem.borrow_mut().set(0x20, 0xe5);
        assert_eq!((0x0e, 0x05), cpu.read_op_registers(0x20));
    }
}
