use std::collections::HashMap;
use std::usize;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use regex::Regex;

pub fn solve(data: &String) -> String {
    let reg_ex = Regex::new(r"\d+\s\d+").unwrap();
    let (whole_seeds_string, sections_string) = data.split_once("\n\n").unwrap();
    let (_label, seeds_string) = whole_seeds_string.split_once(": ").unwrap();
    let mut seeds: Vec<usize> = Vec::new();
    reg_ex.find_iter(seeds_string).for_each(|seed| {
        let (start, range) = seed.as_str().split_once(' ').unwrap();
        (start.parse().unwrap()
            ..(start.parse::<usize>().unwrap() + range.parse::<usize>().unwrap()))
            .for_each(|x| seeds.push(x));
    });
    seeds.dedup();
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
                        return match resolve_step(&current_location, &to, &from, &range) {
                            Some(target) => Done(target),
                            None => Continue(value),
                        };
                    })
                    .into_inner();
            },
        ));
    });

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
