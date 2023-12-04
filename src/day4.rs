use crate::*;
use parse_display::FromStr;
use std::collections::HashSet;

#[derive(FromStr, Debug)]
#[from_str(regex = r"Card +\d+: (?<winning>[ \d]+) \| (?<have>[ \d]+)")]
pub struct Card {
    winning: VecP<usize, " ">,
    have: VecP<usize, " ">,
}

type Input = VecP<Card>;

pub fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(inp: &Input) -> usize {
    inp.0
        .iter()
        .map(|Card { winning, have }| {
            (1 << winning
                .0
                .iter()
                .collect::<HashSet<_>>()
                .intersection(&have.0.iter().collect::<HashSet<_>>())
                .count())
                >> 1
        })
        .sum()
}

pub fn part2(inp: &Input) -> usize {
    let mut cards = vec![1; inp.0.len()];
    for i in 0..cards.len() {
        let Card { winning, have } = &inp.0[i];
        let card_count = cards[i];
        let win_count = winning
            .0
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&have.0.iter().collect::<HashSet<_>>())
            .count();
        if let Some(new_cards) = cards.get_mut(i + 1..i + win_count + 1) {
            new_cards.iter_mut().for_each(|e| *e += card_count);
        }
    }
    cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 13)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 30)
    }
}
