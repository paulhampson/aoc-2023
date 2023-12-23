use std::cmp::{max, min};
use std::collections::HashMap;
use array2d::Array2D;
use crate::read_lines::read_lines;

#[derive(Debug, Copy, Clone)]
struct EndPos {
    x: i32,
    y: i32,
    z: i32
}

#[derive(Debug, Clone)]
struct Brick {
    id: usize,
    ends: Vec<EndPos>,
}

impl Brick {
    fn from_str(string: String, id:usize) -> Brick {
        let mut ends = vec![];
        let end_strings = string.split('~');
        for end in end_strings {
            let mut pos_strings = end.split(',');
            let x = pos_strings.next().unwrap().parse().unwrap();
            let y = pos_strings.next().unwrap().parse().unwrap();
            let z = pos_strings.next().unwrap().parse().unwrap();
            ends.push(EndPos{x,y,z});
        }

        Brick {
            id,
            ends
        }
    }

    fn set_base_z(&mut self, target_z: i32) {
        println!("Dropping brick to target_z {}", target_z);
        let z_delta = target_z - self.min_z();
        for mut end in &mut self.ends {
            end.z += z_delta;
        }
    }

    fn min_x(&self) -> i32 {
        let mut min_x = i32::MAX;
        for end in &self.ends {
            min_x = min(min_x, end.x);
        }
        min_x
    }

    fn min_y(&self) -> i32 {
        let mut min_y = i32::MAX;
        for end in &self.ends {
            min_y = min(min_y, end.y);
        }
        min_y
    }

    fn min_z(&self) -> i32 {
        let mut min_z = i32::MAX;
        for end in &self.ends {
            min_z = min(min_z, end.z);
        }
        min_z
    }

    fn max_x(&self) -> i32 {
        let mut max_x = 0;
        for end in &self.ends {
            max_x = max(max_x, end.x);
        }
        max_x
    }

    fn max_y(&self) -> i32 {
        let mut max_y = 0;
        for end in &self.ends {
            max_y = max(max_y, end.y);
        }
        max_y
    }

    fn max_z(&self) -> i32 {
        let mut max_z = 0;
        for end in &self.ends {
            max_z = max(max_z, end.z);
        }
        max_z
    }

    fn get_xy_overlapping_bricks(&self, bricks: &Vec<Brick>) -> Vec<Brick> {
        let mut overlapping_bricks = vec![];

        for brick in bricks {
            let x_overlap = max(self.min_x(), brick.min_x()) <= min(self.max_x(), brick.max_x());
            let y_overlap = max(self.min_y(), brick.min_y()) <= min(self.max_y(), brick.max_y());
            if x_overlap || y_overlap {
                overlapping_bricks.push(brick.clone());
            }
        }

        return overlapping_bricks;
    }
}

fn drop_bricks(mut brick_snapshot: Vec<Brick>) -> Vec<Brick> {
    let mut dropped_bricks = vec![];

    // 1. Sort the bricks by their current minimum Z as this is where their lowest point is in the snap shot.
    brick_snapshot.sort_by(|a,b| a.min_z().cmp(&b.min_z()));

    // 2. Setup a collision grid of size max x and y. Each grid point stores the highest Z. Checked the input and it's a 10x10 of x,y
    let mut collision_grid:Array2D<i32> = Array2D::filled_with(0, 3,3);

    // 3. For each brick:
    for mut brick in brick_snapshot {
        //    a) find the x,y points it occupies
        let min_x = brick.min_x();
        let min_y = brick.min_y();
        let max_x = brick.max_x();
        let max_y = brick.max_y();

        //    b) find the highest x,y in the collision grid that is in the occupation x,y set
        let mut max_z_from_collision_grid = 0;
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                max_z_from_collision_grid = max(max_z_from_collision_grid, *collision_grid.get(x as usize, y as usize).unwrap());
                println!("Z at {},{} in collision grid is {}", x, y, max_z_from_collision_grid);
            }
        }



        //    c) move the brick down to that z level
        brick.set_base_z(max_z_from_collision_grid+1);

        // find the maximum z of the brick
        let max_z_of_dropped_brick = brick.max_z();

        // push into the result
        dropped_bricks.push(brick);

        //    d) set the collision grid to be the maximum z that the brick covers
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                collision_grid.set(x as usize, y as usize, max_z_of_dropped_brick).expect("Failed to set Z on grid");
            }
        }
    }
    dropped_bricks
}

fn count_bricks_not_supporting_others(brick_snapshot: Vec<Brick>) -> i32 {
    // index the bricks by their max_z values
    let mut z_map:HashMap<i32, Vec<Brick>> = HashMap::new();
    let mut brick_by_id:HashMap<usize, Brick> = HashMap::new();

    for brick in brick_snapshot {
        brick_by_id.insert(brick.id, brick.clone());
        let brick_max_z = brick.max_z();
        if let Some(entry) = z_map.get(&brick_max_z) {
            let mut new_entry = entry.clone();
            new_entry.push(brick);
            z_map.insert(brick_max_z, new_entry);
        } else {
            let entry = vec![brick];
            z_map.insert(brick_max_z, entry);
        }
    }

    // let mut removal_candidate_list: Vec<_> = vec![];
    for (z_level, this_layer_bricks) in z_map.iter() {
        if let Some(bricks_above) = z_map.get(&(z_level + 1)){
            for brick in this_layer_bricks {
                // Find bricks this one supports
                let supported_bricks = brick.get_xy_overlapping_bricks(bricks_above);


            }
        } // if +1 failed we hit the top


        // Bricks on this layer must be supported by at least 1 brick
        // if supported by more than 1 then look if those bricks
        //  (a) supporting any others that have > 1 support or
        //  (b) not supporting any other.
        // in which case we can mark them up as being able to be removed.

    }

    0
}

fn parse_input(filename: &str) -> Vec<Brick> {
    let mut bricks = vec![];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for (id, line) in lines.enumerate() {
            if let Ok(ip) = line {
                bricks.push(Brick::from_str(ip, id));
            }
        }
    }
    else {
        println!("File not read");
    }

    return bricks;
}

pub fn run() {
    println!("Day 22 Part A");
    let input_filename = "inputs/day22/test.txt";

    let brick_snapshot = parse_input(input_filename);
    let dropped_bricks = drop_bricks(brick_snapshot);

    dbg!(count_bricks_not_supporting_others(dropped_bricks));

}