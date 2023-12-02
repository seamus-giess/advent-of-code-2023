use std::collections::HashMap;

pub fn solve(data: &String) -> String {
    let exploded = data.split("\n");

    let mut game_minimums: HashMap<
        &str,
        HashMap<&str, i32>,
    > = HashMap::new();
    for line in exploded.into_iter() {
        let mut line_fragments = line.split(": ");
        let game_name = line_fragments.next().unwrap();
        let reveal_strings =
            line_fragments.next().unwrap().split("; ");
        let mut minimum_cubes: HashMap<&str, i32> =
            HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0),
            ]);
        for reveal_string in reveal_strings.into_iter() {
            let cube_reveals = reveal_string.split(", ");

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
                    > minimum_cubes
                        .get(key)
                        .unwrap()
                        .clone()
                {
                    minimum_cubes.insert(key, value);
                }
            }
        }

        game_minimums.insert(game_name, minimum_cubes);
    }

    let mut sum: i32 = 0;
    for (game, minimums) in game_minimums.into_iter() {
        let mut game_power: i32 = 1;
        for (color, minimum) in minimums.into_iter() {
            game_power = game_power * minimum;
        }
        sum += game_power;
    }

    println!("{:?}", sum);

    return String::from("0"); //sum.to_string();
}
