use crate::*;
use itertools::Itertools;
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Eq, Debug, PartialOrd, Ord, Hash, Clone)]
pub enum Card {
    Joker,
    #[display("2")]
    Two,
    #[display("3")]
    Three,
    #[display("4")]
    Four,
    #[display("5")]
    Five,
    #[display("6")]
    Six,
    #[display("7")]
    Seven,
    #[display("8")]
    Eight,
    #[display("9")]
    Nine,
    #[display("T")]
    Ten,
    #[display("J")]
    Jack,
    #[display("Q")]
    Queen,
    #[display("K")]
    King,
    #[display("A")]
    Ace,
}

#[derive(FromStr, Debug, Clone)]
#[display("{cards} {bid}")]
pub struct Hand {
    cards: VecP<Card, "">,
    bid: usize,
}

type Input = VecP<Hand>;

pub fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum Strength {
    High,
    OnePair,
    TwoPair,
    ThreeKind,
    House,
    FourKind,
    FiveKind,
}

fn strength(cards: &[Card]) -> Strength {
    let mut counts = cards.iter().counts();
    let jokers = counts.remove(&Card::Joker).unwrap_or(0);
    let mut counts = counts.values().sorted().rev();
    match counts.next().unwrap_or(&0) + jokers {
        5 => Strength::FiveKind,
        4 => Strength::FourKind,
        3 => match counts.next() {
            Some(2) => Strength::House,
            _ => Strength::ThreeKind,
        },
        2 => match counts.next() {
            Some(2) => Strength::TwoPair,
            _ => Strength::OnePair,
        },
        _ => Strength::High,
    }
}

pub fn part1(inp: &Input) -> usize {
    inp.0
        .iter()
        .sorted_by(|a, b| {
            strength(&a.cards.0)
                .cmp(&strength(&b.cards.0))
                .then(a.cards.0.cmp(&b.cards.0))
        })
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

pub fn part2(inp: &Input) -> usize {
    let mut inp = inp.0.clone();
    inp.iter_mut()
        .flat_map(|hand| hand.cards.0.iter_mut())
        .for_each(|card| {
            if matches!(card, Card::Jack) {
                *card = Card::Joker;
            }
        });
    inp.iter()
        .sorted_by(|a, b| {
            strength(&a.cards.0)
                .cmp(&strength(&b.cards.0))
                .then(a.cards.0.cmp(&b.cards.0))
        })
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 6440)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 5905)
    }
}
