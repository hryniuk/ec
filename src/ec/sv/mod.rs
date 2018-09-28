use ec::cpu;
use ec::mem;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

pub enum Action {
    None,
    /// Exit the running program and clean up after it.
    Exit,
    /// Read the integer from the stdin and store it at the effective address
    /// of the SVC
    ReadInt(cpu::instruction::Address),
    /// Read the character from the stdin and store it at the effective address
    /// of the SVC
    ReadChar(cpu::instruction::Address),
    /// Write the word at the effective address as an integer
    /// on the output stream.
    WriteInt(cpu::instruction::Address),
    /// Write the character at the effective address as an character
    /// on the output stream.
    WriteChar(cpu::instruction::Address),
}

pub struct Supervisor {
    mem: Rc<RefCell<mem::Memory>>,
}

impl Supervisor {
    pub fn new(mem: Rc<RefCell<mem::Memory>>) -> Supervisor {
        Supervisor { mem }
    }
    pub fn run_with(&self, cpu: &mut cpu::Cpu) {
        loop {
            // TODO: consider "output buffer" to make testing easier
            match cpu.poll(true) {
                Ok(Action::Exit) => break,
                Ok(Action::ReadInt(addr)) => {
                    let read_value: i32 = read!();
                    self.mem.borrow_mut().write_word(addr as usize, read_value);
                }
                Ok(Action::ReadChar(addr)) => {
                    // TODO: accept only ASCII characters
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(n) => {
                            if n > 0 {
                                self.mem
                                    .borrow_mut()
                                    .set(addr as usize, input.into_bytes()[0]);
                            }
                        }
                        Err(error) => panic!("error: {}", error),
                    }
                }
                Ok(Action::WriteInt(addr)) => {
                    println!("{}", self.mem.borrow().read_word(addr as usize));
                }
                Ok(Action::WriteChar(addr)) => {
                    println!("{}", self.mem.borrow().get(addr as usize) as char);
                }
                Ok(Action::None) => (),
                Err(_) => break,
            }
        }
    }
}
