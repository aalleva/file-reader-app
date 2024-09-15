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

// Write Test Case for read_file_contents
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_read_file_contents_existent_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Test File").unwrap();

        let result = read_file_contents(file_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test File\n");
    }

    #[test]
    fn test_read_file_contents_non_existent_file() {
        let result = read_file_contents(PathBuf::from("non_existent_file.txt"));
        assert!(result.is_err());
    }
}