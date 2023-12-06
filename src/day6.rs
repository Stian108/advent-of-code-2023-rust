use crate::*;
use parse_display::FromStr;

#[derive(FromStr, Debug)]
#[display("Time: {time}\nDistance: {distance}")]
pub struct Input {
    time: VecP<usize, " ">,
    distance: VecP<usize, " ">,
}

pub fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(inp: &Input) -> usize {
    inp.distance
        .0
        .iter()
        .zip(inp.time.0.iter())
        .map(|(distance, time)| {
            (1..=*time)
                .filter(|hold| hold * (time - hold) > *distance)
                .count()
        })
        .product()
}

// i know i could take the distance of roots for f(h) = -h^2 + th - d,
// but this is fast enough already, solving in a little over a second.
pub fn part2(inp: &Input) -> usize {
    let distance: usize = inp
        .distance
        .0
        .iter()
        .fold(String::new(), |s, distance| s + &distance.to_string())
        .parse()
        .unwrap();
    let time: usize = inp
        .time
        .0
        .iter()
        .fold(String::new(), |s, time| s + &time.to_string())
        .parse()
        .unwrap();
    (1..=time)
        .filter(|hold| hold * (time - hold) > distance)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 288)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 71503)
    }
}
