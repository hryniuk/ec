use std::path::Path;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead,BufReader};

pub mod record;
pub mod data_triple;

pub struct Alf {
    pub records: Vec<record::Record>
}


impl Alf {
    pub fn from_file(path: &Path) -> Result<Alf, Error> {
        let reader: BufReader<File> = BufReader::new(File::open(path)?);

        // TODO: drop records with invalid sequence numbers
        let records: Result<Vec<record::Record>, Error> = reader.lines()
            .map(|line: Result<String, Error>| {
                line.and_then(record::Record::from)
            })
            .collect();

        records.and_then(|r| Ok(Alf{records: r}))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_alf() {
    }
}
