use array2d::Array2D;
use crate::day11::SpaceDataPoint::{EmptySpace, Galaxy};
use itertools::Itertools;
use num::abs;
use crate::read_lines::read_lines;

#[derive(Clone, PartialEq)]
enum SpaceDataPoint {
    Galaxy,
    EmptySpace
}

impl SpaceDataPoint {
    fn get_space_data_point_from_char(c: char) -> SpaceDataPoint {
        match c {
            '.' => EmptySpace,
            '#' => Galaxy,
            _ => panic!("Invalid map data point")
        }
    }
}

type SpaceImage = Array2D<SpaceDataPoint>;

struct SpaceLocation {
    x: i32,
    y: i32
}

struct GalaxyPair {
    galaxy_a: SpaceLocation,
    galaxy_b: SpaceLocation,
    distance: usize
}

fn parse_input(filename: &str) -> SpaceImage {
    let mut image_vec = vec![];

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let mut row = vec![];
            if let Ok(ip) = line {
                for c in ip.chars() {
                    row.push(SpaceDataPoint::get_space_data_point_from_char(c));
                }
                image_vec.push(row);
            }
        }
    }

    SpaceImage::from_rows(&*image_vec).unwrap()
}

fn expand_universe(image: SpaceImage) -> SpaceImage {
    let mut column_expansion_vec = vec![];
    for col in image.as_columns().iter()
    {
        column_expansion_vec.push(col.clone().to_vec());
        if col.iter().all(|x| *x == EmptySpace) {
            column_expansion_vec.push(col.clone().to_vec());
        }
    }

    let column_expand_image = SpaceImage::from_columns(&*column_expansion_vec).unwrap();

    let mut row_expansion_vec = vec![];
    for row in column_expand_image.as_rows().iter()
    {
        row_expansion_vec.push(row.clone().to_vec());
        if row.iter().all(|x| *x == EmptySpace) {
            row_expansion_vec.push(row.clone().to_vec());
        }
    }

    SpaceImage::from_rows(&*row_expansion_vec).unwrap()
}

fn find_galaxies(space_image: SpaceImage) -> Vec<SpaceLocation> {
    let mut galaxy_list = vec![];

    for (x_pos, y_pos) in space_image.indices_row_major() {
        if *space_image.get(x_pos, y_pos).unwrap() == Galaxy {
            galaxy_list.push(SpaceLocation{
                x: x_pos as i32,
                y: y_pos as i32
            });
        }
    }

    return galaxy_list;
}

fn find_distance_between_galaxies_pairs(galaxy_locations: Vec<SpaceLocation>) -> Vec<i32>
{
    let mut galaxy_distances = vec![];

    for pair in galaxy_locations.iter().combinations(2) {
        let galaxy_a = pair[0];
        let galaxy_b = pair[1];

        let steps_count = abs(galaxy_a.y - galaxy_b.y) + abs(galaxy_a.x - galaxy_b.x);
        galaxy_distances.push(steps_count);
    }

    return galaxy_distances;
}

pub fn run() {
    println!("Day 11 Part A");
    let input_filename = "inputs/day11/input.txt";

    let input_map = parse_input(input_filename);
    let expanded_map = expand_universe(input_map);
    let galaxy_locations = find_galaxies(expanded_map);
    let galaxy_distances = find_distance_between_galaxies_pairs(galaxy_locations);

    dbg!(galaxy_distances.iter().count());
    dbg!(galaxy_distances.iter().sum::<i32>());
}