use std::io::{BufRead,BufReader};
use std::fs::File;
use std::env;
use std::process;

mod ec;
mod mem;
mod record;

fn usage(name: &String) {
    println!("Usage:\n\t{} <alf_path>", name);
}

fn check_line(line: String) {
    println!("{}", line);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        usage(&args[0]);
        process::exit(0);
    }

    let alf_path = &args[1];

    let reader = BufReader::new(File::open(alf_path).expect
        ("Cannot open file"));

    for line in reader.lines() {
        match line {
            Ok(v) => check_line(v),
            Err(e) => println!("Error on reading {} file: {}", &alf_path, e),
        }
    }

    let ecc = ec::Ec{mem : mem::new()};

}
