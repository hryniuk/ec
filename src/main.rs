#[macro_use]
extern crate log;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate text_io;
extern crate getopts;
extern crate num_traits;
extern crate simplelog;

use std::cell::RefCell;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process;
use std::rc::Rc;

mod args;
pub mod ec;

fn read_alf(path: &std::path::PathBuf) -> ec::alf::Alf {
    let alf_file = fs::File::open(&path).unwrap_or_else(|e| {
        error!("Cannot open file {}: {}", path.to_str().unwrap(), e);
        process::exit(1);
    });

    let source_lines: Vec<String> = io::BufReader::new(alf_file)
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()
        .unwrap_or_else(|e| {
            error!(
                "Cannot read lines from file {}: {}",
                path.to_str().unwrap(),
                e
            );
            process::exit(1);
        });

    let alf = ec::alf::Alf::from(source_lines).unwrap_or_else(|e| {
        // TODO: avoid this unwrap in some way
        error!("{}: {}", path.to_str().unwrap(), e);
        process::exit(1);
    });

    return alf;
}

fn main() {
    let alf = read_alf(&args::parse(&env::args().collect()));
    let mut ecc = ec::Ec::new(Rc::new(RefCell::new(ec::mem::Memory::from(&alf))));

    match ecc.run() {
        Ok(_) => (),
        Err(e) => {
            // TODO: add error msg
            error!("EC exited with error");
            process::exit(e as i32);
        }
    }
}
