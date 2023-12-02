use std::cmp::max;
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

fn main() {
    //day1();
    day2();
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