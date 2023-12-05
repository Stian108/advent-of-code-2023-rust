use ndarray::Array2;
use std::collections::{HashMap, HashSet};

type Input = Array2<char>;

pub fn parse_input(input: &str) -> Input {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    let values: Vec<char> = lines.iter().flat_map(|line| line.chars()).collect();

    Array2::from_shape_vec((height, width), values).unwrap()
}

pub fn part1(inp: &Input) -> usize {
    let mut sum = 0;
    let mut num = 0;
    let mut include = false;
    let mut row = 0;
    for ((i, j), c) in inp.indexed_iter() {
        if i > row {
            row = i;
            if include {
                sum += num;
            }
            num = 0;
            include = false;
        }
        if let Some(d) = c.to_digit(10) {
            num *= 10;
            num += d;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if let (Some(ni), Some(nj)) =
                        (i.checked_add_signed(di), j.checked_add_signed(dj))
                    {
                        if let Some(&x) = inp.get((ni, nj)) {
                            if !x.is_digit(10) && x != '.' {
                                include = true;
                            }
                        }
                    }
                }
            }
        } else {
            if include {
                sum += num;
            }
            num = 0;
            include = false;
        }
    }
    sum as usize
}

pub fn part2(inp: &Input) -> usize {
    let mut num = 0;
    let mut include = HashSet::new();
    let mut row = 0;
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for ((i, j), c) in inp.indexed_iter() {
        if i > row {
            row = i;
            for x in include.iter() {
                let nums = gears.entry(*x).or_insert(vec![]);
                (*nums).push(num);
            }
            num = 0;
            include = HashSet::new();
        }
        if let Some(d) = c.to_digit(10) {
            num *= 10;
            num += d as usize;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if let (Some(ni), Some(nj)) =
                        (i.checked_add_signed(di), j.checked_add_signed(dj))
                    {
                        if let Some(&x) = inp.get((ni, nj)) {
                            if x == '*' {
                                include.insert((ni, nj));
                            }
                        }
                    }
                }
            }
        } else {
            for x in include.iter() {
                let nums = gears.entry(*x).or_insert(vec![]);
                (*nums).push(num);
            }
            num = 0;
            include = HashSet::new();
        }
    }
    gears
        .values()
        .map(|v| if v.len() == 2 { v[0] * v[1] } else { 0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 4361)
    }

    #[test]
    fn part1_example2() {
        assert_eq!(
            part1(&parse_input(
                ".11
22*
"
            )),
            33
        )
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 467835)
    }
}
