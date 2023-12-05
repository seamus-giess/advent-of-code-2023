use std::collections::HashMap;
use std::str::FromStr;
use std::usize;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

struct Object {
    seeds: Vec<usize>,
    processes: Vec<Vec<(usize, usize, usize)>>,
}
impl Object {}
struct StringParseError;
impl FromStr for Object {
    type Err = StringParseError;
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        // Split Seeds from Processes
        let (seeds_string, processes_string) = match data.split_once("\n\n") {
            Some((seeds_string, process_string)) => (seeds_string, process_string),
            None => return Err(StringParseError),
        };

        let seeds = match match seeds_string.split_once(": ") {
            Some((_, seed_string)) => seed_string,
            None => return Err(StringParseError),
        }
        .split_whitespace()
        .map(|seed_string| seed_string.parse())
        .collect::<Result<Vec<usize>, _>>()
        {
            Ok(seeds) => seeds,
            Err(_) => return Err(StringParseError),
        };

        let processes = match processes_string
            .split("\n\n")
            .map(|process_string| {
                let process_ranges = match process_string.split_once(":\n") {
                    Some((_process_label, process_range_strings)) => process_range_strings,
                    None => return Err(StringParseError),
                }
                .split('\n')
                .map(|range_string| {
                    let mut range_iter = range_string.splitn(3, ' ');
                    let to = match match range_iter.next() {
                        Some(to) => to,
                        None => return Err(StringParseError),
                    }
                    .parse()
                    {
                        Ok(to) => to,
                        Err(_) => return Err(StringParseError),
                    };
                    let from = match match range_iter.next() {
                        Some(to) => to,
                        None => return Err(StringParseError),
                    }
                    .parse()
                    {
                        Ok(to) => to,
                        Err(_) => return Err(StringParseError),
                    };
                    let range = match match range_iter.next() {
                        Some(to) => to,
                        None => return Err(StringParseError),
                    }
                    .parse()
                    {
                        Ok(to) => to,
                        Err(_) => return Err(StringParseError),
                    };
                    return Ok((to, from, range));
                })
                .collect::<Result<Vec<(usize, usize, usize)>, _>>();
                return process_ranges;
            })
            .collect::<Result<Vec<Vec<(usize, usize, usize)>>, _>>()
        {
            Ok(processes) => processes,
            Err(_) => return Err(StringParseError),
        };

        return Ok(Object {
            seeds: seeds,
            processes: processes,
        });
    }
}
pub fn solve(data: &String) -> String {
    let object = match data.parse::<Object>() {
        Ok(object) => object,
        Err(_) => panic!("Couldn't generate object!"),
    };

    let mut locations: Vec<usize> = Vec::new();
    object.seeds.into_iter().map(|seed| {
        let new_seed = seed.clone();

        object.
    });
    object.seeds.into_iter().for_each(|seed| {
        locations.push(object.processes.into_iter().fold(
            seed,
            |current_location, step: (&str, &str)| {
                return processes
                    .get(&step)
                    .unwrap()
                    .into_iter()
                    .fold_while(current_location, |value, (to, from, range)| {
                        println!(
                            "seed: {}, step: {:?}, start: {}",
                            seed, step, current_location,
                        );
                        return match resolve_step(&current_location, &to, &from, &range) {
                            Some(target) => Done(target),
                            None => Continue(value),
                        };
                    })
                    .into_inner();
            },
        ));
    });

    let smallest = locations
        .into_iter()
        .fold(usize::MAX, |smallest, seed| seed.min(smallest));
    return smallest.to_string();
}

fn resolve_step(start: &usize, to: &usize, from: &usize, range: &usize) -> Option<usize> {
    if from <= start && start < &(from + range) {
        return Some(to + (start - from));
    }
    return None;
}
