use crate::read_lines::read_lines;

fn parse_input(filename: &str) {
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                dbg!(ip);
            }
        }
    }
    else {
        println!("File not read");
    }
}

pub fn run() {
    println!("Day XX Part A");
    let input_filename = "inputs/dayXX/test.txt";

    parse_input(input_filename);
}