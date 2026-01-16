use std::error::Error;

mod birthday;

pub fn run() -> Result<(), Box<dyn Error>> {
    birthday::handle_birthday();

    Ok(())
}
