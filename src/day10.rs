use std::collections::{HashMap, HashSet};

type Input = (HashMap<(isize, isize), Vec<(isize, isize)>>, (isize, isize));

pub fn parse_input(input: &str) -> Input {
    let mut start = (0, 0);
    (
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as isize, y as isize), c))
            })
            .filter_map(|((x, y), c)| match c {
                '|' => Some(((x, y), vec![(x, y + 1), (x, y - 1)])),
                '-' => Some(((x, y), vec![(x + 1, y), (x - 1, y)])),
                'L' => Some(((x, y), vec![(x, y - 1), (x + 1, y)])),
                'J' => Some(((x, y), vec![(x, y - 1), (x - 1, y)])),
                '7' => Some(((x, y), vec![(x, y + 1), (x - 1, y)])),
                'F' => Some(((x, y), vec![(x, y + 1), (x + 1, y)])),
                '.' => None,
                'S' => {
                    start = (x, y);
                    Some(((x, y), vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]))
                }
                _ => unreachable!(),
            })
            .collect(),
        start,
    )
}

pub fn part1(inp: &Input) -> usize {
    let graph = inp.0.clone();
    let source = inp.1;
    let mut visited = HashSet::new();
    let mut q = Vec::new();
    q.push((source, vec![]));
    let mut loops = vec![];
    while !q.is_empty() {
        let (this, lasts) = q.pop().unwrap();
        visited.insert(this);
        let mut next_lasts = lasts.clone();
        next_lasts.push(this);
        for next in graph.get(&this).unwrap_or(&Vec::new()) {
            if graph.contains_key(next) && (lasts.is_empty() || next != lasts.last().unwrap()) {
                if *next == source {
                    loops.push(next_lasts.clone());
                } else if !visited.contains(next) {
                    q.push((*next, next_lasts.clone()))
                }
            }
        }
    }
    loops[0].len() / 2
}

pub fn part2(_inp: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE1: &str = ".....
.S-7.
.|.|.
.L-J.
.....
";

    const EXAMPLE2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse_input(EXAMPLE1)), 4)
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(&parse_input(EXAMPLE2)), 8)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE1)), 0)
    }
}
