use std::str;

pub const ADDRESS_LENGTH: usize = 4;
const DATA_FIELD_LENGTH: u32 = 2;

pub struct DataTriple {
    pub address: usize,
    pub data_fields: Vec<u8>,
}

impl DataTriple {
    /// Creates DataTriple from string containing
    /// 4 bytes of address and
    /// `count` pairs of data fields, each 2 bytes long
    pub fn from(count: u8, data_triple_chunk: &str) -> DataTriple {
        // TODO: add error handling here
        // TODO: adjust types (usize instead of u32? etc)
        let address = u32::from_str_radix(&data_triple_chunk[..ADDRESS_LENGTH], 16).unwrap();

        let data_fields: Vec<u8> = data_triple_chunk
            [ADDRESS_LENGTH..ADDRESS_LENGTH + (u32::from(count) * DATA_FIELD_LENGTH) as usize]
            .as_bytes()
            .chunks(DATA_FIELD_LENGTH as usize)
            .map(|c| u8::from_str_radix(str::from_utf8(c).unwrap(), 16).unwrap())
            .collect::<Vec<u8>>();

        for data_field in data_fields.iter() {
            debug!("Reading data field: {}", data_field);
        }
        debug!("Read data triple count: {} address: {}", count, address);
        DataTriple {
            address: address as usize,
            data_fields,
        }
    }
}
