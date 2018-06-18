use ec::cpu;
use ec::mem;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Supervisor {
    mem: Rc<RefCell<mem::Memory>>,
}

impl Supervisor {
    pub fn new(mem: Rc<RefCell<mem::Memory>>) -> Supervisor {
        Supervisor { mem }
    }
    pub fn run_with(self, cpu: &cpu::Cpu) {}
}
