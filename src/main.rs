#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();

    println!("Nothing to see here. Move along.");
}
