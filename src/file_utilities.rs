use std::{fs, io};


pub fn file_exists(file_path: &str) -> Result<(), io::Error> {
    if fs::read_to_string(file_path).is_ok() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }
}

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or("".to_string())
}

pub fn write_to_file(file_path: &str, contents: &str) -> io::Result<()> {
    fs::write(file_path, contents)
}