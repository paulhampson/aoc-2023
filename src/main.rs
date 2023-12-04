mod day1;
mod read_lines;
mod day3;
mod day2;
mod day4;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_day = match args.len() {
        2 => args[1].as_str(),
        _ => ""
    };

    match target_day {
        "day1" => day1::run(),
        "day2" => day2::run(),
        "day3" => day3::run(),
        "day3b" => day3::run_partb(),
        _ => day4::run()
    }
}



