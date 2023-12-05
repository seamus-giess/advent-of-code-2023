use std::collections::HashMap;
use std::f32::INFINITY;
use std::usize;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn solve(data: &String) -> String {
    let (whole_seeds_string, sections_string) = data.split_once("\n\n").unwrap();
    let (_label, seeds_string) = whole_seeds_string.split_once(": ").unwrap();
    let seeds: Vec<usize> = seeds_string
        .split(' ')
        .into_iter()
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect();
    let sections = sections_string.split("\n\n").collect_vec();

    let maps: HashMap<(&str, &str), Vec<(usize, usize, usize)>> =
        sections.into_iter().fold(HashMap::new(), |map, section| {
            let mut new_map = map.clone();

            let (mut label_string, range_strings) = section.split_once(":\n").unwrap();

            (label_string, _) = label_string.split_once(" map").unwrap();
            let (from, to) = label_string.split_once("-to-").unwrap();

            let ranges: Vec<(usize, usize, usize)> =
                range_strings
                    .split('\n')
                    .into_iter()
                    .fold(Vec::new(), |vec, range_string| {
                        let mut new_vec = vec.clone();
                        let mut iter = range_string.splitn(3, ' ');
                        let to = iter.next().unwrap().parse().unwrap();
                        let from = iter.next().unwrap().parse().unwrap();
                        let range = iter.next().unwrap().parse().unwrap();
                        new_vec.push((to, from, range));
                        return new_vec;
                    });

            new_map.insert((from, to), ranges);
            return new_map;
        });

    let process = Vec::from([
        ("seed", "soil"),
        ("soil", "fertilizer"),
        ("fertilizer", "water"),
        ("water", "light"),
        ("light", "temperature"),
        ("temperature", "humidity"),
        ("humidity", "location"),
    ]);

    let mut locations: Vec<usize> = Vec::new();
    seeds.into_iter().for_each(|seed| {
        locations.push(process.clone().into_iter().fold(
            seed,
            |current_location, step: (&str, &str)| {
                return maps
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

    println!("{:?}", locations);
    let smallest = locations.into_iter().fold(usize::MAX, |smallest, seed| {
        if seed < smallest {
            return seed;
        } else {
            return smallest;
        }
    });
    return smallest.to_string();
}

fn resolve_step(start: &usize, to: &usize, from: &usize, range: &usize) -> Option<usize> {
    if from <= start && start < &(from + range) {
        return Some(to + (start - from));
    }
    return None;
}
