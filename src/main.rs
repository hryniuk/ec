use std::env;
use std::process;

#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::*;

mod ec;
mod mem;
mod alf;
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

    let alf_path: std::path::PathBuf = args::get_alf_path(&args).unwrap_or_else(||{
        print!("{}", args::usage(&args[0]));
        process::exit(1);
    });

    let alf = alf::Alf::from_file(&alf_path).unwrap_or_else(|e|{
        // TODO: avoid this unwrap in some way
        error!("{}: {}", alf_path.to_str().unwrap(), e);
        process::exit(1);
    });

    let ecc = ec::Ec { mem: mem::new() };
}
