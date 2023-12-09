use std::collections::HashMap;
use regex::Regex;
use crate::read_lines::read_lines;

fn read_graph(filename: &str) -> HashMap<String, (String, String)> {
    let mut input_graph = HashMap::new();
    let node_string_re = Regex::new(r"^(?<node_name>[A-Z]{3}) = \((?<left_node>[[A-Z]]{3}), (?<right_node>[A-Z]{3})\)$").unwrap();

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if let Some(captures) = node_string_re.captures(ip.as_str()) {
                    input_graph.insert(captures["node_name"].to_string(), (captures["left_node"].to_string(), captures["right_node"].to_string()));
                }
            }
        }
    }

    return input_graph;
}

fn read_navigation_instructions(filename: &str) -> String {
    if let Ok(mut lines) = read_lines(filename) {
        return lines.nth(0).unwrap().unwrap();
    }

    return "".to_string();
}

fn count_steps(graph: HashMap<String, (String, String)>, instructions: String) -> i32 {
    let mut step_count = 0;
    let mut next_instruction = "AAA".to_string();

    while next_instruction != "ZZZ" {
        for instruction in instructions.chars() {
            let node_idx = next_instruction.as_str();
            match instruction {
                'L' => {
                    let (left_edge, _) = graph.get(node_idx).unwrap().clone();
                    next_instruction = left_edge.clone();
                },
                'R' => {
                    let (_, right_edge) = graph.get(node_idx).unwrap();
                    next_instruction = right_edge.clone();
                },
                _ => assert!(false, "Invalid navigation instruction")
            }
            step_count += 1;
            if next_instruction == "ZZZ" {
                break;
            }
        }
    }

    return step_count;
}

pub fn run() {
    let input_file = "inputs/day8/input.txt";
    let input_graph = read_graph(input_file);
    let input_nav_instructions = read_navigation_instructions(input_file);
    let step_count = count_steps(input_graph, input_nav_instructions);

    println!("{}", step_count);
}



