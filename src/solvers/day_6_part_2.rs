use std::str::FromStr;

struct Object {
    races: Vec<(usize, usize)>,
}
impl Object {}

struct StringParseError;
impl FromStr for Object {
    type Err = StringParseError;
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let (mut times, mut distances) = match data.split_once('\n') {
            Some((times_string, distances_string)) => (
                times_string.split_whitespace().into_iter(),
                distances_string.split_whitespace().into_iter(),
            ),
            None => return Err(StringParseError),
        };

        // pop the first of either as they're labels
        let _times_label = times.next();
        let _distances_label = distances.next();

        let mut races: Vec<(usize, usize)> = Vec::new();
        loop {
            races.push((
                match times.next() {
                    Some(time) => time,
                    None => break,
                }
                .parse()
                .unwrap(),
                match distances.next() {
                    Some(distance) => distance,
                    None => break,
                }
                .parse()
                .unwrap(),
            ));
        }

        return Ok(Object { races: races });
    }
}

pub fn solve(data: &String) -> String {
    let object = match data.parse::<Object>() {
        Ok(object) => object,
        Err(_) => panic!("Couldn't parse object"),
    };

    let score = object.races.into_iter().fold(1, |acc, (time, distance)| {
        return acc * find_num_ways_to_win(time, distance);
    });
    return score.to_string();
}

fn find_num_ways_to_win(time: usize, distance: usize) -> usize {
    let ways_to_win = (0..time)
        .map(|this_time| {
            let charge_time = time - this_time;
            return charge_time * (time - charge_time);
        })
        .filter(|distance_covered| distance_covered > &distance);
    return ways_to_win.count();
}
