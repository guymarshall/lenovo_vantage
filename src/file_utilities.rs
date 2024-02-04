use std::{env, fs, io};
use crate::constants::{BRIGHTNESS, CONSERVATION_MODE, FAN_MODE, FN_LOCK};


fn file_exists(file_path: &str) -> Result<(), io::Error> {
    if fs::read_to_string(file_path).is_ok() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }
}

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap_or("".to_string())
}

pub fn write_to_file(file_path: &str, contents: String) -> io::Result<()> {
    fs::write(file_path, contents)
}

pub fn check_is_linux() {
    if env::consts::OS != "linux" {
        eprintln!("Error: This program is intended to run on Linux only.");
        std::process::exit(1);
    }
}

pub fn check_files_exist() {
    if let Err(err) = file_exists(FN_LOCK) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    if let Err(err) = file_exists(CONSERVATION_MODE) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    if let Err(err) = file_exists(FAN_MODE) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    if let Err(err) = file_exists(BRIGHTNESS) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}