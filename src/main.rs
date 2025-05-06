use interface::Interface;
use std::io::stdin;
use termion::input::TermRead;

mod interface;

fn main() {
    let mut interface = Interface::new();

    for input in stdin().keys() {
        match input {
            Ok(input) => interface.handle_input(input),
            Err(err) => panic!("Error: {err}"),
        }
    }
}
