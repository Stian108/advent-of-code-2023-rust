use std::collections::HashMap;

use crate::*;
use num::Integer;
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Eq, Debug)]
pub enum Dir {
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

#[derive(FromStr, Debug)]
#[display("{src} = ({left}, {right})")]
pub struct Edge {
    src: String,
    left: String,
    right: String,
}
#[derive(FromStr, Debug)]
#[display("{moves}\n\n{edges}")]
pub struct Input {
    moves: VecP<Dir, "">,
    edges: VecP<Edge>,
}

pub fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(inp: &Input) -> usize {
    let g: HashMap<&str, (&str, &str)> = inp
        .edges
        .0
        .iter()
        .map(|Edge { src, left, right }| (src.as_str(), (left.as_str(), right.as_str())))
        .collect();
    let mut count = 0;
    let mut current = "AAA";
    for mov in inp.moves.0.iter().cycle() {
        if current == "ZZZ" {
            break;
        }
        count += 1;
        match mov {
            Dir::Left => current = g[current].0,
            Dir::Right => current = g[current].1,
        }
    }
    count
}

pub fn part2(inp: &Input) -> usize {
    let g: HashMap<&str, (&str, &str)> = inp
        .edges
        .0
        .iter()
        .map(|Edge { src, left, right }| (src.as_str(), (left.as_str(), right.as_str())))
        .collect();
    let mut count = 0;
    let mut currents: Vec<&str> = g.keys().cloned().filter(|k| k.ends_with('A')).collect();
    let mut counts: Vec<usize> = vec![];
    for mov in inp.moves.0.iter().cycle() {
        let mut new_currents = vec![];
        for current in currents[..].iter() {
            if current.ends_with('Z') {
                counts.push(count)
            } else {
                match mov {
                    Dir::Left => new_currents.push(g[current].0),
                    Dir::Right => new_currents.push(g[current].1),
                }
            }
        }
        if new_currents.is_empty() {
            break;
        }
        currents = new_currents;
        count += 1;
    }
    counts.iter().fold(1, |acc, count| acc.lcm(count))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const EXAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    const EXAMPLE3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 2)
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse_input(EXAMPLE2)), 6)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE3)), 6)
    }
}
