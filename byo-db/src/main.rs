use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s: Result<String, io::Error> = read_username_from_file()?;
    println!("{}", s);
    Ok(())
}
