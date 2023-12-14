use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use array2d::Array2D;
use Direction::{East, North, South, West};
use crate::day14::CellType::{CubeRock, EmptyGround, RoundRock};
use crate::read_lines::read_lines;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum CellType {
    RoundRock,
    CubeRock,
    EmptyGround
}

impl CellType {
    fn from_char(c: char) -> CellType {
        match c {
            'O' => RoundRock,
            '#' => CubeRock,
            '.' => EmptyGround,
            _ => panic!("Invalid character in input")
        }
    }
    fn to_char(&self) -> char {
        match self {
            RoundRock => 'O',
            CubeRock => '#',
            EmptyGround => '.'
        }
    }
}

type PlatformMap = Array2D<CellType>;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Copy, Clone)]
struct CycleIdentifier {
    cycle_number: i64,
    direction: Direction
}

fn tilt_platform(platform_map: PlatformMap, direction: &Direction) -> PlatformMap {
    let mut tilted_platform_map_vec = vec![];

    // going north - we want the table in columns
    let lines = match direction {
        North | South => platform_map.as_columns(),
        East  | West  => platform_map.as_rows()
    };

    for mut line in lines {
        if *direction == South || *direction == East {
            line.reverse();
        }
        let line_len = line.iter().count();
        // entry 0 is at the edge we are travelling towards so it can't move
        for i in 1..line_len {
            // move cell as far as it can go if it is a round rock
            if line[i] == RoundRock {
                for j in (1..i+1).rev() {
                    if line[j-1] == CubeRock {
                        // cube rock is the end of the line - we can break out early
                        break;
                    }
                    if line[j-1] == EmptyGround {
                        line[j-1] = RoundRock;
                        line[j] = EmptyGround;
                    }
                }
            }
        }

        if *direction == South || *direction == East {
            line.reverse();
        }

        tilted_platform_map_vec.push(line);
    }

    match direction {
        North | South => PlatformMap::from_columns(&*tilted_platform_map_vec).unwrap(),
        East  | West  => PlatformMap::from_rows(&*tilted_platform_map_vec).unwrap()
    }
}

fn get_total_force(platform_map: PlatformMap) -> i32 {
    let mut total_force = 0;

    for column in platform_map.columns_iter() {
        for (location, cell) in column.rev().enumerate() {
            if *cell == RoundRock {
                total_force += location as i32 + 1;
            }
        }
    }

    total_force
}

fn parse_input(filename: &str) -> PlatformMap {
    let mut platform_vector = vec![];

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut row = vec![];
                for c in ip.chars() {
                    row.push(CellType::from_char(c));
                }
                platform_vector.push(row);
            }
        }
    }

    Array2D::from_rows(&*platform_vector).unwrap()
}

fn print_map(platform_map: &PlatformMap) {
    for row in platform_map.rows_iter() {
        for cell in row {
            print!("{}", cell.to_char());
        }
        println!();
    }
}

pub fn run() {
    println!("Day 14 Part B");
    let input_filename = "inputs/day14/input.txt";
    let target_cycles:i64 = 1000000000;

    let platform_map = parse_input(input_filename);

    let mut tilted_map = platform_map;
    let mut tilt_hash_map:HashMap<u64, CycleIdentifier> = HashMap::new();
    let mut loop_found = false;
    let mut loop_start:CycleIdentifier = CycleIdentifier { cycle_number: 0, direction: Direction::North };
    let mut loop_end:CycleIdentifier = CycleIdentifier { cycle_number: 0, direction: Direction::North };

    for cycle in 0..target_cycles {
        // println!("*** Cycle {}", cycle);
        let spin_cycle = [North, West, South, East];
        for d in spin_cycle {
            tilted_map = tilt_platform(tilted_map, &d);

            let mut hasher = DefaultHasher::new();
            tilted_map.hash(&mut hasher);
            let hash = hasher.finish();
            if tilt_hash_map.contains_key(&hash){
                loop_start = tilt_hash_map[&hash];
                loop_end = CycleIdentifier{
                    cycle_number: cycle,
                    direction: d
                };

                let start_cycle = loop_start.cycle_number;
                let start_direction = &loop_start.direction;
                println!("Found matching loop start is at cycle {}, direction {:?}, loop_end is at cycle {}, direction {:?}",
                       start_cycle, start_direction, cycle, d);
                loop_found = true;
                break;
            } else {
                tilt_hash_map.insert(hash, CycleIdentifier {
                    cycle_number: cycle,
                    direction: d
                });
            }

            // println!("Direction => {:?}", d);
            // print_map(&tilted_map);
            // println!();
        }
        if loop_found {
            break;
        }
        // print_map(&tilted_map);
        // println!();
    }

    // Work out remaining cycles to calculate
    let spin_steps = [North, West, South, East];
    let total_tilts = target_cycles * 4;
    let tilts_to_loop_start = (loop_start.cycle_number * 4) + (spin_steps.iter().position(|&d| d == loop_start.direction).unwrap() as i64);
    let tilts_to_loop_end = (loop_end.cycle_number * 4) + (spin_steps.iter().position(|&d| d == loop_end.direction).unwrap() as i64);
    let tilts_in_loop =  tilts_to_loop_end - tilts_to_loop_start;

    let mut remainder_tilts = (total_tilts - tilts_to_loop_start) % tilts_in_loop;

    dbg!(tilts_to_loop_start);
    dbg!(tilts_in_loop);
    dbg!(remainder_tilts);

    let mut cycle_point = spin_steps.iter().position(|&d| d == loop_end.direction).unwrap();
    // do remaining cycles
    while remainder_tilts > 0 {
        while remainder_tilts > 0 && cycle_point < spin_steps.len() {
            let d = spin_steps[cycle_point];
            tilted_map = tilt_platform(tilted_map, &d);
            cycle_point += 1;
            remainder_tilts -= 1;
        }
        cycle_point = 0;
    }

    dbg!(get_total_force(tilted_map));
}