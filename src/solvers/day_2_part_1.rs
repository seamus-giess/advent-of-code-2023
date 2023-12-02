use std::collections::HashMap;

pub fn solve(data: &String) -> String {
    let exploded = data.split("\n");

    let cube_limits = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let mut possible_games: HashMap<
        &str,
        Vec<HashMap<&str, i32>>,
    > = HashMap::new();
    'lines: for line in exploded.into_iter() {
        let mut line_fragments = line.split(": ");
        let game_name = line_fragments.next().unwrap();
        let reveal_strings =
            line_fragments.next().unwrap().split("; ");
        let mut game_cube_counts: Vec<HashMap<&str, i32>> =
            Vec::new();
        for reveal_string in reveal_strings.into_iter() {
            let cube_reveals = reveal_string.split(", ");
            let mut cube_count = HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0),
            ]);
            for reveal in cube_reveals.into_iter() {
                let mut split_cube_count =
                    reveal.split(" ").into_iter();
                let value = split_cube_count
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                let key = split_cube_count.last().unwrap();

                if value
                    > cube_limits.get(key).unwrap().clone()
                {
                    continue 'lines;
                }

                cube_count.insert(key, value);
            }
            game_cube_counts.push(cube_count);
        }
        possible_games.insert(game_name, game_cube_counts);
    }

    let mut sum: i32 = 0;
    for (game_label, _) in possible_games.into_iter() {
        let game_id: i32 = game_label
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        sum += game_id;
    }

    println!("{:?}", sum);

    return String::from("0"); //sum.to_string();
}
