use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines: Vec<String> = Vec::new();

    for line in handle.lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }

    // Print the array of strings to verify
    for (index, line) in lines.iter().enumerate() {
        println!("Line {}: {}", index + 1, line);
    }
}
