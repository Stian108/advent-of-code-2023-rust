use crate::*;
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Eq, Debug)]
pub enum Cube {
    #[display("red")]
    Red,
    #[display("green")]
    Green,
    #[display("blue")]
    Blue,
}

#[derive(FromStr, Debug)]
#[display("{count} {cube}")]
pub struct Draw {
    count: usize,
    cube: Cube,
}

#[derive(FromStr, Debug)]
#[display("Game {id}: {rounds}")]
pub struct Game {
    id: usize,
    rounds: VecP<VecP<Draw, ",">, ";">,
}

type Input = VecP<Game>;

pub fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(inp: &Input) -> usize {
    inp.0
        .iter()
        .filter_map(|Game { id, rounds }| {
            if !rounds.0.iter().any(|draws| {
                draws.0.iter().any(|Draw { count, cube }| match cube {
                    Cube::Red => *count > 12,
                    Cube::Green => *count > 13,
                    Cube::Blue => *count > 14,
                })
            }) {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(inp: &Input) -> usize {
    inp.0
        .iter()
        .map(|Game { id: _, rounds }| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for Draw { count, cube } in rounds.0.iter().flat_map(|draws| draws.0.iter()) {
                match cube {
                    Cube::Red if *count > red => red = *count,
                    Cube::Green if *count > green => green = *count,
                    Cube::Blue if *count > blue => blue = *count,
                    _ => {}
                }
            }
            red * green * blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 8)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 2286)
    }
}
