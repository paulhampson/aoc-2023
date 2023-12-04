use std::cmp::max;
use regex::Regex;
use crate::read_lines::read_lines;

fn get_ball_count(round: &str, colour: &str ) -> i32
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

fn parse_game(game: String, _max_red: i32, _max_green: i32, _max_blue: i32) -> (i32, bool, i32)
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

    // let is_game_valid = red_count <= _max_red && green_count <= _max_green && blue_count <= _max_blue;
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

        // let is_game_valid = red_count <= _max_red && green_count <= _max_green && blue_count <= _max_blue;
        // if !is_game_valid {
        //     return (game_id, is_game_valid)
        // }
    }

    let power = min_red_required * min_green_required * min_blue_required;

    return (game_id, true, power);
}

pub(crate) fn run() {
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