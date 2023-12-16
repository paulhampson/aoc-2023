use std::collections::HashMap;
use array2d::Array2D;
use LightTravelDirection::{BottomToTop, RightToLeft, TopToBottom};
use crate::day16::CellType::{EmptySpace, MirrorLeftLean, MirrorRightLean, SplitterHorizontal, SplitterVertical};
use crate::day16::LightTravelDirection::LeftToRight;
use crate::read_lines::read_lines;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
enum LightTravelDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop
}

impl LightTravelDirection {

    fn travel_increment(&self) -> (i32, i32) {
        match self {
            LeftToRight => (0,1),
            RightToLeft => (0,-1),
            TopToBottom => (1,0),
            BottomToTop => (-1,0)
        }
    }
}

#[derive(Clone, Debug)]
enum CellType {
    EmptySpace,
    MirrorLeftLean,
    MirrorRightLean,
    SplitterVertical,
    SplitterHorizontal,
}

impl CellType {
    fn from_char(c: char) -> CellType {
        match c {
            '.' => EmptySpace,
            '\\' => MirrorLeftLean,
            '/' => MirrorRightLean,
            '|' => SplitterVertical,
            '-' => SplitterHorizontal,
            _ => panic!("Invalid character for a cell")
        }
    }

    fn travel_through_cell(&self, d: &LightTravelDirection) -> Vec<LightTravelDirection> {
        match self {
            EmptySpace => vec![d.clone()],
            MirrorLeftLean | MirrorRightLean => Self::travel_through_mirror(self, d),
            SplitterVertical | SplitterHorizontal => Self::travel_through_splitter(self, d),
        }
    }

    fn travel_through_mirror(c: &CellType, d: &LightTravelDirection) -> Vec<LightTravelDirection> {
        match c {
            MirrorRightLean => match d { // '/'
                LeftToRight => {vec![BottomToTop]}
                RightToLeft => {vec![TopToBottom]}
                TopToBottom => {vec![RightToLeft]}
                BottomToTop => {vec![LeftToRight]}
            },
            MirrorLeftLean => match d { // '\'
                LeftToRight => {vec![TopToBottom]}
                RightToLeft => {vec![BottomToTop]}
                TopToBottom => {vec![LeftToRight]}
                BottomToTop => {vec![RightToLeft]}
            },
            _ => panic!("Asked to travel through mirror on what is not a mirror!")
        }
    }

    fn travel_through_splitter(c: &CellType, d: &LightTravelDirection) -> Vec<LightTravelDirection> {
        match c {
            SplitterHorizontal => match d { // '-'
                TopToBottom | BottomToTop => vec![LeftToRight, RightToLeft],
                LeftToRight | RightToLeft => vec![d.clone()]
            }
            SplitterVertical => match d { // '|'
                TopToBottom | BottomToTop => vec![d.clone()],
                LeftToRight | RightToLeft => vec![TopToBottom, BottomToTop]
            }
            _ => panic!("Asked to travel through splitter on what is not a splitter!")
        }
    }

    fn translate_light_beam(&self, light_beam: &LightBeam) -> Vec<LightBeam>
    {
        let travel_results = self.travel_through_cell(&light_beam.direction);

        let mut translated_beams = vec![];
        for travel_result in travel_results {
            let (row_delta, col_delta) = travel_result.travel_increment();
            translated_beams.push(LightBeam {
                row: (light_beam.row as i32 + row_delta) as usize,
                column: (light_beam.column as i32 + col_delta) as usize,
                direction: travel_result
            });
        }
        translated_beams
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct LightBeam {
    row: usize,
    column: usize,
    direction: LightTravelDirection
}

type ActivationMap = Array2D<bool>;
type CellGrid = Array2D<CellType>;

fn parse_input(filename: &str) -> CellGrid {
    let mut cell_grid_as_vector = vec![];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let mut cell_row = vec![];
            if let Ok(ip) = line {
                for c in ip.chars() {
                    cell_row.push(CellType::from_char(c));
                }
                cell_grid_as_vector.push(cell_row);
            }
        }
    }
    else {
        println!("File not found");
    }
    CellGrid::from_rows(&*cell_grid_as_vector).unwrap()
}

fn find_activated_cell_count(cell_grid: CellGrid, print_activation_map: bool) -> usize {
    let mut activation_map = ActivationMap::filled_with(false, cell_grid.num_rows(), cell_grid.num_columns());

    let mut light_beams = vec![];
    light_beams.push(LightBeam {
        row: 0,
        column: 0,
        direction: LeftToRight
    });

    let mut beam_history = HashMap::new();

    while light_beams.len() > 0 {
        let mut new_light_beam_set = vec![];
        for light_beam in light_beams.iter().cloned() {
            if activation_map.set(light_beam.row, light_beam.column, true).is_err() {
                // indices are out of bounds - light beam has exited the map, so we don't do anything
                ()
            } else {
                // If the light beam is in the history then we've traversed this route before (same
                // location and direction), so we don't need to do it again as we know what's been
                // activated. This optimises and also stops loops.
                if !beam_history.contains_key(&light_beam) {
                    beam_history.insert(light_beam, true);
                    // light beam is still in the map, we need it to make its next move.
                    let cell = cell_grid.get(light_beam.row, light_beam.column).unwrap();
                    new_light_beam_set.extend(cell.translate_light_beam(&light_beam));
                }
            }
        }
        light_beams = new_light_beam_set;
    }

    if print_activation_map {

        for row in activation_map.rows_iter() {
            let mut row_string = String::from("");
            for &cell in row {
                if cell { row_string.push('#'); }
                else { row_string.push('.') }
            }
            println!( "{}", row_string );
        }
    }

    activation_map.as_row_major().iter().filter(|c| **c).count()
}

pub fn run() {
    println!("Day 16 Part A");
    let input_filename = "inputs/day16/input.txt";

    let cell_grid = parse_input(input_filename);
    dbg!(find_activated_cell_count(cell_grid, false));

}