use std::collections::HashMap;
use indexmap::IndexMap;
use crate::read_lines::read_lines;

struct AocHash {
    _state: u32
}

impl AocHash {

    fn new() -> Self {
        Self { _state: 0 }
    }
    fn initialise(&mut self) {
        self._state = 0;
    }

    fn hash_char(&mut self, c: &char) {
        self._state += c.clone() as u32;
        self._state *= 17;
        self._state %= 256;
    }

    fn hash_str(&mut self, s: &str) {
        for c in s.chars() {
            self.hash_char(&c);
        }
    }

    fn finish(&self) -> u8 {
        self._state as u8
    }
}


fn parse_input_a(filename: &str) -> i32 {
    let mut hashes = vec![];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let items_to_hash = ip.split(',');
                for item in items_to_hash {
                    let mut aoc_hash = AocHash::new();
                    aoc_hash.initialise();
                    aoc_hash.hash_str(item);
                    hashes.push(aoc_hash.finish() as i32);
                }
            }
        }
    }
    hashes.iter().sum()
}

struct Lens {
    label: String,
    focal_length: i32
}

fn parse_input_b(filename: &str) -> HashMap<u8, IndexMap<String, u32>> {
    let mut boxes:HashMap<u8, IndexMap<String, u32>> = HashMap::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let items_to_hash = ip.split(',');
                for item in items_to_hash {
                    let mut label = item.to_string();
                    label.retain(|c| !(c == '-' || c == '=' || c.is_numeric()));
                    dbg!(&label);
                    let mut aoc_hash = AocHash::new();
                    aoc_hash.initialise();
                    aoc_hash.hash_str(label.as_str());
                    let box_id = aoc_hash.finish();
                    dbg!(&box_id);
                    let last_char = item.chars().last().unwrap();

                    if let Some(b) = boxes.get_mut(&box_id) {
                        // do action on box
                        println!("existing box {}", box_id);
                        match last_char {
                            '-' => {
                                println!("remove {} from box", label);
                                b.shift_remove(&label);
                            },
                            '0'..='9' => {
                                println!("add/replace focal length {} of lens {} to box", last_char, label);
                                b.insert(label, last_char.to_digit(10).unwrap());
                            },
                            _ => panic!("Unknown command character")
                        }
                    } else {
                        match last_char {
                            '-' => (), // box doesn't exist so can't remove items
                            '0'..='9' => {
                                let mut new_box = IndexMap::new();
                                new_box.insert(label.clone(), last_char.to_digit(10).unwrap());
                                boxes.insert(box_id, new_box);
                                println!("add/replace focal length {} of lens {} to box", last_char, label)
                            },
                            _ => panic!("Unknown command character")
                        }
                    }
                }
            }
        }
    }
    boxes
}

fn calculate_total_power(boxes: HashMap<u8, IndexMap<String, u32>>) -> u32 {
    // Product:
    //     One plus the box number of the lens in question.
    //     The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
    //     The focal length of the lens.
    let mut total_strength = 0;

    for (&box_id, box_contents) in boxes.iter() {
        for (slot_idx, (_, focal_length)) in box_contents.iter().enumerate() {
            total_strength += (box_id as u32 + 1) * (slot_idx as u32 + 1) * focal_length;
        }
    }

    total_strength
}

pub fn run() {
    println!("Day 15 Part A");
    let input_filename = "inputs/day15/input.txt";

    //dbg!(parse_input_a(input_filename));
    let boxes = parse_input_b(input_filename);
    dbg!(calculate_total_power(boxes));
}