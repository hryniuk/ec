use std::env;
use std::process;

#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::*;
use std::cell::RefCell;
use std::fs;
use std::io;
use std::io::BufRead;
use std::rc::Rc;

mod args;
mod ec;

fn init_logger() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Error, Config::default()).unwrap(),
    ]).unwrap();
}

fn main() {
    init_logger();

    // use library to parse args
    let args: Vec<String> = env::args().collect();

    let alf_path: std::path::PathBuf = args::get_alf_path(&args).unwrap_or_else(|| {
        print!("{}", args::usage(&args[0]));
        process::exit(1);
    });

    let alf_file = fs::File::open(&alf_path).unwrap_or_else(|e| {
        error!("Cannot open file {}: {}", alf_path.to_str().unwrap(), e);
        process::exit(1);
    });

    let source_lines: Vec<String> = io::BufReader::new(alf_file)
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()
        .unwrap_or_else(|e| {
            error!(
                "Cannot read lines from file {}: {}",
                alf_path.to_str().unwrap(),
                e
            );
            process::exit(1);
        });

    let alf = ec::alf::Alf::from(source_lines).unwrap_or_else(|e| {
        // TODO: avoid this unwrap in some way
        error!("{}: {}", alf_path.to_str().unwrap(), e);
        process::exit(1);
    });

    let mut ecc = ec::Ec::new(Rc::new(RefCell::new(ec::mem::Memory::from(&alf))));
    ecc.run();
}
