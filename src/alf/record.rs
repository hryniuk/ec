//use std::vec::Vec;
use std::io::{Error,ErrorKind};
use std::u32;
use std::vec::Vec;
use std::str;

extern crate simplelog;
use simplelog::*;


const CHECKSUM_LENGTH: u32  = 1;
// TODO: consider chaning it to offset (so 4 + CHECKSUM_LENGTH) instead
const ADDRESS_LENGTH: usize = 4;
const TRIPLES_INDEX: usize = 4;
const DATA_FIELD_LENGHT: u32 = 2;


struct DataTriple {
    count: u8,
    address: u32,
    data_fields: Vec<u8>
}


/// Record is a basic unit of absolute load file (ALF)
pub struct Record {
    sequence_number: u32,
    data_triples: Vec<DataTriple>
}


impl Record {
    pub fn from(record_line: String) -> Result<Record, Error> {
        assert!(record_line.len() > 0);
        // TODO: consider converting string to Vec<Ops>
        if !Record::is_valid(&record_line) {
            return Err(Error::new(ErrorKind::Other, "invalid record line"));
        }

        let sequence_number = Record::read_sequence_number(&record_line).unwrap_or(0);
        debug!("Read sequence number {} from record line {}", sequence_number, record_line);

        let data_triples = Record::read_data_triples(&record_line);

        Ok(Record{sequence_number, data_triples})
    }

    fn read_checksum(record_line: &String) -> Option<u32> {
        record_line.chars().next().unwrap().to_digit(16)
    }

    fn read_sequence_number(record_line: &String) -> Option<u32> {
        // TODO: handle this Result correctly
        Some(u32::from_str_radix(&record_line[1..4], 16).unwrap())
    }

    // TODO: consider changing result type
    // to handle errors (Result? Option? empty Vec on error?)
    fn read_data_triples(record_line: &String) -> Vec<DataTriple> {
        let mut i: usize = TRIPLES_INDEX as usize;
        let mut data_triples: Vec<DataTriple> = Vec::new();

        while i < record_line.len() {
            // TODO: add error handling here
            // TODO: move it to DataTriple::new() function
            let count = u32::from_str_radix(&record_line[i..i+1], 16).unwrap();

            let address = u32::from_str_radix(&record_line[i+1..i+ADDRESS_LENGTH+1], 16).unwrap();

            let data_offset: usize = i+ADDRESS_LENGTH+1;
            let data_fields: Vec<u8> = record_line[data_offset..data_offset+(count * DATA_FIELD_LENGHT) as usize]
                .as_bytes()
                .chunks(DATA_FIELD_LENGHT as usize)
                .map(|c| u8::from_str_radix(str::from_utf8(c).unwrap(), 16).unwrap())
                .collect::<Vec<u8>>();

            for data_field in data_fields.iter() {
                debug!("Reading data field: {}", data_field);
            }
            debug!("Read data triple count: {} address: {}", count, address);

            data_triples.push(DataTriple{count: count as u8, address, data_fields});

            i += (1 + count * 2 + ADDRESS_LENGTH as u32) as usize;
        }

        data_triples
    }

    fn calculate_checksum(record_line: &String) -> u32 {
        (record_line[1..].chars().map(|c| c.to_digit(16).unwrap()).sum::<u32>()) % 16
    }

    fn is_valid(record_line: &String) -> bool {
        // TODO: check if all characters are [0-9A-FN]
        match Record::read_checksum(record_line) {
            Some(checksum) => checksum == Record::calculate_checksum(record_line),
            None => false,
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_result_is_constructed_with_valid_record_line() {
        let valid_record_line = String::from("E10241A2301020304207FF1BEC");
        let record = Record::from(valid_record_line);
        assert!(record.is_ok());
    }

    #[test]
    fn test_error_is_reported_on_invalid_checksum() {
        let invalid_checksum_line = String::from("E20241A2301020304207FF1BEC");
        let record = Record::from(invalid_checksum_line);
        assert!(record.is_err());
    }
}
