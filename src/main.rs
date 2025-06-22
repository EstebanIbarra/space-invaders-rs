mod terminal;
use std::error::Error;
use std::io::Result;
use std::result::Result as StdResult;

fn setup() -> StdResult<terminal::Guard, Box<dyn Error>> {
    terminal::Guard::new()
}

fn main() -> Result<()> {
    let term = setup();
    if let Err(e) = term {
        panic!("Couldn't create a Terminal Guard: {}", e);
    }
    let _term = term.unwrap();
    Ok(())
}
