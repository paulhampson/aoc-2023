use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



fn main() {
    //day1();
    //day2();
    //day3();
    day3part_b();
}

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

fn day3() {
    println!("Day 3 part B");
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

fn day3part_b() {
    println!("Day 3");
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

fn get_ball_count( round: &str, colour: &str ) -> i32
{
    let mut ball_count = 0;
    let re_string = format!("([0-9]+) {}", colour);
    let ball_count_re = Regex::new(&re_string).unwrap();
    let capture = ball_count_re.captures(round);
    if !capture.is_none() {
        let (_, [ball_count_str]) = capture.unwrap().extract();
        ball_count = ball_count_str.parse().unwrap();
    }
    return ball_count;
}

fn parse_game(game: String, max_red: i32, max_green: i32, max_blue: i32) -> (i32, bool, i32)
{
    let mut red_count:i32;
    let mut green_count:i32;
    let mut blue_count:i32;
    let mut min_red_required = 0;
    let mut min_blue_required = 0;
    let mut min_green_required = 0;

    let mut game_phases = game.split(';').into_iter();

    let game_id_and_first_game = game_phases.next().unwrap();
    let game_re = Regex::new(r"Game ([0-9]+):.+$").unwrap();
    let (_, [game_id_str]) = game_re.captures(game_id_and_first_game).unwrap().extract();
    let game_id: i32 = game_id_str.parse().unwrap();

    red_count = get_ball_count(game_id_and_first_game, "red");
    min_red_required = max(min_red_required, red_count);
    green_count = get_ball_count(game_id_and_first_game, "green");
    min_green_required = max(min_green_required, green_count);
    blue_count = get_ball_count(game_id_and_first_game, "blue");
    min_blue_required = max(min_blue_required, blue_count);

    // let is_game_valid = red_count <= max_red && green_count <= max_green && blue_count <= max_blue;
    // if !is_game_valid {
    //     return (game_id, is_game_valid)
    // }

    for round in game_phases {
        red_count = get_ball_count(round, "red");
        min_red_required = max(min_red_required, red_count);
        green_count = get_ball_count(round, "green");
        min_green_required = max(min_green_required, green_count);
        blue_count = get_ball_count(round, "blue");
        min_blue_required = max(min_blue_required, blue_count);

        // let is_game_valid = red_count <= max_red && green_count <= max_green && blue_count <= max_blue;
        // if !is_game_valid {
        //     return (game_id, is_game_valid)
        // }
    }

    let power = min_red_required * min_green_required * min_blue_required;

    return (game_id, true, power);
}

fn day2() {
    println!("Day 2");
    let mut game_id_sum = 0;
    let mut power_sum = 0;
    if let Ok(lines) = read_lines("./inputs/day2/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let (game_id, valid, power) = parse_game(ip, 12, 13, 14);
                power_sum += power;
                if valid {
                    game_id_sum += game_id;
                }
            }
        }
    }
    println!("Valid Game ID Sum = {}", game_id_sum);
    println!("Power Sum = {}", power_sum);
}

fn digit_string_check( s: &str ) -> Result<char, char> {
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

fn day1() {
    println!("Day 1");
    let mut sum_of_numbers = 0;
    if let Ok(lines) = read_lines("../inputs/day1/day1.txt") {
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