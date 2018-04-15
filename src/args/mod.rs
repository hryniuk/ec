use std;

pub fn usage(name: &String) -> String {
    format!("Usage:\n\t{} <alf_path>", name)
}

pub fn get_alf_path(args: &[String]) -> Option<std::path::PathBuf> {
    if args.len() <= 1 {
        return Option::None;
    }

    Option::Some(std::path::PathBuf::from(&args[1]))
}
