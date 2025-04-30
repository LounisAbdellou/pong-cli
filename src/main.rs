use interface::Interface;
use std::{error::Error, io};

mod interface;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut interface = Interface::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(nbytes) => interface.parse(nbytes, &mut input).await?,
            Err(err) => panic!("Error: {err}"),
        }

        input.clear();
    }
}
