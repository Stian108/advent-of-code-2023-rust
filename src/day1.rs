use itertools::Itertools;

fn solve(inp: &str, filter: &[&str]) -> usize {
    inp.lines()
        .map(|line| {
            let ds: Vec<_> = filter
                .iter()
                .flat_map(|p| line.match_indices(p))
                .sorted_by_key(|m| m.0)
                .map(|(_, m)| match m {
                    "1" | "one" => 1,
                    "2" | "two" => 2,
                    "3" | "three" => 3,
                    "4" | "four" => 4,
                    "5" | "five" => 5,
                    "6" | "six" => 6,
                    "7" | "seven" => 7,
                    "8" | "eight" => 8,
                    "9" | "nine" => 9,
                    _ => unreachable!(),
                })
                .collect();
            ds.first().unwrap() * 10 + ds.last().unwrap()
        })
        .sum()
}

pub fn part1(inp: &str) -> usize {
    solve(inp, &["1", "2", "3", "4", "5", "6", "7", "8", "9"])
}

pub fn part2(inp: &str) -> usize {
    solve(
        inp,
        &[
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3",
            "4", "5", "6", "7", "8", "9",
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
    const EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 142)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE2), 281)
    }
}
