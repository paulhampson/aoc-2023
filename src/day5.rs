use std::cmp::min;
use regex::Regex;
use crate::read_lines::read_lines;
use indexmap::IndexMap;

struct MapEntry {
    dest_start: i64,
    source_start: i64,
    count: i64
}

type SrcToDestMap = Vec<MapEntry>;
type Almanac = IndexMap<String, SrcToDestMap>;

fn map_source_to_destination(source: i64, src_to_dest_map: &SrcToDestMap) -> i64 {
    // check entries in the map
    for map_entry in src_to_dest_map {
        let min_src = map_entry.source_start;
        let max_src = map_entry.source_start + map_entry.count;

        if min_src <= source && source <= max_src {
            let offset = source - min_src;
            return map_entry.dest_start + offset;
        }
    }

    // otherwise direct mapping where dest = src
    return source;
}

fn seed_to_location(seed_id:i64, almanac: &Almanac) -> i64 {
    let mut dest = seed_id;
    println!("seed = {}", dest);

    for (map_name, src_to_dest_map) in almanac.iter() {
        print!("Looking at {}, src={}", map_name, dest);
        dest = map_source_to_destination(dest, src_to_dest_map);
        println!(", dest={}", dest);
    }

    return dest;
}

fn load_maps() -> (Vec<i64>, Almanac) {
    let mut map_collection = Almanac::new();
    let mut seed_list:Vec<i64> = vec![];

    if let Ok(lines) = read_lines("./inputs/day5/input.txt") {
        let map_entry_re = Regex::new(r"^(?<dest_start>[0-9]*)\s+(?<src_start>[0-9]*)\s+(?<map_length>[0-9]*)$").unwrap();
        let map_name_re = Regex::new(r"^(?<map_name>[\-\w]+) map:$").unwrap();
        let seeds_re = Regex::new(r"^seeds: (?<seeds>[0-9\s?]*)").unwrap();

        let mut map_name:Option<String> = None;
        for line in lines {
            if let Ok(ip) = line {
                if let Some(line_data_capture) = map_entry_re.captures(&ip) {
                    let map_entry = MapEntry {
                        source_start: line_data_capture["src_start"].parse().unwrap(),
                        dest_start: line_data_capture["dest_start"].parse().unwrap(),
                        count: line_data_capture["map_length"].parse().unwrap(),
                    };

                    assert!(map_name.is_some(), "No map name identified yet");
                    let key = map_name.clone().unwrap();
                    let almanac_entry = map_collection.entry(key).or_insert(vec![]);
                    almanac_entry.push(map_entry);
                    almanac_entry.sort_by(|a,b| a.source_start.cmp(&b.source_start));
                } else if let Some(map_name_capture) = map_name_re.captures(&ip) {
                    map_name = Some(map_name_capture["map_name"].to_string());
                } else if let Some(seed_ids_capture) = seeds_re.captures(&ip) {
                    for seed_str in seed_ids_capture["seeds"].to_string().split_ascii_whitespace() {
                        seed_list.push(seed_str.parse().unwrap());
                    }
                }
            }
        }
    }

    return (seed_list, map_collection);
}

pub fn run() {
    println!("Day 5 part A");
    let (seed_list, almanac) = load_maps();

    let mut location:i64 = i64::MAX;
    for seed in seed_list {
        location = min(location, seed_to_location(seed, &almanac));
    }

    println!("Lowest location = {}", location);
}