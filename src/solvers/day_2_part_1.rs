use std::collections::HashMap;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

#[derive(Debug)]
struct Game {
    id: i32,
    cube_reveals: Vec<HashMap<String, i32>>,
}
impl Game {
    pub fn is_valid(&self) -> bool {
        return self
            .cube_reveals
            .clone()
            .into_iter()
            .fold_while(true, |is_reveal_valid, cube_reveal| {
                let is_counts_valid = cube_reveal
                    .into_iter()
                    .fold_while(is_reveal_valid, |_is_counts_valid, (key, cube_count)| {
                        let limit = match key.as_str() {
                            "red" => 12,
                            "green" => 13,
                            "blue" => 14,
                            _ => panic!("Invalid color!"),
                        };
                        return if cube_count <= limit {
                            Continue(true)
                        } else {
                            Done(false)
                        };
                    })
                    .into_inner();
                return if is_counts_valid {
                    Continue(is_counts_valid)
                } else {
                    Done(is_counts_valid)
                };
            })
            .into_inner();
    }
}

struct GameParseError;
impl std::str::FromStr for Game {
    type Err = GameParseError;

    fn from_str(string: &str) -> Result<Game, GameParseError> {
        let (game_name, cube_set_strings) = match string.split_once(": ") {
            Some((game_name, cube_set_strings)) => (
                match game_name.replace("Game ", "").parse::<i32>() {
                    Ok(id) => id,
                    Err(_error) => panic!("Couldn't parse game id."),
                },
                cube_set_strings.split("; "),
            ),
            None => return Err(GameParseError),
        };

        /*
            Parse: "red: 1, green 2, blue 3; red 4, green 5, blue 6"
            Into: [
                {"red": 1, "green": 2, "blue": 3},
                {"red": 4, "green": 5, "blue": 6},
            ]
        */
        let mut cube_reveals: Vec<HashMap<String, i32>> = Vec::new();
        cube_set_strings.for_each(|cube_set_string| {
            let mut cube_set: HashMap<String, i32> = HashMap::new();
            cube_set_string.split(", ").for_each(|cube_set_string| {
                match cube_set_string.split_once(" ") {
                    Some((count, label)) => cube_set.insert(
                        label.to_string(),
                        match count.parse() {
                            Ok(count) => count,
                            _ => panic!("Can't parse set count to i32"),
                        },
                    ),
                    _ => panic!("Can't split cube map: Game"),
                };
            });
            cube_reveals.push(cube_set);
        });

        return Ok(Game {
            id: game_name,
            cube_reveals: cube_reveals,
        });
    }
}

pub fn solve(data: &String) -> String {
    let lines = data.split("\n");

    let games = lines.map(|line| {
        return match line.parse::<Game>() {
            Ok(game) => game,
            Err(_error) => panic!("Couldn't parse game."),
        };
    });

    let valid_games = games.filter(|game| {
        return game.is_valid();
    });
    return valid_games
        .fold(0, |sum, game| {
            return sum + game.id;
        })
        .to_string();
}
