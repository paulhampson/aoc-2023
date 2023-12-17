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
            // row, col
            North => (-1, 0),
            South => (1, 0),
            West => (0, -1),
            East => (0, 1)
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Cart{
    p: Pos,
    d: Option<DirectionOfTravel>,
    h: Vec<DirectionOfTravel>
}

fn part_a_next_steps(c: &Cart) -> Vec<DirectionOfTravel>
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

    // eliminate direction if we have been in the same direction 3 times
    if c.h.len() >= 3 {
        if c.h.rchunks(3).next().unwrap().iter().all_equal() {
            let d = c.h.last().unwrap();
            if let Some((index, _)) = possible_next_step.iter().find_position(|&s| s == d) {
                possible_next_step.remove(index);
            }
        }
    }

    possible_next_step
}

fn part_b_next_steps(c: &Cart) -> Vec<DirectionOfTravel>
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

    // eliminate directions if we have been in the same direction 10 times
    if c.h.len() >= 10 && c.h.iter().all_equal() {
        let d = c.h.last().unwrap();
        if let Some((index, _)) = possible_next_step.iter().find_position(|&s| s == d) {
            possible_next_step.remove(index);
        }
    }

    // travel minimum of 4 steps in same direction before being allowed to turn
    if c.h.len() > 0 {
        if c.h.len() < 4 || !c.h.rchunks(4).next().unwrap().iter().all_equal()
        {
            possible_next_step = vec![c.h.last().unwrap().clone()];
        }
    }

    possible_next_step
}

fn next_cart_states(weight_grid: &WeightGrid, c: &Cart, part_a: bool) -> Vec<(Cart, usize)>
{

    let possible_next_step:Vec<DirectionOfTravel> =
        if part_a {
            part_a_next_steps(c)
        } else {
            part_b_next_steps(c)
        };

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
        let max_history_length = if part_a { 3 } else { 10 };
        if updated_history.len() > max_history_length {
            let start_index = updated_history.len() - max_history_length;
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

fn success_check_part_a(c: &Cart, t: &Pos) -> bool {
    c.p == *t
}

fn success_check_part_b(c: &Cart, t: &Pos) -> bool {
    // in position and we had 4 steps in the sam direction to slow down
    c.p == *t && c.h.rchunks(4).next().unwrap().iter().all_equal()
}

fn find_lowest_heat_loss_path_part_a(weight_grid: &WeightGrid) -> usize {

    let successors = |c:&Cart| next_cart_states(weight_grid, c, true);
    let target_position = Pos(weight_grid.num_rows()-1, weight_grid.num_columns()-1);
    let success_check = |c:&Cart| success_check_part_a(c, &target_position);
    let starting_state = Cart {
        p: Pos(0,0),
        d: None,
        h: vec![]
    };

    let (_, cost) = dijkstra(&starting_state, successors, success_check).unwrap();
    cost
}

fn find_lowest_heat_loss_path_part_b(weight_grid: &WeightGrid) -> usize {

    let successors = |c:&Cart| next_cart_states(weight_grid, c, false);
    let target_position = Pos(weight_grid.num_rows()-1, weight_grid.num_columns()-1);
    let success_check = |c:&Cart| success_check_part_b(c, &target_position);
    let starting_state = Cart {
        p: Pos(0,0),
        d: None,
        h: vec![]
    };

    let (_, cost) = dijkstra(&starting_state, successors, success_check).unwrap();
    cost
}

pub fn run() {
    println!("Day 17 Part A");
    let input_filename = "inputs/day17/input.txt";

    let weight_grid = parse_input(input_filename);
    dbg!(find_lowest_heat_loss_path_part_a(&weight_grid));

    println!("Day 17 Part B");
    dbg!(find_lowest_heat_loss_path_part_b(&weight_grid));
}