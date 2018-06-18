mod instruction;

use ec::mem;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Cpu {
    ilc: u32,
    mem: Rc<RefCell<mem::Memory>>,
}

impl Cpu {
    pub fn new(mem: Rc<RefCell<mem::Memory>>) -> Cpu {
        Cpu { ilc: 0, mem }
    }
    pub fn poll() {}
}
