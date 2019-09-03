#[macro_use]
extern crate log;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate text_io;

pub mod ec;

fn main() {
    println!("Hello fconv: {}!", ec::cpu::fconv::bytes_to_float());
}
