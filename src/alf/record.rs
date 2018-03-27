//use std::vec::Vec;
use std::io::Error;


pub struct Record {
    data: String
}


impl Record {
    pub fn from(record_line: String) -> Result<Record, Error> {
        Ok(Record{data: record_line})
    }

    fn verify_checksum(record_line: String) -> bool {
        //let checksum = record_line;
        //let characters_sum: i32 = 0;
        //println!("{}", (record_line.chars().map(|c| c.to_digit(16).unwrap()).sum::<u32>() - 14) % 16);
        return true;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verify_checksum() {
        let checksum = String::from("E10241A2301020304207FF1BEC");
        assert!(Record::verify_checksum(checksum));
    }
}
