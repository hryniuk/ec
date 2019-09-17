use getopts;
use simplelog::*;
use std;
use std::process;

fn init_logger(log_level: LevelFilter) {
    match TermLogger::new(log_level, Config::default()) {
        Some(tl) => match CombinedLogger::init(vec![tl]) {
            Ok(_) => {}
            Err(e) => error!("Cannot initialize logger: {}", e),
        },
        None => error!("Cannot initialize logger"),
    }
}

fn print_usage(program: &str, opts: &getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn parse(args: &Vec<String>) -> std::path::PathBuf {
    let program: String = args[0].clone();

    // TODO: add usage
    let mut options = getopts::Options::new();
    options.optflag("q", "quiet", "disable logs");
    options.optflag("v", "verbose", "be verbose");
    options.optflag(
        "t",
        "trace",
        "be even more verbose - enables TRACE log level",
    );
    options.optopt("f", "alf", "path to ALF", "PATH");

    let matches = match options.parse(args.into_iter().skip(1).collect::<Vec<_>>()) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("t") {
        init_logger(LevelFilter::Trace);
    } else if matches.opt_present("v") {
        init_logger(LevelFilter::Debug);
    } else if matches.opt_present("q") {
        init_logger(LevelFilter::Off);
    } else {
        init_logger(LevelFilter::Info);
    }

    let alf_path: std::path::PathBuf = if matches.opt_present("f") {
        std::path::PathBuf::from(matches.opt_str("f").clone().unwrap())
    } else {
        print_usage(&program, &options);
        process::exit(1);
    };

    alf_path
}
