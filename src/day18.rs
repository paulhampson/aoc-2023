use num::abs;
use crate::day18::Direction::{Down, Left, Right, Up};
use crate::read_lines::read_lines;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn from_char(c: char) -> Direction {
        //0 means R, 1 means D, 2 means L, and 3 means U.
        match c {
            'U' | '3'  => Up,
            'D' | '1' => Down,
            'L' | '2' => Left,
            'R' | '0' => Right,
            _ => panic!("Invalid character for direction")
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DigInstruction {
    d: Direction,
    step_count: i64,
    colour_str: String
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Point(i64, i64);

fn generate_boundary_point_list(instructions: Vec<DigInstruction>) -> (Vec<Point>, i64) {
    let mut current_position = Point(0,0);
    let mut point_list = vec![current_position.clone()];
    let mut total_steps = 0;

    for instruction in instructions {
        let Point(mut current_x, mut current_y) = current_position;

        match instruction.d {
            Up => current_y = current_y - instruction.step_count,
            Down => current_y = current_y + instruction.step_count,
            Left => current_x = current_x - instruction.step_count,
            Right => current_x = current_x + instruction.step_count
        }

        total_steps += instruction.step_count;
        current_position = Point(current_x, current_y);
        point_list.push(current_position.clone());
    }

    (point_list, total_steps)
}

fn get_interior_point_count(boundary_points: &Vec<Point>) -> (i64,i64) {

    // Calculate area using shoelace
    let number_of_points = boundary_points.iter().count()-1; // ignore very last as it closes loop
    let mut sum1 = 0;
    let mut sum2 = 0;

    for idx in 0..number_of_points-1 {
        sum1 = sum1 + (boundary_points[idx].0 as i64 * boundary_points[idx+1].1 as i64);
        sum2 = sum2 + (boundary_points[idx].1 as i64 * boundary_points[idx+1].0 as i64);
    }

    // Link back to the start
    sum1 = sum1 + (boundary_points[number_of_points-1].0 * boundary_points[0].1) as i64;
    sum2 = sum2 + (boundary_points[0].0 * boundary_points[number_of_points-1].1) as i64;

    let area_of_polygon = abs(sum1 - sum2) / 2;
    dbg!(area_of_polygon);
    return (area_of_polygon - ((number_of_points as i64)/2) + 1 , area_of_polygon);
}


fn parse_input(filename: &str) -> Vec<DigInstruction> {
    let mut dig_instructions = vec![];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut string_parts = ip.split_ascii_whitespace();
                dig_instructions.push(DigInstruction {
                    d: Direction::from_char(string_parts.next().unwrap().chars().next().unwrap()),
                    step_count: string_parts.next().unwrap().parse().unwrap(),
                    colour_str: string_parts.next().unwrap().to_string()
                });
            }
        }
    }
    else {
        println!("File not read");
    }
    dig_instructions
}

fn part_b_get_instructions(di: &Vec<DigInstruction>) -> Vec<DigInstruction> {
    let mut new_instructions = vec![];

    for original_instruction in di {
        let hex_code = original_instruction.colour_str.trim_start_matches("(#").trim_end_matches(")");
        let (steps_str, direction_str) = hex_code.split_at(5);

        new_instructions.push(DigInstruction {
            d: Direction::from_char(direction_str.chars().next().unwrap()),
            step_count: i64::from_str_radix(steps_str, 16).unwrap(),
            colour_str: "".parse().unwrap()
        });
    }

    new_instructions
}

pub fn run() {
    println!("Day 18 Part A");
    let input_filename = "inputs/day18/input.txt";

    let dig_instructions = parse_input(input_filename);
    let (boundary_points, perimeter_length) = generate_boundary_point_list(dig_instructions.clone());
    let (_, area) = get_interior_point_count(&boundary_points);

    dbg!(area + (perimeter_length/2) as i64 + 1);

    println!("Day 18 Part B");
    let part_b_instructions = part_b_get_instructions(&dig_instructions);
    let (part_b_boundary_points, part_b_perimeter_length) = generate_boundary_point_list(part_b_instructions);
    let (_, part_b_area) = get_interior_point_count(&part_b_boundary_points);

    dbg!(part_b_area + (part_b_perimeter_length/2) as i64 + 1);
}