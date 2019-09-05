#[macro_use]
extern crate log;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate text_io;

pub mod ec;

use std::io;

fn main() {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<f32>() {
        Err(..) => println!("this was not an integer: {}", trimmed),
        Ok(i) => {
            println!("{} converted to {:x?}", i, ec::cpu::fconv::float32_to_bytes(i));
        }
    };
}
