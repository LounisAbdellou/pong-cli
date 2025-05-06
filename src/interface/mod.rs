use termion::{color, event::Key};

pub struct Interface {}

impl Interface {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&mut self, nbytes: usize, input: &mut String) {
        input.pop();
        let command = input.as_str();

        if nbytes < 1 {
            return;
        }

        if command == "login" {
            // self.display_login();
        } else {
            println!(
                "{}command \"{input}\" does not exist{}",
                color::Fg(color::Red),
                color::Fg(color::Reset)
            );
        }
    }

    pub fn handle_input(&self, input: Key) {}
}
