use crate::*;

type Input = VecP<VecP<isize, " ">>;

pub fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(inp: &Input) -> isize {
    inp.0
        .iter()
        .map(|seq| {
            let mut curr = seq.0.clone();
            let mut sum = 0;
            while !curr.iter().all(|&e| e == 0) {
                sum += curr.last().unwrap();
                let mut next = vec![];
                for i in 1..curr.len() {
                    next.push(curr[i] - curr[i - 1])
                }
                curr = next;
            }
            sum
        })
        .sum()
}

pub fn part2(inp: &Input) -> isize {
    inp.0
        .iter()
        .map(|seq| {
            let mut curr = seq.0.clone();
            let mut sum = 0;
            let mut alt = -1;
            while !curr.iter().all(|&e| e == 0) {
                alt *= -1;
                sum += curr.first().unwrap() * alt;
                let mut next = vec![];
                for i in 1..curr.len() {
                    next.push(curr[i] - curr[i - 1])
                }
                curr = next;
            }
            sum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 114)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 2)
    }
}
