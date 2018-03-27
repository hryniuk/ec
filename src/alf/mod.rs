use std::path::Path;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead,BufReader};

mod record;

pub struct Alf {
    records: Vec<record::Record>
}

fn line_to_record(line: Result<String, Error>) -> Result<record::Record, Error> {
    match line {
        Ok(line) => record::Record::from(line),
        Err(e) => Err(e),
    }
}

impl Alf {
    pub fn from_file(path: &Path) -> Result<Alf, Error> {
        let reader: BufReader<File> = BufReader::new(File::open(path)?);

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
