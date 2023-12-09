use crate::read_lines::read_lines;

fn predict_next(sequence: Vec<i32>) -> i32 {
    // find differences
    let differences:Vec<i32> = sequence.iter().as_slice().windows(2).map(|x| x[1] - x[0]).collect();

    if differences.iter().all(|x| *x == 0) {
        return sequence.last().unwrap() + 0;
    }

    return sequence.last().unwrap() + predict_next(differences);
}

fn process_input(filename: &str) -> Vec<i32> {
    let mut results = vec![];

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let input_sequence = ip.split_whitespace().map(|x| x.parse::<i32>().unwrap()).rev().collect();
                results.push(predict_next(input_sequence));
            }
        }
    }
    else {
        println!("File not found");
    }

    return results;
}

pub fn run() {
    let result = process_input("inputs/day9/input.txt");
    println!("{}", result.iter().sum::<i32>());
}