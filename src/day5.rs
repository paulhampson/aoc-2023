use std::cmp::min;
use regex::Regex;
use crate::read_lines::read_lines;
use indexmap::IndexMap;

struct MapEntry {
    dest_start: i64,
    source_start: i64,
    count: i64
}

#[derive(Clone, Copy)]
struct MappingChunk {
    chunk_start: i64,
    chunk_count: i64
}

type SrcToDestMap = Vec<MapEntry>;
type Almanac = IndexMap<String, SrcToDestMap>;

fn map_source_to_destination(source: i64, src_to_dest_map: &SrcToDestMap) -> i64 {
    // check entries in the map
    for map_entry in src_to_dest_map {
        let min_src = map_entry.source_start;
        let max_src = map_entry.source_start + map_entry.count;

        if min_src <= source && source < max_src {
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

/// Takes a source chunk and identifies where it maps to and returns the destination chunk and
/// also the remaining source chunk if the source is not fully covered by the destination.
fn map_source_to_destination_chunked(source_chunk: MappingChunk, src_to_dest_map: &SrcToDestMap) -> (MappingChunk, Option<MappingChunk>) {
    // check entries in the map, work out what was consumed from the source chunk and return the destination chunk and remaining chunk (if any)
    for map_entry in src_to_dest_map {
        let min_map_src = map_entry.source_start;
        let max_map_src = map_entry.source_start + map_entry.count;

        //          |--source_chunk -----------------------|
        // |--map entry ------------|
        //          |--dest_chunk---|-remaining_src_chunk--|
        if min_map_src <= source_chunk.chunk_start && source_chunk.chunk_start < max_map_src {
            let dest_offset = source_chunk.chunk_start - min_map_src;
            let src_max_consumption = max_map_src - source_chunk.chunk_start;

            return map_to_dest_and_split_chunk(&source_chunk, map_entry, dest_offset, src_max_consumption);
        }
    }

    // otherwise direct mapping where dest = src (offset=0), but we need to locate where the chunk ends and
    // split it if necessary
    for map_entry in src_to_dest_map {
        let min_map_src = map_entry.source_start;

        if min_map_src > source_chunk.chunk_start {
            let src_max_consumption = min_map_src - source_chunk.chunk_start;

            // create a proxy map for this chunk of the map so that we can do the splitting correctly
            // The proxy map starts where the src starts and runs all the way until the map_entry's
            // start point (calculated as src_max_consumption).
            let proxy_map_entry = MapEntry{
                source_start: source_chunk.chunk_start,
                dest_start: source_chunk.chunk_start,
                count: src_max_consumption
            };

            return map_to_dest_and_split_chunk(&source_chunk, &proxy_map_entry, 0, src_max_consumption);
        }
    }

    // if we reach here we fell off the end of the src=dest map search, so the whole source will be
    // consumed with offset 0 mapping as the end of the map is a src=dest that runs for infinity
    let proxy_map_entry = MapEntry{
        source_start: source_chunk.chunk_start,
        dest_start: source_chunk.chunk_start,
        count: source_chunk.chunk_count
    };
    return map_to_dest_and_split_chunk(&source_chunk, &proxy_map_entry, 0, source_chunk.chunk_count);
}

fn map_to_dest_and_split_chunk(source_chunk: &MappingChunk, map_entry: &MapEntry, dest_offset: i64, src_max_consumption: i64) -> (MappingChunk, Option<MappingChunk>) {
    let is_whole_chunk_consumed = src_max_consumption >= source_chunk.chunk_count;
    return if is_whole_chunk_consumed {
        let destination_chunk = MappingChunk {
            chunk_start: map_entry.dest_start + dest_offset,
            chunk_count: source_chunk.chunk_count
        };
        // println!("whole chunk consumed");
        // dbg!(source_chunk.chunk_start);
        // dbg!(source_chunk.chunk_count);
        // dbg!(destination_chunk.chunk_start);
        // dbg!(destination_chunk.chunk_count);

        (destination_chunk, None)
    } else {
        let destination_chunk = MappingChunk {
            chunk_start: map_entry.dest_start + dest_offset,
            chunk_count: src_max_consumption
        };
        let remaining_source_chunk = MappingChunk {
            chunk_start: source_chunk.chunk_start + src_max_consumption,
            chunk_count: source_chunk.chunk_count - src_max_consumption
        };
        // println!("--8<-- chunk split");
        // dbg!(src_max_consumption);
        // dbg!(source_chunk.chunk_start);
        // dbg!(source_chunk.chunk_count);
        // dbg!(destination_chunk.chunk_start);
        // dbg!(destination_chunk.chunk_count);
        // dbg!(remaining_source_chunk.chunk_start);
        // dbg!(remaining_source_chunk.chunk_count);

        (destination_chunk, Some(remaining_source_chunk))
    };
}

fn minimum_map_to_location_chunked(seed_chunk: MappingChunk, starting_map_name: &str, almanac: &Almanac) -> i64 {
    let mut destination_chunk = seed_chunk;
    let mut min_location_found = i64::MAX;
    println!("--> starting mapping branch");

    let almanac_start_idx = almanac.get_index_of(starting_map_name).unwrap();
    for (map_name, src_to_dest_map) in almanac.iter().skip(almanac_start_idx) {
        println!("Looking at map {}, source chunk start={}, source chunk size={}", map_name, destination_chunk.chunk_start, destination_chunk.chunk_count);

        let (new_destination_chunk, remaining_source_chunk) = map_source_to_destination_chunked(destination_chunk, src_to_dest_map);
        destination_chunk = new_destination_chunk;

        // check if the chunk has remaining source to cover and find the minimum destination of that source
        if let Some(remaining_source_chunk) = remaining_source_chunk {
            min_location_found = min(min_location_found, minimum_map_to_location_chunked(remaining_source_chunk, map_name, almanac));
        }
    }
    let result = min(min_location_found, destination_chunk.chunk_start);
    println!("lowest location found = {}", result);

    return result;
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

pub fn run_part_b() {
    println!("Day 5 part B");
    let (seed_list, almanac) = load_maps();

    // For part B to be efficient need process chunks of seeds, not individual seeds - so we can split into multiple paths for each mapping

    let mut location:i64 = i64::MAX;
    for chunk in seed_list.chunks(2) {
        println!("\n-> New seed chunk {} {}", chunk[0], chunk[1]);
        let seed_chunk = MappingChunk {
            chunk_start: chunk[0],
            chunk_count: chunk[1]
        };

        let location_found = minimum_map_to_location_chunked(seed_chunk, "seed-to-soil", &almanac);
        location = min(location, location_found);
        println!("location found = {}", location_found);
    }

    println!("\nLowest location = {}", location);
}