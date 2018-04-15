pub const SIZE: usize = 8096;

use ec;

pub struct Memory {
    mem: [u8; SIZE]
}


impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; SIZE]
        }
    }

    pub fn from(alf: &ec::alf::Alf) -> Memory {
        let mut mem: Memory = Memory::new();

        for record in &alf.records {
            mem.apply_record(&record);
        }

        mem
    }

    pub fn get(&self, address: usize) -> u8 {
        self.mem[address]
    }

    pub fn set(&mut self, address: usize, value: u8) {
        self.mem[address] = value;
    }

    pub fn get_gpr(&self, index: usize) -> u32 {
        assert!(index < 16);
        let mut value: u32 = 0;
        for v in self.mem[index * 4..(index + 1) * 4].iter() {
            value <<= 8;
            value += *v as u32;
        }
        value
    }

    pub fn set_gpr(&mut self, index: usize, value: u32) {
        assert!(index < 16);
        for (i, e) in self.mem[index * 4..(index + 1) * 4].iter_mut().enumerate() {
            *e = (value & (0xff << (4 - i - 1) * 8)) as u8;
        }
    }

    // TODO: find a better name
    fn apply_record(&mut self, record: &ec::alf::record::Record) {
        for data_triple in &record.data_triples {
            for (i, data_field) in data_triple.data_fields.iter().enumerate() {
                assert!(data_triple.address + i < SIZE);
                debug!("Setting {} value at {}", data_field, data_triple.address + i);
                self.set(data_triple.address + i, *data_field);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gpr() {
        let mut memory: Memory = Memory::new();
        let index = 1;
        assert_eq!(0, memory.get_gpr(index));

        let value = 0x30;
        memory.set_gpr(index, value);
        assert_eq!(value, memory.get_gpr(index));
    }

    #[test]
    fn test_memory_access() {
        let mut memory: Memory = Memory::new();
        let address = 0x70;
        let value = 0xa0;

        assert_eq!(0, memory.get(address));
        memory.set(address, value);
        assert_eq!(value, memory.get(address));
    }
}
