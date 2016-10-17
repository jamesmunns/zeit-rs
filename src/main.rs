#[macro_use]
extern crate clap;

mod cli;

fn main() {
    cli::parse_cli();
}
