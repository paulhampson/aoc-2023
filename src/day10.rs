use array2d::Array2D;
use crate::day10::Direction::{East, North, South, West};
use crate::day10::PipeSegment::{EastWest, Ground, NorthEast, NorthSouth, NorthWest, SouthEast, SouthWest, StartPosition};
use crate::read_lines::read_lines;
use enum_iterator::{all, Sequence};
use num::abs;

#[derive(Clone, Eq, PartialEq)]
enum PipeSegment {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartPosition
}

impl PipeSegment {
    fn char_to_segment(c: char) -> PipeSegment {
        match c {
            '|' => NorthSouth,
            '-' => EastWest,
            'L' => NorthEast,
            'J' => NorthWest,
            '7' => SouthWest,
            'F' => SouthEast,
            '.' => Ground,
            'S' => StartPosition,
            _ => panic!("Invalid map character provided '{}'", c)
        }
    }

    /// Enter pipe travelling in a direction, returns the new direction of travel
    fn traverse_pipe(&self, current_direction: Direction) -> Direction {
        match self {
            NorthSouth => current_direction,
            EastWest => current_direction,
            NorthEast => {
                match current_direction {
                    South => East,
                    West => North,
                    _ => panic!("Invalid pipe travel")
                }
            },
            NorthWest => {
                match current_direction {
                    South => West,
                    East => North,
                    _ => panic!("Invalid pipe travel")
                }
            },
            SouthEast => {
                match current_direction {
                    North => East,
                    West => South,
                    _ => panic!("Invalid pipe travel")
                }
            },
            SouthWest => {
                match current_direction {
                    North => West,
                    East => South,
                    _ => panic!("Invalid pipe travel")
                }
            },
            _ => panic!("Trying to traverse ground or start position")
        }
    }
}

#[derive(Sequence, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {

    /// Return the row and column delta for a particular direction
    fn get_check_delta(&self) -> (i32, i32) {
        match self {
            North => (-1, 0),
            East => (0, 1),
            South => (1, 0),
            West => (0, -1)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct PipeMapLocation {
    row: usize,
    column: usize,
}

type PipeMap = Array2D<PipeSegment>;

trait PipeExtensions {
    fn get_pipe_at_location(&self, p: PipeMapLocation) -> PipeSegment;
}
impl PipeExtensions for PipeMap {
    fn get_pipe_at_location(&self, p: PipeMapLocation) -> PipeSegment
    {
        self.get(p.row, p.column).unwrap().clone()
    }
}

fn parse_input_and_find_start(input_filename: &str ) -> (PipeMap, PipeMapLocation) {
    let mut parse_input = vec![];
    let mut starting_location = PipeMapLocation { row: 0, column: 0 };

    if let Ok(lines) = read_lines(input_filename) {
        // Consumes the iterator, returns an (Optional) String
        for (row_idx, line) in lines.enumerate() {
            if let Ok(ip) = line {
                let mut row = vec![];
                for (col_idx, c) in ip.chars().enumerate() {
                    let pipe_segment = PipeSegment::char_to_segment(c);
                    if pipe_segment == StartPosition {
                        starting_location = PipeMapLocation {
                            row: row_idx,
                            column: col_idx
                        }
                    }
                    row.push(pipe_segment);
                }
                parse_input.push(row);
            }
        }
    }

    return (Array2D::from_rows(&*parse_input).unwrap(), starting_location);
}

fn next_location_in_direction(start_location: &PipeMapLocation, direction: Direction) -> Option<PipeMapLocation> {
    let (row_delta, col_delta) = direction.get_check_delta();

    let row = start_location.row as i32 + row_delta;
    let col = start_location.column as i32 + col_delta;

    if row < 0 || col < 0 {
        None
    } else {
        Some(PipeMapLocation {
            row: row as usize,
            column: col as usize
        })
    }
}
fn find_starting_direction(start_location: PipeMapLocation, pipe_map: &PipeMap) -> Direction {
    let mut start_direction = North;

    // search N, E, S, W for connecting pipes
    for direction in all::<Direction>() {
        if let Some(check_location) = next_location_in_direction(&start_location, direction){
            let adjacent_pipe = pipe_map.get_pipe_at_location(check_location);

            match direction {
                North => if adjacent_pipe == NorthSouth || adjacent_pipe == SouthEast || adjacent_pipe == SouthWest {
                    start_direction = North;
                }
                East => if adjacent_pipe == EastWest || adjacent_pipe == SouthWest || adjacent_pipe == NorthWest {
                    start_direction = East;
                }
                South => if adjacent_pipe == NorthSouth || adjacent_pipe == NorthWest || adjacent_pipe == NorthEast {
                    start_direction = South;
                }
                West => if adjacent_pipe == EastWest || adjacent_pipe == NorthEast || adjacent_pipe == SouthEast {
                    start_direction = West;
                }
            }
        }
    }

    return start_direction;
}

fn find_pipe_route(start_location: PipeMapLocation, pipe_map: &PipeMap) -> Vec<PipeMapLocation>
{
    let mut route = vec![];
    let mut keep_looking = true;

    let mut direction = find_starting_direction(start_location, pipe_map);
    let mut current_location = start_location;

    route.push(current_location);
    current_location = next_location_in_direction(&current_location, direction).unwrap();

    while keep_looking {
        direction = pipe_map.get_pipe_at_location(current_location)
                        .traverse_pipe(direction);
        route.push(current_location);

        current_location = next_location_in_direction(&current_location, direction).unwrap();
        keep_looking = start_location != current_location;
    }

    return route;
}

pub fn run() {
    println!("Day 10 Part A");
    let input_filename = "inputs/day10/input.txt";

    let (pipe_map, start_point) = parse_input_and_find_start(input_filename);
    let route = find_pipe_route(start_point, &pipe_map);

    dbg!(route.iter().count() / 2);

    println!("Day 10 Part B");

    dbg!(get_interior_point_count(route));
}

/// Pick's Theorem (https://en.wikipedia.org/wiki/Pick%27s_theorem) gives:
///
/// Area = interior_point_count + boundary_point_count/2 - 1
///
/// We want interior points. So:
///
/// interior_point_count = Area + 1 - boundary_point_count/2
///
/// Using the 'Shoelace algorithm' we can determine the area. The boundary point count is the
/// length of the route we found in part A.
fn get_interior_point_count(boundary_points: Vec<PipeMapLocation>) -> i64 {

    // Calculate area using shoelace
    let number_of_points = boundary_points.iter().count();
    let mut sum1 = 0;
    let mut sum2 = 0;

    for idx in 0..number_of_points-1 {
        sum1 = sum1 + (boundary_points[idx].column * boundary_points[idx+1].row) as i64;
        sum2 = sum2 + (boundary_points[idx].row * boundary_points[idx+1].column) as i64;
    }

    // Link back to the start
    sum1 = sum1 + (boundary_points[number_of_points-1].column * boundary_points[0].row) as i64;
    sum2 = sum2 + (boundary_points[number_of_points-1].row * boundary_points[0].column) as i64;

    let area_of_polygon = abs(sum1 - sum2) / 2;

    return area_of_polygon - ((number_of_points as i64)/2) + 1;
}