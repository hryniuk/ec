pub const SIZE : usize = 4048;
use ec;

pub struct Memory {
    mem: [u8; SIZE]
}

pub fn new() -> Memory {
    Memory {
        mem: [0; SIZE]
    }
}

impl Memory {
    pub fn from(alf: ec::alf::Alf) {

    }

    pub fn get_gpr(&self, index : usize) -> u32 {
        assert!(index < 16);
        let mut value : u32 = 0;
        for v in self.mem[index * 4..(index+1) * 4].iter() {
            value <<= 8;
            value += *v as u32;
        }
        value
    }

    pub fn set_gpr(&mut self, index: usize, value: u32) {
        assert!(index < 16);
        for (i, e) in self.mem[index * 4..(index+1) * 4].iter_mut().enumerate() {
            *e = (value & (0xff << (4-i-1) * 8)) as u8;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gpr() {
        let mut memory : Memory = new();
        let index = 1;
        assert_eq!(0, memory.get_gpr(index));

        let value = 0x30;
        memory.set_gpr(index, value);
        assert_eq!(value, memory.get_gpr(index));
    }

    #[test]
    fn test_memory_access() {
        let mut memory: Memory = new();
        let address = 0x70;
        let value = 0xa0;

        assert_eq!(0, memory.get(address));
        memory.set(address, value);
        assert_eq!(value, memory.get(address));
    }
}
