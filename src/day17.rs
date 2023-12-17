use array2d::Array2D;
use itertools::Itertools;
use crate::read_lines::read_lines;
use pathfinding::prelude::dijkstra;
use DirectionOfTravel::{East, North, South, West};

type WeightGrid = Array2D<usize>;

fn parse_input(filename: &str) -> WeightGrid {
    let mut weight_grid_rows = vec![];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let mut row = vec![];
            if let Ok(ip) = line {
                for c in ip.chars() {
                    row.push(c.to_digit(10).unwrap() as usize);
                }
            }
            weight_grid_rows.push(row);
        }
    }
    else {
        println!("File not read");
    }

    Array2D::from_rows(&*weight_grid_rows).unwrap()
}

// row, column
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
enum DirectionOfTravel {
    North,
    East,
    South,
    West,
}

impl DirectionOfTravel {
    fn go(d: DirectionOfTravel) -> (i32, i32) {
        match d {
            North => (0, -1),
            South => (0, 1),
            West => (-1, 0),
            East => (1, 0)
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Cart{
    p: Pos,
    d: Option<DirectionOfTravel>,
    h: Vec<DirectionOfTravel>
}

fn next_cart_states(weight_grid: &WeightGrid, c: &Cart) -> Vec<(Cart, usize)>
{
    // First work out what would be the reverse direction and eliminate that from the set, if we aren't travelling yet
    // (e.g. at the start) we can go in any direction
    let mut possible_next_step = vec![North, South, West, East];
    if let Some(direction_of_travel) = c.d {
        possible_next_step = match direction_of_travel {
            North => vec![North, West, East],
            East => vec![North, South, East],
            South => vec![South, West, East],
            West => vec![North, South, West],
        }
    }

    // eliminate directions if we have been in the same direction 3 times
    if c.h.len() == 3 && c.h.iter().all_equal() {
        let d = c.h.last().unwrap();
        if let Some((index, _)) = possible_next_step.iter().find_position(|&s| s == d) {
            possible_next_step.remove(index);
        }
    }

    let mut next_steps_vec = vec![];

    for step in possible_next_step {
        let (row_delta, col_delta) = DirectionOfTravel::go(step);
        let next_row = c.p.0 as i32 + row_delta;
        let next_col = c.p.1 as i32 + col_delta;

        // check we are still in bounds
        if next_row < 0 || next_row >= weight_grid.num_rows() as i32 ||
            next_col < 0 || next_col >= weight_grid.num_columns() as i32 {
            continue;
        }

        let mut updated_history = c.h.clone();
        updated_history.push(step);
        if updated_history.len() > 3 {
            let start_index = updated_history.len() - 3;
            updated_history = updated_history.drain(start_index..).collect();
        }

        let next_cart_state = Cart {
            p: Pos(next_row as usize, next_col as usize),
            h: updated_history,
            d: Some(step)
        };

        let &weight = weight_grid.get(next_row as usize, next_col as usize).unwrap();
        next_steps_vec.push((next_cart_state, weight));
    }

    return next_steps_vec;
}

fn success_check(c: &Cart, t: &Pos) -> bool {
    c.p == *t
}

fn find_lowest_heat_loss_path(weight_grid: &WeightGrid) -> usize {

    let successors = |c:&Cart| next_cart_states(weight_grid, c);
    let target_position = Pos(weight_grid.num_rows()-1, weight_grid.num_columns()-1);
    let success_check = |c:&Cart| success_check(c, &target_position);
    let starting_state = Cart {
        p: Pos(0,0),
        d: None,
        h: vec![]
    };

    let (path, cost) = dijkstra(&starting_state, successors, success_check).unwrap();
    dbg!(path);
    cost
}

// fn next_steps(weight_grid: &WeightGrid, current_pos: &Pos) -> Vec<(Pos, usize)>
// {
//     // (r, c)
//     static UP: (i32, i32) = (0, -1);
//     static DOWN: (i32, i32) = (0, 1);
//     static LEFT: (i32, i32) = (-1, 0);
//     static RIGHT: (i32, i32) = (1, 0);
//
//     // generate a vector of possible next steps and their weight
//     let possible_paths = vec![
//         vec![UP, RIGHT],
//         vec![UP, UP, RIGHT],
//         vec![UP, UP, UP, RIGHT],
//         vec![UP, LEFT],
//         vec![UP, UP, LEFT],
//         vec![UP, UP, UP, LEFT],
//         vec![DOWN, RIGHT],
//         vec![DOWN, DOWN, RIGHT],
//         vec![DOWN, DOWN, DOWN, RIGHT],
//         vec![DOWN, LEFT],
//         vec![DOWN, DOWN, LEFT],
//         vec![DOWN, DOWN, DOWN, LEFT],
//         vec![LEFT, UP],
//         vec![LEFT, LEFT, UP],
//         vec![LEFT, LEFT, LEFT, UP],
//         vec![LEFT, DOWN],
//         vec![LEFT, LEFT, DOWN],
//         vec![LEFT, LEFT, LEFT, DOWN],
//         vec![RIGHT, UP],
//         vec![RIGHT, RIGHT, UP],
//         vec![RIGHT, RIGHT, RIGHT, UP],
//         vec![RIGHT, DOWN],
//         vec![RIGHT, RIGHT, DOWN],
//         vec![RIGHT, RIGHT, RIGHT, DOWN],
//     ];
//
//     // calculate final position and weight
//     let mut next_positions_with_weight = vec![];
//     for path in possible_paths {
//         let &Pos(row, col) = current_pos;
//         let mut final_row = row as i32;
//         let mut final_col = col as i32;
//         let mut total_cost = 0;
//         let mut path_valid = true;
//
//         for step in path {
//             let (row_delta, col_delta) = step;
//             final_row = final_row + row_delta;
//             final_col = final_col + col_delta;
//             if let Some(cost) = weight_grid.get(final_row as usize, final_col as usize) {
//                 total_cost += cost;
//             } else {
//                 path_valid = false;
//                 break;
//             }
//         }
//
//         // add to the options if it's a valid path through the grid
//         if path_valid {
//             next_positions_with_weight.push((Pos(final_row as usize, final_col as usize), total_cost));
//         }
//     }
//
//     next_positions_with_weight
// }

// fn find_lowest_heat_loss_path(weight_grid: &WeightGrid) -> usize {
//
//     let successors = |p:&Pos| next_steps(weight_grid, p);
//     let goal:Pos = Pos(weight_grid.num_rows()-1, weight_grid.num_columns()-1);
//     let starting_position = Pos(0,0);
//
//     let (path, cost) = dijkstra(&starting_position, successors, |p| *p == goal).unwrap();
//     dbg!(path);
//     cost
// }

pub fn run() {
    println!("Day 17 Part A");
    let input_filename = "inputs/day17/test.txt";

    let weight_grid = parse_input(input_filename);
    dbg!(find_lowest_heat_loss_path(&weight_grid));
}