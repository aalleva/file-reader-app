use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        println!("Gile path is: {}", file_path);
        
        let file = File::open(&file_path);
        let file = match file {
            Ok(file) => file,
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::NotFound => {
                        panic!("File not found: {}", error)
                    }
                    _ => {
                        panic!("Error opening file: {}", error)
                    }
                }
            }
        };
        
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(error) => {
                    panic!("Error reading line: {}", error)
                }
            }
        }

    } else {
        println!("No file path provided");
    }

}