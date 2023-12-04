use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

struct Grid {
    rows: Vec<Vec<char>>,
}
impl Grid {
    pub fn get_adjacent_numbers(&self, x: usize, y: usize) -> i32 {
        let adjacent_coordinates = Vec::from([
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]);

        let adjacent_parts: Vec<i32> = adjacent_coordinates
            .into_iter()
            .map(|(adj_x, adj_y)| {
                return self.get_full_number(adj_x, adj_y);
            })
            .unique()
            .filter(|value| return value.clone() != (0 as i32))
            .collect();

        if adjacent_parts.len() != 2 {
            return 0;
        }

        return adjacent_parts.into_iter().fold(1, |acc, part_number| {
            return acc * part_number;
        });
    }
    fn get_full_number(&self, x: usize, y: usize) -> i32 {
        let mut number_string = match self.rows[y][x] {
            '.' => return 0,
            char => char.to_string(),
        };

        let mut right_shift: usize = 1;
        loop {
            match self.rows[y].get(x + right_shift) {
                None | Some('.') => break, // Effectively "break;"
                Some(symbol) => {
                    match DIGITS.contains(symbol) {
                        true => {
                            number_string.push(symbol.clone());
                            right_shift += 1;
                        }
                        _ => break,
                    };
                }
            };
        }

        let mut left_shift: usize = 1;
        loop {
            match self.rows[y].get(x - left_shift) {
                None | Some('.') => break,
                Some(symbol) => match DIGITS.contains(symbol) {
                    true => {
                        number_string.insert(0, symbol.clone());
                        left_shift += 1;
                    }
                    _ => break,
                },
            };
        }
        return number_string.parse().unwrap();
    }
}
struct GridParseError;
impl FromStr for Grid {
    type Err = GridParseError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let lines = data.split("\n");

        let grid: Vec<Vec<char>> = lines
            .map(|line: &str| {
                return line.chars().collect_vec();
            })
            .collect::<Vec<Vec<char>>>();

        return Ok(Grid { rows: grid });
    }
}
pub fn solve(data: &String) -> String {
    let grid = match data.clone().parse::<Grid>() {
        Ok(grid) => grid,
        _ => panic!("Failed to make the grid!"),
    };

    let symbols: Regex = Regex::new(r"[*]").unwrap();

    let line_length = data.find('\n').unwrap();

    let mut sum = 0;
    symbols
        .find_iter(data.clone().replace('\n', "").as_str())
        .for_each(|symbol| {
            let x = symbol.start() % line_length;
            let y = (symbol.start()) / line_length;
            sum += grid.get_adjacent_numbers(x, y);
        });

    return sum.to_string();
}
