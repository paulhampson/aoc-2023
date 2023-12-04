use std::cmp::{max, min};
use regex::Regex;
use crate::read_lines::read_lines;

fn is_symbol(c: char) -> bool
{
    return !(c.is_digit(10) || c == '.');
}

fn check_input(target_line: &str, line_prior: Option<&String>, line_following: Option<&String>) -> Vec<i32>
{
    let number_re = Regex::new(r".*?(?<number>[0-9]+).*?").unwrap();
    let mut found_parts:Vec<i32> = vec![];
    for capture in number_re.captures_iter(target_line) {
        let number = &capture["number"];
        let start_index = capture.get(1).unwrap().start();
        let end_index = capture.get(1).unwrap().end();
        let mut has_adjacent_symbol = false;

        println!("number={}, start_index={}, end_index={}, target_line={}",number,start_index,end_index,target_line);

        if line_prior.is_some() {
            let start_point = match start_index {
                0 => 0,
                _ => start_index - 1
            };

            let mut end_point = end_index + 1;
            if end_index == line_prior.unwrap().len() {
                end_point = end_index;
            }

            let area_of_interest = &line_prior.unwrap()[start_point..end_point];
            for c in area_of_interest.chars() {
                if has_adjacent_symbol {
                    break;
                }
                has_adjacent_symbol = is_symbol(c);

            }
        }

        if line_following.is_some() {
            let start_point = match start_index {
                0 => 0,
                _ => start_index - 1
            };
            let mut end_point = end_index + 1;
            if end_index == line_following.unwrap().len() {
                end_point = end_index;
            }

            let area_of_interest = &line_following.unwrap()[start_point..end_point];
            for c in area_of_interest.chars() {
                if has_adjacent_symbol {
                    break;
                }
                has_adjacent_symbol = is_symbol(c);
            }
        }

        if start_index > 0 {
            if is_symbol(target_line.chars().nth(start_index-1).unwrap()) {
                has_adjacent_symbol = true;
            }
        }

        if end_index < target_line.len() {
            if is_symbol(target_line.chars().nth(end_index).unwrap()) {
                has_adjacent_symbol = true;
            }
        }

        if has_adjacent_symbol {
            found_parts.push(number.parse::<i32>().unwrap());
        }
    }

    return found_parts;
}

pub fn run() {
    println!("Day 3 part A");
    let mut input_data: Vec<String> = vec![];
    if let Ok(lines) = read_lines("./inputs/day3/test.txt") {
        for line in lines {
            if let Ok(ip) = line {
                input_data.push(ip);
            }
        }
    }

    // find position and length of number in the line
    // check the characters before and after it for symbols (indices A and B)
    // check the characters on the line above and below that are between the same indices as A and B
    let mut all_found_parts: Vec<i32> = vec![];
    for (idx, line) in input_data.iter().enumerate() {
        let line_prior = match idx {
            0 => None,
            _ => input_data.get(idx-1)
        };
        let line_following = input_data.get(idx+1);
        let mut found_parts = check_input(&line, line_prior, line_following);
        all_found_parts.append(&mut found_parts);
    }

    let sum_of_parts:i32 = all_found_parts.iter().sum();
    println!("{}",sum_of_parts);
}

fn extract_adjacent_numbers(line: &str, star_index:usize) -> Vec<i32> {
    let mut gear_numbers:Vec<i32> = vec![];
    let re_number = Regex::new(r".*?([0-9]+).*?").unwrap();

    let s = line;
    let start_point = max(0, star_index - 3);
    let end_point = min(s.len(), star_index + 4);

    let area_of_interest = &s[start_point..end_point];
    println!("{}", area_of_interest);

    // pull numbers from line following, then need to check it's touching the star_index
    for number_capture in re_number.captures_iter(area_of_interest) {
        let number_str = number_capture.get(1).unwrap().as_str();
        let number_start = number_capture.get(1).unwrap().start();
        let number_len = number_capture.get(1).unwrap().len();

        // ...*...
        // .12345.
        let mut start_check:Option<usize> = None;
        match number_len {
            1 => start_check = Some(2),
            2 => start_check = Some(1),
            3 => start_check = Some(0),
            _ => ()
        }

        if start_check.is_some() {
            if number_start >= start_check.unwrap() && number_start <= 4 {
                gear_numbers.push(number_str.parse::<i32>().unwrap());
            }
        }
    }

    return gear_numbers;
}

fn check_for_gears(target_line: &String, line_prior: Option<&String>, line_following: Option<&String>) -> Vec<i32> {
    let mut gear_ratios:Vec<i32> = vec![];
    let re_star = Regex::new(r".*?(\*).*?").unwrap();


    // find *'s in the target line and check for surrounding numbers
    for capture in re_star.captures_iter(target_line) {
        let star_index = capture.get(1).unwrap().start();
        let mut gear_numbers:Vec<i32> = vec![];

        // look for number in line prior
        if line_prior.is_some() {
            gear_numbers.extend(extract_adjacent_numbers(line_prior.unwrap(), star_index).iter());
        }

        // look for number in target line
        gear_numbers.extend(extract_adjacent_numbers(target_line, star_index).iter());

        // look for number line following
        if line_following.is_some() {
            gear_numbers.extend(extract_adjacent_numbers(line_following.unwrap(), star_index).iter());
        }

        assert!(gear_numbers.len() <= 2, "More than 2 gear numbers found!");
        match gear_numbers.len() {
            2 => {
                print!("gears = ");
                for gear in gear_numbers.iter() {
                    print!("{} ",gear);
                }
                println!();

                gear_ratios.push(gear_numbers.iter().product());
                let product:i32 = gear_numbers.iter().product();
                println!("gear ratio {}", product);
            },
            _ => ()
        }

    }

    println!("--");

    return gear_ratios;
}

pub fn run_partb() {
    println!("Day 3 part B");
    let mut input_data: Vec<String> = vec![];
    if let Ok(lines) = read_lines("./inputs/day3/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                input_data.push(ip);
            }
        }
    }

    // find position and length of number in the line
    // check the characters before and after it for symbols (indices A and B)
    // check the characters on the line above and below that are between the same indices as A and B
    let mut all_found_gears: Vec<i32> = vec![];
    for (idx, line) in input_data.iter().enumerate() {
        let line_prior = match idx {
            0 => None,
            _ => input_data.get(idx-1)
        };
        let line_following = input_data.get(idx+1);
        let found_gears = check_for_gears(line, line_prior, line_following);
        all_found_gears.extend(found_gears);
    }

    let sum_of_gears:i32 = all_found_gears.iter().sum();
    println!("{}",sum_of_gears);
}

