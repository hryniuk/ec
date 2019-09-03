// TODO: make it private
pub mod fconv;
pub mod instruction;
pub mod opcode;

use self::instruction::Address;
use self::instruction::Register;
use crate::ec::mem;
use crate::ec::sv;
use crate::ec::EcError;
use num_traits::FromPrimitive;
use std::cell::RefCell;
use std::cmp;
use std::i32;
use std::rc::Rc;

const REGISTERS_OFFSET: usize = 1;
const ADDRESS_OFFSET: usize = 2;

type IndirectBit = u8;

#[derive(Debug)]
enum AluOpType {
    And,
    Or,
    Xor,
    Not,
    Add,
    Sub,
    RSub,
    Mul,
    Div,
    RDiv,
    Rem,
    RRem,
}

enum Ccr {
    Empty,
    Overflow,
    Greater,
    Lower,
    Equal,
}

pub struct Cpu {
    ccr: Ccr,
    ilc: usize,
    mem: Rc<RefCell<mem::Memory>>,
}

impl Cpu {
    pub fn new(mem: Rc<RefCell<mem::Memory>>) -> Cpu {
        Cpu {
            ccr: Ccr::Empty,
            ilc: 0x40,
            mem,
        }
    }
    fn set_ccr(&mut self, value: i32) {
        if value > 0 {
            self.ccr = Ccr::Greater;
        } else if value < 0 {
            self.ccr = Ccr::Lower;
        } else if value == 0 {
            self.ccr = Ccr::Equal;
        }
    }
    fn mask_ccr(&self, mask: u8) -> bool {
        match &self.ccr {
            Ccr::Overflow => (mask & (1 << 3)) != 0,
            Ccr::Greater => (mask & (1 << 2)) != 0,
            Ccr::Lower => (mask & (1 << 1)) != 0,
            Ccr::Equal => (mask & 1) != 0,
            Ccr::Empty => false,
        }
    }
    fn op_type(op_code: opcode::OpCode) -> Option<opcode::OpType> {
        if opcode::RR_INSTR.contains(&op_code) {
            return Some(opcode::OpType::Rr);
        } else if opcode::RS_INSTR.contains(&op_code) {
            return Some(opcode::OpType::Rs);
        } else if opcode::RRM_INSTR.contains(&op_code) {
            return Some(opcode::OpType::Rrm);
        } else if opcode::IM_INSTR.contains(&op_code) {
            return Some(opcode::OpType::Im);
        } else if opcode::CH_INSTR.contains(&op_code) {
            return Some(opcode::OpType::Ch);
        }
        None
    }
    fn read_opcode(&self, address: usize) -> (IndirectBit, opcode::OpCode) {
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
            (i32::from(b1 & 0xf) << 16) | (i32::from(b2) << 8) | i32::from(b3),
        )
    }

    fn alu_op(&mut self, op1: i32, op2: i32, alu_op_type: &AluOpType) -> i32 {
        trace!(
            "alu_op call with op1 = {} op2 = {} alu_op_type = {:?}",
            op1,
            op2,
            alu_op_type
        );
        match alu_op_type {
            AluOpType::And => op1 & op2,
            AluOpType::Or => op1 | op2,
            AluOpType::Xor => op1 ^ op2,
            AluOpType::Not => !op2,
            AluOpType::Add => op1 + op2,
            AluOpType::Sub => {
                let result = op1 - op2;
                self.set_ccr(result);
                result
            }
            AluOpType::RSub => op2 - op1,
            AluOpType::Mul => op1 * op2,
            AluOpType::Div => op1 / op2,
            AluOpType::RDiv => op2 / op1,
            AluOpType::Rem => op1 % op2,
            AluOpType::RRem => op2 % op1,
        }
    }

    fn alu1(&mut self, addr1: usize, op2: i32, alu_op_type: &AluOpType) {
        let op1 = self.mem.borrow().read_word(addr1 as usize);
        let result = self.alu_op(op1, op2, alu_op_type);
        self.mem.borrow_mut().write_word(addr1, result);
    }

    fn alu2(&mut self, addr1: usize, addr2: usize, alu_op_type: &AluOpType) {
        let op1 = self.mem.borrow().read_word(addr1 as usize);
        let op2 = self.mem.borrow().read_word(addr2 as usize);
        let result = self.alu_op(op1, op2, alu_op_type);
        self.mem.borrow_mut().write_word(addr1, result);
    }

    fn read_instruction(&self) -> instruction::Instruction {
        trace!("Reading next instruction at {}", self.ilc);
        let (_indirect_bit, op_code) = self.read_opcode(self.ilc);
        match Cpu::op_type(op_code) {
            Some(opcode::OpType::Rr) => {
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
                    // NOTE: as 0x0d opcode is not used, it could be used a special value
                    // denoting the end of a program (?)
                    Some(opcode::OpCodeValue::Lr) => {
                        if r1 == 0 && r2 == 0 {
                            return instruction::Instruction::None;
                        }
                        return instruction::Instruction::RegisterRegister(
                            opcode::OpCodeValue::Lr,
                            r1,
                            r2,
                        );
                    }
                    Some(o) => {
                        return instruction::Instruction::RegisterRegister(o, r1, r2);
                    }
                    None => {
                        error!(
                            "Conversion error: op code with value = {} doesn't exist.",
                            op_code
                        );
                    }
                }
            }
            Some(opcode::OpType::Rrm) => {
                let (m1, r2) = self.read_op_registers(self.ilc + REGISTERS_OFFSET);
                trace!("RRm instruction m1 = {} r2 = {}", m1, r2);
                match FromPrimitive::from_u8(op_code) {
                    Some(o) => {
                        return instruction::Instruction::RegisterRegisterMask(o, m1, r2);
                    }
                    None => {
                        error!(
                            "Conversion error: op code with value = {} doesn't exist.",
                            op_code
                        );
                    }
                }
            }
            Some(opcode::OpType::Rs) => {
                let (r1, r2) = self.read_op_registers(self.ilc + REGISTERS_OFFSET);
                trace!("RS instruction, r1 = {} r2 = {}", r1, r2);
                let address = self.read_op_address(self.ilc);

                match FromPrimitive::from_u8(op_code) {
                    Some(o) => {
                        return instruction::Instruction::RegisterStorage(o, r1, r2, address);
                    }
                    None => {
                        error!(
                            "Conversion error: op code with value = {} doesn't exist.",
                            op_code
                        );
                    }
                }
            }
            Some(opcode::OpType::Im) => {
                let (r1, value) = self.read_r1_and_value(self.ilc);
                trace!("IM instruction {} {}", r1, value);
                match FromPrimitive::from_u8(op_code) {
                    Some(o) => {
                        return instruction::Instruction::Immediate(o, r1, value);
                    }
                    None => {
                        error!(
                            "Conversion error: op code with value = {} doesn't exist.",
                            op_code
                        );
                    }
                }
            }
            Some(opcode::OpType::Ch) => {
                let (r1, r2) = self.read_op_registers(self.ilc + REGISTERS_OFFSET);
                trace!("CH instruction, r1 = {} r2 = {}", r1, r2);
                let address = self.read_op_address(self.ilc);

                match FromPrimitive::from_u8(op_code) {
                    Some(o) => {
                        return instruction::Instruction::Character(o, r1, r2, address);
                    }
                    None => {
                        error!(
                            "Conversion error: op code with value = {} doesn't exist.",
                            op_code
                        );
                    }
                }
            }
            _ => {
                error!("Unsupported op type for op code = {}", op_code);
            }
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
            Some(opcode::OpType::Rr) | Some(opcode::OpType::Rrm) => {
                self.ilc += 0x2;
            }
            _ => {
                self.ilc += 0x4;
            }
        }
        if trace {
            // NOTE: it eases debugging very much and should be taken
            // into account when moving to opcodes only
            trace!("{:?}", next_instr);
        }
        // TODO: Match instruction format (RR/RS/IM/CH) instead of type,
        // cause handling one instruction format is in most cases the same.
        // Next, instruction types can me map to functions, e.g.
        // A => |a, b| a + b;
        // S => |a, b| a - b;
        match next_instr {
            instruction::Instruction::RegisterStorage(op_code, r1, _r2, address) => {
                match op_code {
                    opcode::OpCodeValue::L => {
                        let value = self.mem.borrow().read_word(address as usize);
                        self.mem.borrow_mut().write_reg(r1 as usize, value);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::St => {
                        let value = self.mem.borrow().read_reg(r1 as usize);
                        self.mem.borrow_mut().write_word(address as usize, value);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Swap => {
                        // TODO: set proper CCR bits
                        let r1_value = self.mem.borrow().read_reg(r1 as usize);
                        let swap_value = self.mem.borrow().read_word(address as usize);
                        debug!(
                            "running swap instruction with {} ({} reg) and {} ({} address)",
                            r1_value, r1, swap_value, address
                        );
                        self.mem.borrow_mut().write_reg(r1 as usize, swap_value);
                        self.mem.borrow_mut().write_word(address as usize, r1_value);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Svc => {
                        // TODO: consider additional interface for register
                        // value retrieval
                        let action_id = self.mem.borrow().read_reg(r1 as usize);
                        trace!("Supervisor call with id {}", action_id);
                        match action_id {
                            0 => return Ok(sv::Action::Exit),
                            1 => return Ok(sv::Action::ReadInt(address)),
                            3 => return Ok(sv::Action::ReadChar(address)),
                            5 => return Ok(sv::Action::WriteInt(address)),
                            7 => return Ok(sv::Action::WriteChar(address)),
                            _ => return Ok(sv::Action::Exit),
                        }
                    }
                    opcode::OpCodeValue::And => {
                        // TODO: set proper CCR bits
                        self.alu2((r1 as usize) * 4, address as usize, &AluOpType::And);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Or => {
                        // TODO: set proper CCR bits
                        self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Or);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Xor => {
                        // TODO: set proper CCR bits
                        self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Xor);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Not => {
                        // TODO: set proper CCR bits
                        self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Not);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::A => {
                        // TODO: compare sum to 0 and set CCR
                        self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Add);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::S => {
                        // TODO: compare sum to 0 and set CCR
                        self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Sub);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Rs => {
                        // TODO: compare sum to 0 and set CCR
                        self.alu2((r1 as usize) * 4, address as usize, &AluOpType::RSub);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::M => {
                        // TODO: compare sum to 0 and set CCR
                        self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Mul);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::D => {
                        // TODO: compare sum to 0 and set CCR
                        self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Div);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Rd => {
                        // TODO: compare sum to 0 and set CCR
                        self.alu2((r1 as usize) * 4, address as usize, &AluOpType::RDiv);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Rem => {
                        // TODO: compare sum to 0 and set CCR
                        self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Rem);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Rrem => {
                        // TODO: compare sum to 0 and set CCR
                        self.alu2((r1 as usize) * 4, address as usize, &AluOpType::RRem);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Min => {
                        let r1_value = self.mem.borrow().read_reg(r1 as usize);
                        let address_value = self.mem.borrow().read_word(address as usize);
                        self.mem
                            .borrow_mut()
                            .write_reg(r1 as usize, cmp::min(r1_value, address_value) as i32);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Max => {
                        let r1_value = self.mem.borrow().read_reg(r1 as usize);
                        let address_value = self.mem.borrow().read_word(address as usize);
                        self.mem
                            .borrow_mut()
                            .write_reg(r1 as usize, cmp::max(r1_value, address_value) as i32);
                        return Ok(sv::Action::None);
                    }
                    _ => {
                        warn!("Unsupported instruction with op code = {:?}", op_code);
                    }
                }
            }
            instruction::Instruction::RegisterRegister(op_code, r1, r2) => {
                match op_code {
                    // TODO: add trace messages?
                    // TODO: add indirect bit support
                    opcode::OpCodeValue::Lr => {
                        trace!(
                            "running LoadRegister instruction with r1 = {} r2 = {}",
                            r1,
                            r2
                        );
                        let value = self.mem.borrow().read_reg(r2 as usize);
                        self.mem.borrow_mut().write_reg(r1 as usize, value);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Lnr => {
                        trace!(
                            "running LoadNegativeRegister instruction with r1 = {} r2 = {}",
                            r1,
                            r2
                        );
                        let value = self.mem.borrow().read_reg(r2 as usize);
                        self.mem.borrow_mut().write_reg(r1 as usize, !value + 1);
                        return Ok(sv::Action::None);
                    }
                    // TODO: add indirect bit support
                    opcode::OpCodeValue::Str => {
                        trace!(
                            "running StoreRegister instruction with r1 = {} r2 = {}",
                            r1,
                            r2
                        );
                        let value = self.mem.borrow().read_reg(r1 as usize);
                        self.mem.borrow_mut().write_reg(r2 as usize, value);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Swapr => {
                        // TODO: set proper CCR bits
                        let r1_value = self.mem.borrow().read_reg(r1 as usize);
                        let r2_value = self.mem.borrow().read_reg(r2 as usize);
                        debug!(
                            "running swapr instruction with {} ({} reg) and {} ({} reg)",
                            r1_value, r1, r2_value, r2
                        );
                        self.mem.borrow_mut().write_reg(r1 as usize, r2_value);
                        self.mem.borrow_mut().write_reg(r2 as usize, r1_value);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Andr => {
                        self.alu2((r1 as usize) * 4, (r2 as usize) * 4, &AluOpType::And);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Orr => {
                        self.alu2((r1 as usize) * 4, (r2 as usize) * 4, &AluOpType::Or);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Xorr => {
                        self.alu2((r1 as usize) * 4, (r2 as usize) * 4, &AluOpType::Xor);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Notr => {
                        self.alu2((r1 as usize) * 4, (r2 as usize) * 4, &AluOpType::Not);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Balr => {
                        self.mem
                            .borrow_mut()
                            .write_reg(r1 as usize, self.ilc as i32);
                        self.ilc = self.mem.borrow().read_reg(r2 as usize) as usize;
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Ar => {
                        self.alu2((r1 as usize) * 4, (r2 as usize) * 4, &AluOpType::Add);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Sr => {
                        self.alu2((r1 as usize) * 4, (r2 as usize) * 4, &AluOpType::Sub);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Rsr => {
                        self.alu2((r1 as usize) * 4, (r2 as usize) * 4, &AluOpType::RSub);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Mr => {
                        self.alu2((r1 as usize) * 4, (r2 as usize) * 4, &AluOpType::Mul);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Dr => {
                        self.alu2((r1 as usize) * 4, (r2 as usize) * 4, &AluOpType::Div);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Rdr => {
                        self.alu2((r1 as usize) * 4, (r2 as usize) * 4, &AluOpType::RDiv);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Remr => {
                        self.alu2((r1 as usize) * 4, (r2 as usize) * 4, &AluOpType::Rem);
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Rremr => {
                        self.alu2((r1 as usize) * 4, (r2 as usize) * 4, &AluOpType::RRem);
                        return Ok(sv::Action::None);
                    }
                    _ => {
                        warn!("Unsupported instruction with op code = {:?}", op_code);
                    }
                }
            }
            instruction::Instruction::RegisterRegisterMask(op_code, m1, r2) => {
                match op_code {
                    opcode::OpCodeValue::Bcsr => {
                        // TODO: compare sum to 0 and set CCR
                        if self.mask_ccr(m1) {
                            self.ilc = self.mem.borrow().read_reg(r2 as usize) as usize
                        }
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Bcrr => {
                        debug!("running BCRR instruction with {} mask and {} reg", m1, r2);
                        if !self.mask_ccr(m1) {
                            self.ilc = self.mem.borrow().read_reg(r2 as usize) as usize
                        }
                        return Ok(sv::Action::None);
                    }
                    opcode::OpCodeValue::Sacr => {
                        debug!("running SACR instruction with {} mask and {} reg", m1, r2);
                        if self.mask_ccr(m1) {
                            self.mem.borrow_mut().write_reg(r2 as usize, i32::MIN);
                        } else {
                            self.mem.borrow_mut().write_reg(r2 as usize, 0);
                        }
                        return Ok(sv::Action::None);
                    }
                    _ => {
                        warn!("Unsupported instruction with op code = {:?}", op_code);
                    }
                }
            }
            instruction::Instruction::Immediate(op_code, r1, value) => match op_code {
                opcode::OpCodeValue::Li => {
                    self.mem.borrow_mut().write_reg(r1 as usize, value as i32);
                    self.set_ccr(value);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Andi => {
                    self.alu1((r1 as usize) * 4, value, &AluOpType::And);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Ori => {
                    self.alu1((r1 as usize) * 4, value, &AluOpType::Or);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Xori => {
                    self.alu1((r1 as usize) * 4, value, &AluOpType::Xor);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Noti => {
                    self.alu1((r1 as usize) * 4, value, &AluOpType::Not);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Ai => {
                    self.alu1((r1 as usize) * 4, value, &AluOpType::Add);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Si => {
                    self.alu1((r1 as usize) * 4, value, &AluOpType::Sub);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Rsi => {
                    self.alu1((r1 as usize) * 4, value, &AluOpType::RSub);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Mi => {
                    self.alu1((r1 as usize) * 4, value, &AluOpType::Mul);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Di => {
                    self.alu1((r1 as usize) * 4, value, &AluOpType::Div);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Rdi => {
                    self.alu1((r1 as usize) * 4, value, &AluOpType::RDiv);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Remi => {
                    self.alu1((r1 as usize) * 4, value, &AluOpType::Rem);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Rremi => {
                    self.alu1((r1 as usize) * 4, value, &AluOpType::RRem);
                    return Ok(sv::Action::None);
                }
                _ => {
                    warn!("Unsupported instruction with op code = {:?}", op_code);
                }
            },
            instruction::Instruction::Character(op_code, r1, _r2, address) => match op_code {
                opcode::OpCodeValue::Andc => {
                    self.alu2((r1 as usize) * 4, address as usize, &AluOpType::And);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Orc => {
                    self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Or);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Xorc => {
                    self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Xor);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Notc => {
                    self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Not);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Ac => {
                    self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Add);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Sc => {
                    self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Sub);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Rsc => {
                    self.alu2((r1 as usize) * 4, address as usize, &AluOpType::RSub);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Mc => {
                    self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Mul);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Dc => {
                    self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Div);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Rdc => {
                    self.alu2((r1 as usize) * 4, address as usize, &AluOpType::RDiv);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Remc => {
                    self.alu2((r1 as usize) * 4, address as usize, &AluOpType::Rem);
                    return Ok(sv::Action::None);
                }
                opcode::OpCodeValue::Rremc => {
                    self.alu2((r1 as usize) * 4, address as usize, &AluOpType::RRem);
                    return Ok(sv::Action::None);
                }
                _ => {
                    warn!("Unsupported instruction with op code = {:?}", op_code);
                }
            },
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
