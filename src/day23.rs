use std::cmp::max;
use array2d::Array2D;
use itertools::Itertools;
use pathfinding::directed::dfs::dfs_reach;
use crate::day23::MapTileType::{EastSlope, Forest, NorthSlope, Path, SouthSlope, WestSlope};
use crate::read_lines::read_lines;

#[derive(Copy, Clone, Eq, PartialEq)]
enum MapTileType {
    Path,
    Forest,
    NorthSlope,
    SouthSlope,
    EastSlope,
    WestSlope
}

impl MapTileType {
    fn from_char(c: &char) -> MapTileType {
        match c {
            '.' => Path,
            '#' => Forest,
            '^' => NorthSlope,
            '>' => EastSlope,
            'v' => SouthSlope,
            '<' => WestSlope,
            _ => panic!("Incorrect map character")
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos(usize, usize);

type MapGrid = Array2D<MapTileType>;

trait MapGridExt<T> {
    fn get_pos(&self, pos: &Pos) -> Option<&T>;
}

impl MapGridExt<MapTileType> for MapGrid {
    fn get_pos(&self, p: &Pos) -> Option<&MapTileType> {
        // Pos(x, y) translates to (col, row)
        let Pos(col, row) = *p;
        self.get(row, col)
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct MapTraversalTracker {
    position: Pos,
    history: Vec<Pos>
}

fn parse_input(filename: &str) -> Array2D<MapTileType> {
    let mut rows = vec![];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let mut row = vec![];
            if let Ok(ip) = line {
                for c in ip.chars() {
                    row.push(MapTileType::from_char(&c));
                }
            }
            rows.push(row);
        }
    }
    else {
        println!("File not read");
    }

    MapGrid::from_rows(&*rows).unwrap()
}

fn get_start_point(map_grid: &MapGrid) -> Pos {
    let grid_as_rows = map_grid.as_rows();
    let first_row = grid_as_rows.get(0).unwrap();
    let start_x_pos = first_row.iter().position(|&x| x == Path).unwrap();
    Pos(start_x_pos, 0)
}

fn get_end_point(map_grid: &MapGrid) -> Pos {
    let grid_as_rows = map_grid.as_rows();
    let last_row = grid_as_rows.last().unwrap();
    let finish_x_pos = last_row.iter().position(|&x| x == Path).unwrap();
    Pos(finish_x_pos, map_grid.num_rows()-1)
}

fn next_map_positions(map_grid: &MapGrid, current_tracker: &MapTraversalTracker) -> Vec<MapTraversalTracker>
{
    // Find next positions. Rules:
    // (1) Can only move N/E/S/W
    // (2) Cannot go somewhere in the history
    // (3) Can only go in direction of slope if on a slope
    // (4) Can only step on paths

    // x, y
    static SOUTH_STEP: (i32, i32) = (0, 1);
    static NORTH_STEP:  (i32, i32) = (0, -1);
    static EAST_STEP: (i32, i32) = (1, 0);
    static WEST_STEP: (i32, i32) = (-1, 0);

    let mut possible_next_positions = vec![];


    let current_map_tile = map_grid.get_pos(&current_tracker.position).unwrap();

    // Build valid step options - slopes can only move in the direction they slope in
    // Rule 1 & 3
    let deltas = match current_map_tile {
        NorthSlope => vec![NORTH_STEP],
        SouthSlope => vec![SOUTH_STEP],
        EastSlope => vec![EAST_STEP],
        WestSlope => vec![WEST_STEP],
        _ => vec![NORTH_STEP, SOUTH_STEP, EAST_STEP, WEST_STEP]
    };

    for d in deltas {
        let Pos(x,y) = current_tracker.position;
        let next_x = x as i32 + d.0;
        let next_y = y as i32 + d.1;

        // Eliminate out of bound options
        if next_x < 0 || next_x >= map_grid.num_columns() as i32 ||
            next_y < 0 || next_y >= map_grid.num_rows() as i32 {
            continue;
        }

        let next_pos = Pos(next_x as usize, next_y as usize);

        // Eliminate options that are forest - Rule 4
        match map_grid.get_pos(&next_pos).unwrap() {
            Forest => continue,
            _ => ()
        };

        possible_next_positions.push(next_pos);
    }

    // filter out any options that already are in the history (rule 2) as we build the final output
    let mut move_options_to_return = vec![];
    for position in possible_next_positions.iter().filter(|p| !current_tracker.history.contains(p)) {
        let mut new_tracker = MapTraversalTracker {
            position: position.clone(),
            history: current_tracker.history.clone()
        };

        // include this position in the history
        new_tracker.history.push(current_tracker.position.clone());

        move_options_to_return.push(new_tracker);
    }

    move_options_to_return
}

fn get_longest_scenic_path_length(map_grid: &MapGrid, start_position: &Pos, end_position: &Pos) -> usize {
    let starting_tracker = MapTraversalTracker{ position: start_position.clone(), history: vec![] };
    let mut routes_to_end = dfs_reach(starting_tracker, |ct| next_map_positions(map_grid, ct)).into_iter()
                                .filter(|ct| ct.position == *end_position);

    let mut max_len = 0;
    for reached_node in routes_to_end {
        max_len = max(reached_node.history.len(), max_len);
    }

    max_len
}

pub fn run() {
    println!("Day 23 Part A");
    let input_filename = "inputs/day23/input.txt";

    let map_grid = parse_input(input_filename);
    let start_point = get_start_point(&map_grid);
    let end_point = get_end_point(&map_grid);

    dbg!(get_longest_scenic_path_length(&map_grid, &start_point, &end_point));

}