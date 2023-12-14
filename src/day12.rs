use crate::day12::SpringStatus::{Damaged, Operational, Unknown};
use crate::read_lines::read_lines;

#[derive(Debug)]
enum SpringStatus {
    Operational,
    Damaged,
    Unknown
}

impl SpringStatus {
    fn from_char(c: char) -> SpringStatus {
        match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => panic!("Invalid spring status character")
        }
    }
}

type DamageMap = Vec<SpringStatus>;

#[derive(Debug)]
struct SpringRecord {
    damage_map: DamageMap,
    damage_report: Vec<i32>
}

type UnsatisfiedRegion = SpringRecord;

fn parse_input(filename: &str) -> Vec<SpringRecord> {
    let mut spring_records = vec![];

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let split_input:Vec<&str> = ip.split_whitespace().collect();

                // handle the symbolic map
                let mut damage_map = DamageMap::new();
                for c in split_input[0].chars() {
                    damage_map.push(SpringStatus::from_char(c));
                }

                // handle the numeric damage description
                let damage_report = split_input[1].split(",").map(|x| x.parse::<i32>().unwrap()).collect();

                spring_records.push( SpringRecord{
                    damage_map,
                    damage_report
                })
            }
        }
    }

    return spring_records;
}

fn find_unsatisfied_map_regions_and_associated_record(record: SpringRecord) -> Vec<UnsatisfiedRegion>
{
    let unsatisfied_regions = vec![];

    let full_damage_map = record.damage_map;
    for s in full_damage_map {
        dbg!(s);
    }

    return unsatisfied_regions;
}


pub fn run() {
    println!("Day 12 Part A");
    let input_filename = "inputs/day12/test.txt";

    let spring_records = parse_input(input_filename);
    dbg!(spring_records);
}