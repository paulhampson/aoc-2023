use crate::read_lines::read_lines;

fn digit_string_check(s: &str ) -> Result<char, char> {
    let check_items = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ];
    for (check_string, value) in check_items {
        if s.starts_with(check_string) {
            return Ok(value);
        }
    }
    return Err('X');
}

pub fn run() {
    println!("Day 1");
    let mut sum_of_numbers = 0;
    if let Ok(lines) = read_lines("./inputs/day1/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let mut digits_in_string = String::new();
            if let Ok(ip) = line {
                for (i, c) in ip.chars().enumerate() {
                    let (_, remaining_string) = ip.split_at(i);

                    // check for either numeric digits or string digits
                    if c.is_digit(10) {
                        digits_in_string.push(c);
                    } else if let Ok(digit) = digit_string_check(remaining_string) {
                        digits_in_string.push(digit);
                    }
                }
            }

            // now get the first and last digit and sum
            let first_digit = digits_in_string.chars().nth(0).unwrap();
            let last_digit = digits_in_string.chars().nth_back(0).unwrap();
            let mut number_from_string = first_digit.to_string();
            number_from_string.push(last_digit);
            let n: i32 = number_from_string.parse().unwrap();
            sum_of_numbers += n;
        }
    }
    println!("{}", sum_of_numbers);
}