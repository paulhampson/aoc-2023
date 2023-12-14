use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use array2d::Array2D;
use crate::read_lines::read_lines;

#[derive(Hash, Clone, Eq, PartialEq)]
enum MapEntry {
    Rock,
    Ash
}

impl MapEntry {
    fn from_char(c: char) -> MapEntry {
        match c {
            '#'=> MapEntry::Rock,
            '.'=> MapEntry::Ash,
            _ => panic!("Invalid character for map entry")
        }
    }
}

type Note = Array2D<MapEntry>;

fn get_line_hash(l: &Vec<MapEntry>) -> u64 {
    let mut hasher = DefaultHasher::new();
    l.hash(&mut hasher);
    hasher.finish()
}

fn is_reflection_perfect(line_hashes: &Vec<u64>, position: usize) -> bool {
    let mut a = position as isize;
    let mut b = position + 1;
    let mut is_matched = true;

    while (a >= 0) && (b < line_hashes.len()) {
        is_matched = is_matched && (line_hashes[a as usize] == line_hashes[b]);

        a -= 1;
        b += 1;
    }

    is_matched
}

fn get_line_difference_count(line_a: &Vec<MapEntry>, line_b: &Vec<MapEntry>) -> i32
{
    let mut line_difference_count = 0;

    // count differences
    for (idx, element_from_a) in line_a.iter().enumerate() {
        let element_from_b = line_b.get(idx).unwrap();
        if element_from_a != element_from_b {
            line_difference_count += 1;
        }
    }

    line_difference_count
}

fn is_reflection_with_single_smudge(lines: &Vec<Vec<MapEntry>>, line_hashes: &Vec<u64>,  position: usize) -> bool
{
    let mut a = position as isize;
    let mut b = position + 1;
    let mut smudge_count = 0;

    while (a >= 0) && (b < lines.len()) {
        if line_hashes[a as usize] != line_hashes[b] {
            smudge_count += get_line_difference_count(lines.get(a as usize).unwrap(), lines.get(b).unwrap());
        }

        a -= 1;
        b += 1;
    }

    smudge_count == 1
}

fn find_reflection(lines: Vec<Vec<MapEntry>>) -> (Option<usize>, Option<usize>) {
    let mut line_hashes = vec![];
    let mut reflection_positions = vec![];
    line_hashes.push(get_line_hash(lines.first().unwrap()));

    let mut perfect_reflection_pos = None;
    let mut smudged_reflection_pos = None;

    for (position, line) in lines.iter().skip(1).enumerate() {
        let line_hash = get_line_hash(line);

        if line_hash == *line_hashes.last().unwrap() ||
            get_line_difference_count(line, lines.get(position).unwrap()) == 1 {
            reflection_positions.push(position);
        }
        line_hashes.push(line_hash);

    }

    for reflection_position in reflection_positions.clone() {
        if is_reflection_perfect(&line_hashes, reflection_position) {
            perfect_reflection_pos = Some(reflection_position);
            break;
        }
    }

    for reflection_position in reflection_positions {
        if is_reflection_with_single_smudge(&lines, &line_hashes, reflection_position) {
            smudged_reflection_pos = Some(reflection_position);
            break;
        }
    }

    (perfect_reflection_pos, smudged_reflection_pos)
}

fn parse_input(filename: &str) -> (usize, usize) {
    let mut summary_a = 0;
    let mut summary_b = 0;

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut note_vector = vec![];
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() || ip == "END" {
                    let note = Note::from_rows(&*note_vector).unwrap();

                    let (vertical_reflection_pos_a, vertical_reflection_pos_b) = find_reflection(note.as_columns());
                    let (horizontal_reflection_pos_a, horizontal_reflection_pos_b) = find_reflection(note.as_rows());

                    if vertical_reflection_pos_a.is_some() {
                        summary_a += vertical_reflection_pos_a.unwrap() + 1;
                    }
                    if horizontal_reflection_pos_a.is_some() {
                        summary_a += 100 * (horizontal_reflection_pos_a.unwrap() + 1);
                    }

                    if vertical_reflection_pos_b.is_some() {
                        summary_b += vertical_reflection_pos_b.unwrap() + 1;
                    }
                    if horizontal_reflection_pos_b.is_some() {
                        summary_b += 100 * (horizontal_reflection_pos_b.unwrap() + 1);
                    }

                    // next note - start fresh
                    note_vector.clear();
                } else {
                    let mut map_line = vec![];
                    for c in ip.chars() {
                        map_line.push(MapEntry::from_char(c));
                    }
                    note_vector.push(map_line);
                }
            }
        }
    }

    return (summary_a, summary_b);
}

pub fn run() {
    println!("Day 13 Part A");
    let input_filename = "inputs/day13/input.txt";

    dbg!(parse_input(input_filename));
}