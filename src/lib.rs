#![allow(incomplete_features)]
#![feature(adt_const_params)]

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use std::{fmt::Debug, str::FromStr};

extern crate derive_more;
use derive_more::From;

#[derive(Debug, Clone, From)]
pub struct VecP<T, const P: &'static str = "\n">(Vec<T>);

impl<T: std::str::FromStr, const P: &'static str> FromStr for VecP<T, P> {
    type Err = T::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(P)
            .filter_map(|v| {
                let trimmed = v.trim();
                (!trimmed.is_empty()).then_some(trimmed.parse())
            })
            .collect::<Result<_, Self::Err>>()
            .map(Self)
    }
}
