use ec::cpu::instruction;
use std::num;

pub mod data_triple;
pub mod record;

/// Struct representing ALF - absolute load file, which describes
/// the contents of computer memory prior to execution.
pub struct Alf {
    pub records: Vec<record::Record>,
    pub start_address: instruction::Address,
}

impl Alf {
    fn read_start_address(end_record: &String) -> Result<instruction::Address, num::ParseIntError> {
        instruction::Address::from_str_radix(&end_record[3..], 16)
    }
    fn assert_sequence_numbers(records: &Vec<record::Record>) -> bool {
        for (i, record) in records.iter().enumerate() {
            if record.sequence_number != i as u32 {
                return false;
            }
        }

        true
    }
    pub fn from(mut lines: Vec<String>) -> Result<Alf, String> {
        let start_address: instruction::Address;

        match lines.pop() {
            Some(end_record) => {
                if !end_record.starts_with("END") {
                    return Err(
                        "ALF's last line should include END record with a start address".to_owned(),
                    );
                }

                match Alf::read_start_address(&end_record) {
                    Ok(address) => start_address = address,
                    Err(e) => {
                        return Err(format!(
                            "Cannot read start address from end record {}: {}",
                            end_record, e
                        ))
                    }
                }
            }
            None => {
                return Err("ALF cannot be empty".to_owned());
            }
        }

        match lines
            .into_iter()
            .map(record::Record::from)
            .collect::<Result<Vec<_>, String>>()
        {
            Ok(records) => {
                if Alf::assert_sequence_numbers(&records) {
                    Ok(Alf {
                        records,
                        start_address,
                    })
                } else {
                    Err(format!("Records indices should be consecutive numbers"))
                }
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_returns_error_on_empty_alf() {
        assert!(Alf::from(vec![]).is_err());
    }
    #[test]
    fn test_returns_error_when_end_is_missing() {
        let lines = vec![String::from("E10241A2301020304207FF1BEC")];

        assert!(Alf::from(lines).is_err());
    }

    #[test]
    fn test_returns_error_on_invalid_sequence_numbers() {
        let lines = vec![String::from("6001400102E010000"), String::from("END000A")];

        assert!(Alf::from(lines).is_err());
    }
    #[test]
    fn test_creates_alf_from_valid_source() {
        let lines = vec![String::from("6000400102E010000"), String::from("END000A")];
        let expected_records_count = lines.len() - 1;
        let alf = Alf::from(lines);

        assert!(alf.is_ok());
        let records = alf.unwrap().records;
        assert_eq!(expected_records_count, records.len());
    }
}
