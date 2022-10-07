use std::fs::File;
use std::io::{Result, Write};

pub fn write(data: String) -> Result<()> {
    let path = "results.json";
    let mut output = File::create(path)?;
    write!(output, "{}", data);
    Ok(())
}