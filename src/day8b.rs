use std::collections::HashMap;
use num::Integer;
use regex::Regex;
use crate::read_lines::read_lines;

fn read_graph(filename: &str) -> HashMap<String, (String, String)> {
    let mut input_graph = HashMap::new();
    let node_string_re = Regex::new(r"^(?<node_name>[0-9A-Z]{3}) = \((?<left_node>[0-9A-Z]{3}), (?<right_node>[0-9A-Z]{3})\)$").unwrap();

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



fn count_steps(graph: HashMap<String, (String, String)>, instructions: String) -> Vec<i64> {
    let mut all_step_counts = vec![];


    // Find all the starting nodes - ones that end with A
    let starting_nodes:Vec<&String> = graph.keys().filter(|n| n.chars().nth_back(0).unwrap() == 'A').collect();
    println!("starting_nodes = {:?}", starting_nodes);

    // Find how many steps for each starting node to a '**Z' node
    for starting_node in starting_nodes.iter() {
        let mut next_node = starting_node.to_string();
        let mut step_count = 0;
        println!("processing starting node {}", next_node);

        while next_node.chars().nth_back(0).unwrap() != 'Z' {
            for instruction in instructions.chars() {
                let node_idx = next_node.as_str();
                match instruction {
                    'L' => {
                        let (left_edge, _) = graph.get(node_idx).unwrap().clone();
                        next_node = left_edge;
                    },
                    'R' => {
                        let (_, right_edge) = graph.get(node_idx).unwrap().clone();
                        next_node = right_edge;
                    },
                    _ => assert!(false, "Invalid navigation instruction")
                }

                step_count += 1;

                // check nodes don't equal the stop condition
                if next_node.chars().nth_back(0).unwrap() == 'Z' {
                    all_step_counts.push(step_count);
                    break;
                }
            }
        }
    }

    return all_step_counts;
}

fn vector_lcm(v: Vec<i64>) -> i64 {
    let mut result = v.first().unwrap().clone();
    for n in v.iter().skip(1) {
        result = result.lcm(n).clone();
    }

    return result;
}

pub fn run() {
    let input_file = "inputs/day8/input.txt";
    let input_graph = read_graph(input_file);
    let input_nav_instructions = read_navigation_instructions(input_file);
    let step_counts = count_steps(input_graph, input_nav_instructions);

    println!("Find the least common multiple of {:?}", step_counts);
    println!("LCM = {}", vector_lcm(step_counts));
}



