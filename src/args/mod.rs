pub fn usage(name: &String) -> String {
    format!("Usage:\n\t{} <alf_path>", name)
}

pub fn get_alf_path(args: &[String]) -> Option<&str> {
    if args.len() <= 1 {
        return Option::None;
    }

    Option::Some(&args[1])
}