use itertools::Itertools;
use crate::day12::SpringStatus::{Damaged, Operational, Unknown};
use crate::read_lines::read_lines;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
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
type DamageReport = Vec<i32>;

#[derive(Debug, Eq, PartialEq, Clone)]
struct SpringRecord {
    damage_map: DamageMap,
    damage_report: DamageReport
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

fn calculate_report(damage_map: &DamageMap) -> DamageReport
{
    damage_map.split(|&c| c == Operational).map(|area| area.len() as i32).filter(|&n| n != 0).collect::<DamageReport>()
}

fn create_hypothetical_map_from_collapsed_unknowns(collapsed_unknowns: Vec<&SpringStatus>, damage_map: &DamageMap) -> DamageMap
{
    let mut hypothetical_damage_map = damage_map.clone();
    for &entry in collapsed_unknowns {
        let insertion_position = hypothetical_damage_map.iter().position(|&x| x == Unknown).unwrap();
        hypothetical_damage_map[insertion_position] = entry;
    }

    hypothetical_damage_map
}

fn find_possible_arrangements(spring_record: &SpringRecord) -> i32
{
    let mut possible_count = 0;

    // find total number of damaged areas to place
    let total_damaged_cells:i32 = spring_record.damage_report.iter().sum();
    let inplace_damaged_cells = spring_record.damage_map.iter().filter(|&&c| c == Damaged).count() as i32;
    let number_unknown_cells = spring_record.damage_map.iter().filter(|&&c| c == Unknown).count() as i32;
    let damaged_cells_to_place = total_damaged_cells - inplace_damaged_cells;


    let mut possible_options = vec![Damaged; damaged_cells_to_place as usize];
    possible_options.extend(vec![Operational; (number_unknown_cells-damaged_cells_to_place) as usize]);

    dbg!(number_unknown_cells);

    for combination in possible_options.iter().permutations(number_unknown_cells as usize).unique() {
        let hypothetical_map = create_hypothetical_map_from_collapsed_unknowns(combination, &spring_record.damage_map);
        let hypothetical_report = calculate_report(&hypothetical_map);

        if hypothetical_report == spring_record.damage_report {
            possible_count += 1;
        }
    }


    possible_count
}

pub fn run() {
    println!("Day 12 Part A");
    let input_filename = "inputs/day12/input.txt";

    let spring_records = parse_input(input_filename);
    let mut possible_arrangement_counts = vec![];
    for record in spring_records.iter() {
        possible_arrangement_counts.push(find_possible_arrangements(record));
    }
    println!("Answer = {}", possible_arrangement_counts.iter().sum::<i32>());

}