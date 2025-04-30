use session::Session;
use std::error::Error;
use termion::color;

mod session;

pub struct Interface {
    session: Session,
}

impl Interface {
    pub fn new() -> Self {
        Self {
            session: Session::new(),
        }
    }

    pub async fn login(&self) -> Result<(), Box<dyn Error>> {
        let body = reqwest::get("http://127.0.0.1:3000/").await?.text().await?;

        println!("body = {body:?}");

        Ok(())
    }

    pub async fn parse(&mut self, nbytes: usize, input: &mut String) -> Result<(), Box<dyn Error>> {
        input.pop();
        let command = input.as_str();

        if nbytes < 1 {
            return Ok(());
        }

        if command == "login" {
            self.login().await?;
        } else {
            println!(
                "{}command \"{input}\" does not exist{}",
                color::Fg(color::Red),
                color::Fg(color::Reset)
            );
        }

        // match input.as_str() {
        //     "login" => self.login(),
        //     _ => println!("{input}: command not found"),
        // };
        Ok(())
    }
}
