use std::{error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    let f: File = File::open("hello.txt")?;
    //let f: Result<File, std::io::Error> = File::open("hello.txt");
    Ok(())
}
