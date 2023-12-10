use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

struct Object {
    instructions: Vec<char>,
    steps: HashMap<String, (String, String)>,
}
impl Object {}

struct StringParseError;
impl FromStr for Object {
    type Err = StringParseError;
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let (instructions_string, coordinates) = data.split_once("\n\n").unwrap();

        let instructions = instructions_string.chars().collect_vec();

        let mut steps = HashMap::new();
        coordinates
            .to_string()
            .replace('(', "")
            .replace(')', "")
            .split('\n')
            .into_iter()
            .map(|x: &str| x.split_once(" = ").unwrap())
            .map(|(label, pair_string)| (label, pair_string.split_once(", ").unwrap()))
            .for_each(|(label, (left, right))| {
                steps.insert(label.to_string(), (left.to_string(), right.to_string()));
            });

        return Ok(Object {
            instructions: instructions,
            steps: steps,
        });
    }
}

pub fn solve(data: &String) -> String {
    let _object = match data.parse::<Object>() {
        Ok(object) => object,
        Err(_) => panic!("Could not parse object"),
    };

    return "sum".to_string();
}
