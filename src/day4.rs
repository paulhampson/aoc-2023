use std::collections::HashMap;
use regex::Regex;
use crate::read_lines::read_lines;

fn convert_number_capture_to_vector(s: &str) -> Vec<i32> {
    return s.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
}

fn parse_game_card(card_string: String) -> (i32, u32) {
    let mut points;

    let card_re = Regex::new(r"(Card\s*[0-9]+:)\s*(?<winning_numbers>([0-9]+\s*)+)\|\s+(?<scratch_numbers>([0-9]+\s*)+)").unwrap();

    let Some(captures) = card_re.captures(card_string.as_str()) else { return (0,0) };
    let winning_numbers_string = &captures["winning_numbers"];
    let scratch_numbers_string = &captures["scratch_numbers"];

    // convert capture numbers to array of integers
    // compare the vectors to count matches
    dbg!(winning_numbers_string);
    dbg!(scratch_numbers_string);

    let winning_numbers = convert_number_capture_to_vector(winning_numbers_string);
    let scratch_numbers = convert_number_capture_to_vector(scratch_numbers_string);

    let mut match_count:u32 = 0;
    for n in winning_numbers.iter() {
        match_count += scratch_numbers.iter().filter(|&x| x == n).count() as u32;
    }

    dbg!(match_count);
    points = match match_count {
        0 => 0,
        _ => i32::pow(2, match_count - 1)
    };
    dbg!(points);
    return (points, match_count);
}

pub fn run() {
    println!("Day 4 part A");
    let mut point_total = 0;
    if let Ok(lines) = read_lines("./inputs/day4/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let (points, _) = parse_game_card(ip);
                point_total += points;
            }
        }
    }

    println!("{}", point_total);
}

pub fn run_part_b() {
    println!("Day 4 part B");
    let mut total_card_count = 0;
    let mut point_list = HashMap::new();
    point_list.insert(0, 1);

    if let Ok(lines) = read_lines("./inputs/day4/input.txt") {
        for (idx, line) in lines.enumerate() {
            if let Ok(ip) = line {
                let multiplier = match point_list.contains_key(&idx) {
                    true => point_list[&idx],
                    false => 1
                };

                let (_, win_count) = parse_game_card(ip);
                total_card_count += multiplier;

                for i in (idx+1)..(idx+(win_count as usize)+1) {
                    if point_list.contains_key(&i) {
                        point_list.insert(i, point_list[&i] + multiplier);
                    } else {
                        point_list.insert(i, 1+multiplier);
                    }
                }
            }
        }
    }

    println!("{}", total_card_count);
}