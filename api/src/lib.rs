use std::error::Error;

fn start() -> Result<(), Box<dyn Error>> {
    println!("Compiled!");
    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}