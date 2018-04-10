//use std::vec::Vec;
use std::io::{Error,ErrorKind};


pub struct Record {
    data: String
}


impl Record {
    pub fn from(record_line: String) -> Result<Record, Error> {
        assert!(record_line.len() > 0);
        // TODO: consider converting string to Vec<Ops>
        if !Record::is_valid(&record_line) {
            return Err(Error::new(ErrorKind::Other, "invalid record line"));
        }

        let sequence_number = Record::read_sequence_number(&record_line);
        debug!("Read sequence number {} from record line {}", sequence_number.unwrap(), record_line);

        Ok(Record{data: record_line})
    }

    fn read_checksum(record_line: &String) -> Option<u32> {
        record_line.chars().next().unwrap().to_digit(16)
    }

    fn read_sequence_number(record_line: &String) -> Option<u32> {
        // TODO: handle this Result correctly
        Some(u32::from_str_radix(&record_line[1..4], 16).unwrap())
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
