pub const SIZE: usize = 8096;

use crate::ec;

pub struct Memory {
    mem: [u8; SIZE],
}

/// Implementation assumes each word consists of four internal
/// EC characters
impl Memory {
    pub fn new() -> Memory {
        Memory { mem: [0; SIZE] }
    }

    pub fn from(alf: &ec::alf::Alf) -> Memory {
        let mut mem: Memory = Memory::new();

        for record in &alf.records {
            mem.apply_record(&record);
        }

        mem
    }

    pub fn get(&self, address: usize) -> u8 {
        trace!("reading memory[{}] = {}", address, self.mem[address]);
        self.mem[address]
    }

    pub fn set(&mut self, address: usize, value: u8) {
        trace!("setting memory[{}] to {}", address, value);
        self.mem[address] = value;
    }

    pub fn read_reg(&self, address: usize) -> i32 {
        self.read_word(address * 4)
    }

    pub fn write_reg(&mut self, address: usize, value: i32) {
        self.write_word(address * 4, value);
    }

    pub fn read_word(&self, address: usize) -> i32 {
        let value: i32 = (i32::from(self.mem[address]) << 24)
            + (i32::from(self.mem[address + 1]) << 16)
            + (i32::from(self.mem[address + 2]) << 8)
            + i32::from(self.mem[address + 3]);
        trace!("reading word at {} = {}", address, value);
        value
    }

    pub fn write_word(&mut self, address: usize, value: i32) {
        trace!("setting word at {} to {}", address, value);
        self.mem[address] = (value >> 24) as u8;
        self.mem[address + 1] = ((value >> 16) & 0xff) as u8;
        self.mem[address + 2] = ((value >> 8) & 0xff) as u8;
        self.mem[address + 3] = (value & 0xff) as u8;
    }

    pub fn get_gpr(&self, index: usize) -> u32 {
        assert!(index < 16);
        let mut value: u32 = 0;
        for v in self.mem[index * 4..(index + 1) * 4].iter() {
            value <<= 8;
            value += u32::from(*v);
        }
        value
    }

    pub fn set_gpr(&mut self, index: usize, value: u32) {
        assert!(index < 16);
        for (i, e) in self.mem[index * 4..(index + 1) * 4].iter_mut().enumerate() {
            *e = (value & (0xff << ((4 - i - 1) * 8))) as u8;
        }
    }

    // TODO: find a better name
    fn apply_record(&mut self, record: &ec::alf::record::Record) {
        for data_triple in &record.data_triples {
            for (i, data_field) in data_triple.data_fields.iter().enumerate() {
                assert!(data_triple.address + i < SIZE);
                debug!(
                    "Setting {} value at {}",
                    data_field,
                    data_triple.address + i
                );
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

    #[test]
    fn test_read_written_value_from_word() {
        let mut memory: Memory = Memory::new();
        let address = 0x70;
        let expected_value = -i32::pow(2, 30);

        memory.write_word(address, expected_value);
        assert_eq!(expected_value, memory.read_word(address));
    }
}
