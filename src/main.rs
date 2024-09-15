use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();

    let mut file = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(error) => return Err(error),
    };

    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(error) => return Err(error),
    }

    Ok(string)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(file_path) = &args.get(1) {
        println!("Reading file: {}", file_path);

        match read_file_contents(PathBuf::from(file_path)) {
            Ok(contents) => {
                println!("File contents:");
                println!("{}", contents);
            }
            Err(error) => {
                println!("Error reading file: {}", error);
            }
        }
    }
}