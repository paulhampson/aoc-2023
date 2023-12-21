use std::collections::HashMap;
use array2d::Array2D;
use itertools::Itertools;
use crate::day21::MapLocationState::{GardenPlot, Rocks, StartingPosition};
use crate::read_lines::read_lines;

#[derive(Debug, Clone, Eq, PartialEq)]
enum MapLocationState {
    StartingPosition,
    GardenPlot,
    Rocks
}

impl MapLocationState {
    fn from_character(c: &char) -> MapLocationState {
        match c {
            'S' => StartingPosition,
            '.' => GardenPlot,
            '#' => Rocks,
            _ => panic!("Invalid location state")
        }
    }
}

type GardenPlotMap = Array2D<MapLocationState>;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Pos(i32, i32);

fn parse_input(filename: &str) -> (Array2D<MapLocationState>, Pos) {
    let mut map_as_vec = vec![];
    let mut start_pos = Pos(0, 0);
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut row_idx = 0;
        for line in lines {
            let mut row = vec![];
            if let Ok(ip) = line {
                for (col_idx, c) in ip.chars().enumerate() {
                    let m = MapLocationState::from_character(&c);
                    if m == StartingPosition {
                        start_pos = Pos(row_idx, col_idx as i32);
                    }
                    row.push(m);
                }
            }
            map_as_vec.push(row);
            row_idx += 1;
        }
    }
    else {
        println!("File not read");
    }

    (GardenPlotMap::from_rows(&*map_as_vec).unwrap(), start_pos)
}

fn get_next_steps(current_position: &Pos, map: &GardenPlotMap) -> Vec<Pos> {
    let mut next_steps = vec![];
    let direction_deltas = vec![(0,1), (0,-1), (1,0), (-1,0)];

    for delta in direction_deltas {
        let new_pos = Pos(current_position.0 + delta.0, current_position.1 + delta.1);
        let Pos(row, col) = new_pos;
        if let Some(map_point) = map.get(row as usize, col as usize) {
            match map_point {
                GardenPlot | StartingPosition => next_steps.push(new_pos),
                _ => ()
            };
        }
    }

    return next_steps;
}

fn find_positions_after_steps(map: &GardenPlotMap, start: &Pos, steps: usize) -> usize {
    let mut seen_positions:HashMap<Pos, Vec<Pos>> = HashMap::new();

    let mut next_positions = get_next_steps(&start, map);
    seen_positions.insert(start.clone(), next_positions.clone());
    for step in 1..steps {
        println!("Processing step {}",step);

        let mut next_layer_positions:Vec<Pos> = vec![];
        for next_pos in next_positions {
            if let Some(next_steps) = seen_positions.get(&next_pos) {
                next_layer_positions.extend(next_steps.clone());
            } else {
                let next_steps = get_next_steps(&next_pos, map);
                seen_positions.insert(next_pos, next_steps.clone());
                next_layer_positions.extend(next_steps);
            }
        }
        next_positions = next_layer_positions.into_iter().unique().collect();
    }

    next_positions.iter().count()
}

pub fn run() {
    println!("Day 21 Part A");
    let input_filename = "inputs/day21/input.txt";

    let (plot_map, start_pos) = parse_input(input_filename);

    dbg!(&start_pos);
    dbg!(find_positions_after_steps(&plot_map, &start_pos, 64));
}