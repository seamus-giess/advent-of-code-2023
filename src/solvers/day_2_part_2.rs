use std::collections::HashMap;

#[allow(dead_code)]
struct Game {
    id: i32,
    cube_reveals: Vec<HashMap<String, i32>>,
}
impl Game {
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        return self
            .cube_reveals
            .clone()
            .into_iter()
            .fold(true, |is_reveal_valid, cube_reveal| {
                return cube_reveal.into_iter().fold(
                    is_reveal_valid,
                    |is_counts_valid, (key, cube_count)| {
                        let limit = match key.as_str() {
                            "red" => 12,
                            "green" => 13,
                            "blue" => 14,
                            _ => panic!("Invalid color!"),
                        };
                        return is_counts_valid && (cube_count <= limit);
                    },
                );
            });
    }

    pub fn min_cubes_required(&self) -> HashMap<String, i32> {
        return self.cube_reveals.clone().into_iter().fold(
            HashMap::new(),
            |minimum_cubes: HashMap<String, i32>, cube_count| {
                let mut new_minimums = minimum_cubes.clone();
                cube_count.into_iter().for_each(|(colour, count)| {
                    let current_minimum = match minimum_cubes.get(&colour) {
                        Some(minimum) => minimum.clone(),
                        None => 0,
                    };
                    if count > current_minimum {
                        new_minimums.insert(colour, count);
                    }
                });
                return new_minimums;
            },
        );
    }

    pub fn get_power(&self) -> i32 {
        return self
            .min_cubes_required()
            .into_iter()
            .fold(1, |power, (_colour, min_required)| {
                return power * min_required;
            });
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
                            _ => panic!("Can't parse set count to i32."),
                        },
                    ),
                    None => panic!("Can't split cube map."),
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

    return games
        .fold(0, |sum, game| {
            return sum + game.get_power();
        })
        .to_string();
}
