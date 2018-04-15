use std::io::{Error,ErrorKind};
use std::u32;
use std::vec::Vec;

extern crate simplelog;

use ec::alf::data_triple::DataTriple;
use ec::alf::data_triple::ADDRESS_LENGTH;


const CHECKSUM_LENGTH: u32  = 1;
// TODO: consider chaning it to offset (so 4 + CHECKSUM_LENGTH) instead
const TRIPLES_INDEX: usize = 4;


/// Record is a basic unit of absolute load file (ALF)
pub struct Record {
    sequence_number: u32,
    pub data_triples: Vec<DataTriple>
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
            let count = u32::from_str_radix(&record_line[i..i+1], 16).unwrap();
            let chunk_length: usize = ADDRESS_LENGTH + (2 * count as usize);
            data_triples.push(DataTriple::from(count as u8, &record_line[i+1..i+1+chunk_length]));

            i += (CHECKSUM_LENGTH + count * 2 + ADDRESS_LENGTH as u32) as usize;
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
