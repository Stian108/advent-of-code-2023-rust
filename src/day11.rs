use itertools::Itertools;
use ndarray::Array2;
use std::collections::HashSet;

type Input = Array2<char>;

pub fn parse_input(input: &str) -> Input {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    let values: Vec<char> = lines.iter().flat_map(|line| line.chars()).collect();

    Array2::from_shape_vec((height, width), values).unwrap()
}

fn solve(inp: &Input, gap_size: usize) -> usize {
    let mut row_gaps = HashSet::new();
    for i in 0..inp.nrows() {
        if inp.row(i).iter().all(|&c| c == '.') {
            row_gaps.insert(i);
        }
    }
    let mut col_gaps = HashSet::new();
    for j in 0..inp.ncols() {
        if inp.column(j).iter().all(|&c| c == '.') {
            col_gaps.insert(j);
        }
    }
    let mut points = Vec::new();
    let mut row_gap_sum = 0;
    for i in 0..inp.nrows() {
        if row_gaps.contains(&i) {
            row_gap_sum += 1;
        } else {
            let mut col_gap_sum = 0;
            for j in 0..inp.ncols() {
                if col_gaps.contains(&j) {
                    col_gap_sum += 1;
                } else if inp[(i, j)] == '#' {
                    points.push((
                        i + row_gap_sum * (gap_size - 1),
                        j + col_gap_sum * (gap_size - 1),
                    ))
                }
            }
        }
    }
    points
        .iter()
        .combinations(2)
        .map(|pair| pair[0].0.abs_diff(pair[1].0) + pair[0].1.abs_diff(pair[1].1))
        .sum()
}

pub fn part1(inp: &Input) -> usize {
    solve(inp, 2)
}

pub fn part2(inp: &Input) -> usize {
    solve(inp, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 374)
    }

    #[test]
    fn part2_example1() {
        assert_eq!(solve(&parse_input(EXAMPLE), 10), 1030)
    }

    #[test]
    fn part2_example2() {
        assert_eq!(solve(&parse_input(EXAMPLE), 100), 8410)
    }
}
