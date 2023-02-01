use std::fs::OpenOptions;
use std::io::Write;

pub fn write(data: String, path: &String) {
    let mut file_to_write = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap_or_else(|error| {
            panic!("Problem opening the file: {:?}", error);
        });

    write!(file_to_write, "{}", data).unwrap_or_else(|error| {
        panic!("Problem writing to file: {:?}", error);
    })
}

