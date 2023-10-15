use rustyline::Editor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rl = Editor::<()>::new()?;

    Ok(())
}
