mod day1;
mod read_lines;
mod day3;
mod day2;
mod day4;
mod day5;
mod day6;
mod day7;
mod day7b;
mod day8;
mod day8b;
mod day9;
mod day10;
mod day11;
mod day14;
mod day12;
mod day13;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day21;
mod day22;
mod day23;


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
        "day4" => day4::run(),
        "day4b" => day4::run_part_b(),
        "day5" => day5::run(),
        "day5b" => day5::run_part_b(),
        "day6" => day6::run(),
        "day7" => day7::run(),
        "day7b" => day7b::run(),
        "day8" => day8::run(),
        "day8b" => day8b::run(),
        "day9" => day9::run(),
        "day10" => day10::run(),
        "day11" => day11::run(),
        "day12" => day12::run(), // not completed
        "day13" => day13::run(),
        "day14" => day14::run(),
        "day15" => day15::run(),
        "day16" => day16::run(),
        "day17" => day17::run(),
        "day18" => day18::run(),
        "day19" => day19::run(), // part A only
        "day20" => todo!(),
        "day21" => day21::run(),
        "day22" => day22::run(), // incomplete
        "day23" | _ => day23::run(),
    }
}



