use array2d::Array2D;
use crate::day11::SpaceDataPoint::{EmptySpace, Galaxy};
use itertools::Itertools;
use num::abs;
use crate::read_lines::read_lines;

#[derive(Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug)]
struct SpaceLocation {
    x: i64,
    y: i64
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

#[allow(dead_code)]
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

fn count_prior_blanks(x_pos: usize, y_pos: usize, space_image: &SpaceImage) -> (i64, i64) {
    let mut prior_x_blank_count = 0;
    let mut prior_y_blank_count = 0;

    for (i, mut column) in space_image.columns_iter().enumerate() {
        if i >= x_pos {
            break;
        }
        if column.all(|&p1| p1 == EmptySpace) {
            prior_x_blank_count += 1;
        }
    }

    for (i, mut row) in space_image.rows_iter().enumerate() {
        if i >= y_pos {
            break;
        }
        if row.all(|&p1| p1 == EmptySpace) {
            prior_y_blank_count += 1;
        }
    }

    return (prior_x_blank_count, prior_y_blank_count);
}

fn find_galaxies(space_image: SpaceImage, age_factor: i64) -> Vec<SpaceLocation> {
    let mut galaxy_list = vec![];

    for (y_pos, x_pos) in space_image.indices_row_major() {
        if *space_image.get(y_pos, x_pos).unwrap() == Galaxy {
            // find blank rows and cols before this position and expand the positions
            let (prior_blank_x_count, prior_blank_y_count) = count_prior_blanks(x_pos, y_pos, &space_image);


            galaxy_list.push(SpaceLocation{
                x: ((age_factor-1) * prior_blank_x_count) + x_pos as i64,
                y: ((age_factor-1) * prior_blank_y_count) + y_pos as i64
            });
        }
    }

    return galaxy_list;
}

fn find_distance_between_galaxies_pairs(galaxy_locations: Vec<SpaceLocation>) -> Vec<i64>
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
    println!("Day 11 Part B");
    let input_filename = "inputs/day11/input.txt";

    let input_map = parse_input(input_filename);
    // let expanded_map = expand_universe(input_map);
    // let galaxy_locations = find_galaxies(expanded_map, 0);
    let galaxy_locations = find_galaxies(input_map, 1000000);
    dbg!(&galaxy_locations);
    let galaxy_distances = find_distance_between_galaxies_pairs(galaxy_locations);

    dbg!(galaxy_distances.iter().count());
    dbg!(galaxy_distances.iter().sum::<i64>());
}