use crate::*;
use itertools::Itertools;
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Eq, Debug)]
pub enum Category {
    #[display("seed")]
    Seed,
    #[display("soil")]
    Soil,
    #[display("fertilizer")]
    Fertilizer,
    #[display("water")]
    Water,
    #[display("light")]
    Light,
    #[display("temperature")]
    Temperature,
    #[display("humidity")]
    Humidity,
    #[display("location")]
    Location,
}

#[derive(FromStr, Debug)]
#[display("{dest_start} {source_start} {length}")]
pub struct Range {
    dest_start: usize,
    source_start: usize,
    length: usize,
}

#[derive(FromStr, Debug)]
#[display("{_from}-to-{_to} map:\n{mappings}")]
pub struct Map {
    _from: Category,
    _to: Category,
    mappings: VecP<Range>,
}
#[derive(FromStr, Debug)]
#[display("seeds: {seeds}\n\n{maps}")]
pub struct Input {
    seeds: VecP<usize, " ">,
    maps: VecP<Map, "\n\n">,
}

pub fn parse_input(input: &str) -> Input {
    input.parse().unwrap()
}

pub fn part1(inp: &Input) -> usize {
    let mut values = inp.seeds.0.clone();
    for map in inp.maps.0.iter() {
        for val in values.iter_mut() {
            for Range {
                dest_start,
                source_start,
                length,
            } in map.mappings.0.iter()
            {
                if *val >= *source_start && *val < source_start + length {
                    *val += dest_start;
                    *val -= source_start;
                    break;
                }
            }
        }
    }
    *values.iter().min().unwrap()
}

pub fn part2(inp: &Input) -> usize {
    let mut ranges: Vec<_> = inp.seeds.0.iter().cloned().tuples().collect();
    for map in inp.maps.0.iter() {
        let mut new_ranges: Vec<(usize, usize)> = vec![];
        while !ranges.is_empty() {
            let (start, len) = ranges.pop().unwrap();
            let mut added = false;
            for Range {
                dest_start,
                source_start,
                length,
            } in map.mappings.0.iter()
            {
                if start >= *source_start && start < source_start + length {
                    let new_start = start + dest_start - source_start;
                    if start + len < source_start + length {
                        new_ranges.push((new_start, len));
                    } else {
                        new_ranges.push((new_start, dest_start + length - new_start));
                        ranges.push((
                            source_start + length,
                            &new_start + len - (dest_start + length),
                        ));
                    }
                    added = true;
                    break;
                } else if start + len >= *source_start && start + len < source_start + length {
                    new_ranges.push((*dest_start, len + start + 1 - source_start));
                    ranges.push((start, source_start - start - 1));
                    added = true;
                    break;
                }
            }
            if !added {
                new_ranges.push((start, len))
            }
        }
        ranges = new_ranges;
    }
    *ranges.iter().map(|(start, _)| start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 35)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(EXAMPLE)), 46)
    }
}
