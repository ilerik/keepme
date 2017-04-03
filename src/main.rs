extern crate pretty_env_logger;
extern crate keepme;

use keepme::core::KeepmeCore;

fn main() {
    pretty_env_logger::init().unwrap();
    PerimeterCore::start_server("0.0.0.0:3000", "/tmp/data-rs/");
}
