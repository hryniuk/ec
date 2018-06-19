use ec::cpu;
use ec::mem;
use std::cell::RefCell;
use std::rc::Rc;

pub enum Action {
    None,
    Exit,
    WriteInt(cpu::instruction::Address),
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
            match cpu.poll(true) {
                Ok(Action::Exit) => break,
                Ok(Action::WriteInt(addr)) => {
                    print!("{}", self.mem.borrow().read_word(addr as usize));
                }
                Err(e) => break,
                _ => break,
            }
        }
    }
}
