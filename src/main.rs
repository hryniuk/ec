use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;

#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::*;

mod ec;
mod mem;
mod record;
mod args;


fn check_line(line: String) {
    println!("{}", line);
}


fn init_logger()
{
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default()).unwrap(),
        ]
    ).unwrap();
}


fn main() {
    init_logger();

    let args: Vec<String> = env::args().collect();

    // TODO: change this behaviour to print usage silently
    let alf_path = args::get_alf_path(&args).expect(&args::usage(&args[0]));
    debug!("Read alf path from args: {}", alf_path);

    let reader = BufReader::new(File::open(alf_path).expect
    ("Cannot open file"));

    for line in reader.lines() {
        match line {
            Ok(v) => check_line(v),
            Err(e) => println!("Error on reading {} file: {}", &alf_path, e),
        }
    }

    let ecc = ec::Ec { mem: mem::new() };
}
